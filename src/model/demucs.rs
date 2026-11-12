//! Demucs v4 (HTDemucs) hybrid transformer, ported to Candle.
//!
//! This is the network half of the Demucs path: the spectrogram branch
//! (`encoder`/`decoder`), the waveform branch (`tencoder`/`tdecoder`), the
//! DConv residual blocks, the cross transformer, and the exact STFT / iSTFT
//! and normalisation constants of `demucs/htdemucs.py`.
//!
//! Weight file: the `safetensors` artifact produced by
//! `tools/convert_demucs.py` (see `models/955717e8-8726e21a.manifest.json` in
//! the item-4 evidence tree). The port never reads the original `.th`.
//!
//! The layout is *not* negotiable: every tensor name and shape is validated
//! against [`expected_key_shapes`], which mirrors the Python module tree one
//! for one. A missing, extra or mis-shaped tensor is a hard error, never a
//! silent skip.
//!
//! Frequency-axis convolutions are rectangular in the reference (`kernel
//! (8,1)`, `stride (4,1)`, `padding (2,0)`), while `candle_nn` only exposes
//! square 2-D kernels. Because the time axis has kernel 1 / stride 1 they are
//! exactly a 1-D convolution over the frequency axis applied to every time
//! step, which is how they are implemented here (see [`FreqConv`]).

use candle_core::{DType, Device, Module, Tensor, D};
use candle_nn::{layer_norm, Conv1d, Conv1dConfig, LayerNorm, LayerNormConfig, Linear, ModuleT};
use candle_nn::VarBuilder;

use crate::error::{ConfigError, Error};

/// Network source order (`HTDemucs.sources`).
pub const SOURCES: [&str; 4] = ["drums", "bass", "other", "vocals"];

const NFFT: usize = 4096;
const HOP: usize = 1024;
const LAYER_NORM_EPS: f64 = 1e-5;
const GROUP_NORM_EPS: f64 = 1e-5;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// The structural parameters of the pinned HTDemucs checkpoint.
///
/// Values are the `kwargs` stored in the `.th` package (see
/// `analysis/architecture.json`); they are fixed for this profile and exposed
/// as fields only so the loader can validate against the manifest.
#[derive(Debug, Clone)]
pub struct DemucsConfig {
    pub audio_channels: usize,
    pub samplerate: usize,
    pub segment_sec_num: usize,
    pub segment_sec_den: usize,
    pub channels: usize,
    pub growth: usize,
    pub depth: usize,
    pub kernel_size: usize,
    pub stride: usize,
    pub norm_starts: usize,
    pub norm_groups: usize,
    pub dconv_depth: usize,
    pub dconv_comp: usize,
    pub context: usize,
    pub context_enc: usize,
    pub bottom_channels: usize,
    pub t_layers: usize,
    pub t_heads: usize,
    pub t_hidden_scale: usize,
    pub t_max_period: f64,
    pub freq_emb_scale: f64,
    pub emb_scale: f64,
    pub rescale: f64,
}

impl Default for DemucsConfig {
    fn default() -> Self {
        Self {
            audio_channels: 2,
            samplerate: 44100,
            // segment = Fraction(39, 5) seconds -> int(samplerate * segment)
            segment_sec_num: 39,
            segment_sec_den: 5,
            channels: 48,
            growth: 2,
            depth: 4,
            kernel_size: 8,
            stride: 4,
            norm_starts: 4,
            norm_groups: 4,
            dconv_depth: 2,
            dconv_comp: 8,
            context: 1,
            context_enc: 0,
            bottom_channels: 512,
            t_layers: 5,
            t_heads: 8,
            t_hidden_scale: 4,
            t_max_period: 10000.0,
            freq_emb_scale: 0.2,
            emb_scale: 10.0,
            rescale: 0.1,
        }
    }
}

impl DemucsConfig {
    /// `int(samplerate * segment)`, i.e. 343980 samples (7.8 s at 44.1 kHz).
    pub fn segment_samples(&self) -> usize {
        // int(44100 * 39/5) = int(343980.0); integer arithmetic keeps it exact.
        self.samplerate * self.segment_sec_num / self.segment_sec_den
    }

    pub fn num_sources(&self) -> usize {
        SOURCES.len()
    }
}

// ---------------------------------------------------------------------------
// Small building blocks
// ---------------------------------------------------------------------------

/// GroupNorm with `groups` groups over the channel axis of `(batch, channels,
/// ...)` tensors. PyTorch normalises with the *biased* variance and eps 1e-5.
#[derive(Debug)]
struct GroupNorm {
    weight: Tensor,
    bias: Tensor,
    groups: usize,
}

impl GroupNorm {
    fn load(channels: usize, groups: usize, vb: VarBuilder) -> Result<Self, Error> {
        let weight = vb.get(channels, "weight").map_err(Error::Candle)?;
        let bias = vb.get(channels, "bias").map_err(Error::Candle)?;
        Ok(Self {
            weight,
            bias,
            groups,
        })
    }

    /// `x`: `(b, c, t)` or `(b, c, f, t)`.
    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let dims = x.dims().to_vec();
        if dims.len() < 3 {
            return Err(Error::Config(ConfigError::Invalid(
                "GroupNorm expects at least 3 dimensions".into(),
            )));
        }
        let (b, c) = (dims[0], dims[1]);
        let g = self.groups;
        let cg = c / g;
        let mut shape = vec![b, g, cg];
        shape.extend_from_slice(&dims[2..]);
        let xg = x.reshape(shape.clone()).map_err(Error::Candle)?;
        let n: usize = shape[2..].iter().product();
        let count = n as f64;
        let mut sum = xg.sum_keepdim(2).map_err(Error::Candle)?;
        for d in 3..shape.len() {
            sum = sum.sum_keepdim(d).map_err(Error::Candle)?;
        }
        let mean = (sum / count).map_err(Error::Candle)?;
        let mut diff = xg.broadcast_sub(&mean).map_err(Error::Candle)?;
        let mut sq = diff.sqr().map_err(Error::Candle)?;
        // The variance (like the mean above) is pooled over the *whole group*:
        // every channel of the group plus every trailing dimension. Reducing
        // only dims 3.. would divide each channel by its own temporal variance
        // instead, which is a different statistic whenever the channels of a
        // group carry different scales.
        sq = sq.sum_keepdim(2).map_err(Error::Candle)?;
        for d in 3..shape.len() {
            sq = sq.sum_keepdim(d).map_err(Error::Candle)?;
        }
        let var = (sq / count).map_err(Error::Candle)?;
        let denom = (&var + GROUP_NORM_EPS).map_err(Error::Candle)?.sqrt().map_err(Error::Candle)?;
        diff = diff.broadcast_div(&denom).map_err(Error::Candle)?;
        let y = diff.reshape(dims.clone()).map_err(Error::Candle)?;
        let mut wshape = vec![1, c];
        wshape.resize(dims.len(), 1);
        let w = self.weight.reshape(wshape.clone()).map_err(Error::Candle)?;
        let bias = self.bias.reshape(wshape).map_err(Error::Candle)?;
        let y = y.broadcast_mul(&w).map_err(Error::Candle)?;
        y.broadcast_add(&bias).map_err(Error::Candle)
    }
}

/// The reference applies its output `GroupNorm` channel-first while the
/// transformer works token-first: transpose both ways around the norm.
fn group_norm_last(norm: &GroupNorm, x: &Tensor) -> Result<Tensor, Error> {
    let y = x
        .permute((0, 2, 1))
        .map_err(Error::Candle)?
        .contiguous()
        .map_err(Error::Candle)?;
    let y = norm.forward(&y)?;
    y.permute((0, 2, 1))
        .map_err(Error::Candle)?
        .contiguous()
        .map_err(Error::Candle)
}

/// Prints the peak/RMS of a stage when `DEMUCS_TRACE` is set (debug only).
fn trace_peak(label: &str, t: &Tensor) -> Result<(), Error> {
    if std::env::var("DEMUCS_TRACE").is_err() {
        return Ok(());
    }
    let v = t
        .to_device(&Device::Cpu)
        .map_err(Error::Candle)?
        .flatten_all()
        .map_err(Error::Candle)?
        .to_vec1::<f32>()
        .map_err(Error::Candle)?;
    let peak = v.iter().fold(0f32, |m, x| m.max(x.abs()));
    let rms = (v
        .iter()
        .map(|x| (*x as f64) * (*x as f64))
        .sum::<f64>()
        / v.len() as f64)
        .sqrt();
    eprintln!("[demucs-trace] {label} peak={peak:.5} rms={rms:.5}");
    if let Ok(dir) = std::env::var("DEMUCS_DUMP") {
        let path = std::path::Path::new(&dir).join(format!("ours-{label}.f32"));
        let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
        let _ = std::fs::write(path, bytes);
    }
    Ok(())
}

fn glu(x: &Tensor, dim: usize) -> Result<Tensor, Error> {
    let n = x.dim(dim).map_err(Error::Candle)?;    if n % 2 != 0 {
        return Err(Error::Config(ConfigError::Invalid(
            "GLU needs an even channel count".into(),
        )));
    }
    let a = x.narrow(dim, 0, n / 2).map_err(Error::Candle)?;
    let b = x.narrow(dim, n / 2, n / 2).map_err(Error::Candle)?;
    let sig = candle_nn::ops::sigmoid(&b).map_err(Error::Candle)?;
    a.broadcast_mul(&sig).map_err(Error::Candle)
}

/// A convolution along the frequency axis with a `(k, 1)` kernel, `(s, 1)`
/// stride and `(p, 0)` padding, applied independently at every time step.
#[derive(Debug)]
struct FreqConv {
    conv: Conv1d,
}

impl FreqConv {
    fn load(
        chin: usize,
        chout: usize,
        kernel: usize,
        stride: usize,
        pad: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let w = vb
            .get((chout, chin, kernel, 1), "weight")
            .map_err(Error::Candle)?
            .squeeze(D::Minus1)
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        let conv = Conv1d::new(
            w,
            Some(b),
            Conv1dConfig {
                padding: pad,
                stride,
                dilation: 1,
                groups: 1,
                ..Default::default()
            },
        );
        let _ = (kernel, stride, pad);
        Ok(Self { conv })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let (b, c, f, t) = x.dims4().map_err(Error::Candle)?;
        // (b, c, f, t) -> (b, t, c, f) -> (b*t, c, f)
        let y = x
            .permute((0, 3, 1, 2))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?
            .reshape((b * t, c, f))
            .map_err(Error::Candle)?;
        let y = self.conv.forward(&y).map_err(Error::Candle)?;
        let fo = y.dim(2).map_err(Error::Candle)?;
        y.reshape((b, t, y.dim(1).map_err(Error::Candle)?, fo))
            .map_err(Error::Candle)?
            .permute((0, 2, 3, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)
    }
}

/// Square `(k,k)`-kernel frequency-domain convolution: decoder rewrites act on
/// the `(freq, time)` plane, not on the frequency axis alone.
#[derive(Debug)]
struct FreqConv2d {
    conv: candle_nn::Conv2d,
}

impl FreqConv2d {
    fn load(
        chin: usize,
        chout: usize,
        kernel: usize,
        pad: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let w = vb
            .get((chout, chin, kernel, kernel), "weight")
            .map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        let cfg = candle_nn::Conv2dConfig {
            padding: pad,
            stride: 1,
            dilation: 1,
            groups: 1,
            ..Default::default()
        };
        Ok(Self {
            conv: candle_nn::Conv2d::new(w, Some(b), cfg),
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        self.conv.forward(x).map_err(Error::Candle)
    }
}

/// `(k,1)`-kernel frequency transposed convolution: per time step, 1-D
/// transposed convolution over the frequency axis.
#[derive(Debug)]
struct FreqConvTranspose {
    weight: Tensor,
    bias: Tensor,
    kernel: usize,
    stride: usize,
}

impl FreqConvTranspose {
    fn load(
        chin: usize,
        chout: usize,
        kernel: usize,
        stride: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let w = vb
            .get((chin, chout, kernel, 1), "weight")
            .map_err(Error::Candle)?
            .squeeze(D::Minus1)
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        Ok(Self {
            weight: w,
            bias: b,
            kernel,
            stride,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let (b, c, f, t) = x.dims4().map_err(Error::Candle)?;
        let s = self.stride;
        let k = self.kernel;
        let n = b * t;
        let y = x
            .permute((0, 3, 1, 2))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?
            .reshape((n, c, f))
            .map_err(Error::Candle)?;
        // `nn.ConvTranspose2d(kernel (k,1), stride (s,1))` along the frequency
        // axis is, per time step, `Conv1d` over the input with `s - 1` zeros
        // inserted after every sample and the kernel flipped:
        //   out[o] = sum_i x[i] * w[o - i*s]   for o in 0 .. (f-1)*s + k - 1.
        // This is spelled out with the verified `conv1d` primitive because
        // `Tensor::conv_transpose1d` in candle 0.11.1 does not reproduce that
        // definition for k > 1 (see `freq_transposed_conv_matches_the_definition`).
        let z = y
            .unsqueeze(3)
            .map_err(Error::Candle)?
            .pad_with_zeros(D::Minus1, 0, s - 1)
            .map_err(Error::Candle)?
            .reshape((n, c, f * s))
            .map_err(Error::Candle)?
            .narrow(2, 0, (f - 1) * s + 1)
            .map_err(Error::Candle)?;
        let w = self
            .weight
            .permute((1, 0, 2))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?
            .flip(&[2])
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let y = z.conv1d(&w, k - 1, 1, 1, 1).map_err(Error::Candle)?;
        let c_out = self.bias.dim(0).map_err(Error::Candle)?;
        let y = y
            .broadcast_add(&self.bias.reshape((1, c_out, 1)).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let fo = y.dim(2).map_err(Error::Candle)?;
        y.reshape((b, t, c_out, fo))
            .map_err(Error::Candle)?
            .permute((0, 2, 3, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)
    }
}

/// `nn.Conv1d` with an explicit kernel/stride/padding (all along time).
#[derive(Debug)]
struct Conv1dLayer {
    conv: Conv1d,
    stride: usize,
}

impl Conv1dLayer {
    fn load(
        chin: usize,
        chout: usize,
        kernel: usize,
        stride: usize,
        pad: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let w = vb.get((chout, chin, kernel), "weight").map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        let conv = Conv1d::new(
            w,
            Some(b),
            Conv1dConfig {
                padding: pad,
                stride,
                dilation: 1,
                groups: 1,
                ..Default::default()
            },
        );
        let _ = (pad, kernel);
        Ok(Self { conv, stride })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let len = x.dim(D::Minus1).map_err(Error::Candle)?;
        let x = if len % self.stride != 0 {
            // HEncLayer pads the tail with zeros so that `len % stride == 0`.
            let pad = self.stride - (len % self.stride);
            let x = x.pad_with_zeros(D::Minus1, 0, pad).map_err(Error::Candle)?;
            x
        } else {
            x.clone()
        };
        self.conv.forward(&x).map_err(Error::Candle)
    }
}

/// 1-D transposed convolution (time axis), no padding, then sliced by the
/// caller exactly like `HDecLayer`.
#[derive(Debug)]
struct ConvTranspose1dLayer {
    weight: Tensor,
    bias: Option<Tensor>,
    stride: usize,
}

impl ConvTranspose1dLayer {
    fn load(chin: usize, chout: usize, kernel: usize, stride: usize, vb: VarBuilder) -> Result<Self, Error> {
        let w = vb.get((chin, chout, kernel), "weight").map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        Ok(Self {
            weight: w,
            bias: Some(b),
            stride,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let y = x
            .conv_transpose1d(&self.weight, 0, 0, self.stride, 1, 1)
            .map_err(Error::Candle)?;
        match &self.bias {
            Some(b) => {
                let b = b.reshape((1, b.dim(0).map_err(Error::Candle)?, 1)).map_err(Error::Candle)?;
                y.broadcast_add(&b).map_err(Error::Candle)
            }
            None => Ok(y),
        }
    }
}

/// A square 2-D convolution (encoder `rewrite` is 1x1, decoder `rewrite` 3x3).
#[derive(Debug)]
struct Conv2dLayer {
    weight: Tensor,
    bias: Option<Tensor>,
    kernel: usize,
    stride: usize,
    pad: usize,
}

impl Conv2dLayer {
    fn load(
        chin: usize,
        chout: usize,
        kernel: usize,
        stride: usize,
        pad: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let w = vb
            .get((chout, chin, kernel, kernel), "weight")
            .map_err(Error::Candle)?;
        let b = vb.get(chout, "bias").map_err(Error::Candle)?;
        Ok(Self {
            weight: w,
            bias: Some(b),
            kernel,
            stride,
            pad,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let y = x
            .conv2d(
                &self.weight,
                self.pad,
                self.stride,
                1,
                1,
            )
            .map_err(Error::Candle)?;
        match &self.bias {
            Some(b) => {
                let c = b.dim(0).map_err(Error::Candle)?;
                let b = b.reshape((1, c, 1, 1)).map_err(Error::Candle)?;
                y.broadcast_add(&b).map_err(Error::Candle)
            }
            None => Ok(y),
        }
    }
}

// ---------------------------------------------------------------------------
// DConv residual branch
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct DConvBlock {
    label: String,
    conv1: Conv1d,
    norm1: GroupNorm,
    conv2: Conv1d,
    norm2: GroupNorm,
    scale: Tensor,
}

impl DConvBlock {
    fn load(
        channels: usize,
        compress: usize,
        dilate: usize,
        label: String,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let hidden = channels / compress;
        let kernel = 3usize;
        let conv1 = candle_nn::conv1d(
            channels,
            hidden,
            kernel,
            Conv1dConfig {
                padding: dilate * (kernel / 2),
                stride: 1,
                dilation: dilate,
                groups: 1,
                ..Default::default()
            },
            vb.pp("0"),
        )
        .map_err(Error::Candle)?;
        let norm1 = GroupNorm::load(hidden, 1, vb.pp("1"))?;
        let conv2 = candle_nn::conv1d(
            hidden,
            2 * channels,
            1,
            Conv1dConfig {
                padding: 0,
                stride: 1,
                dilation: 1,
                groups: 1,
                ..Default::default()
            },
            vb.pp("3"),
        )
        .map_err(Error::Candle)?;
        let norm2 = GroupNorm::load(2 * channels, 1, vb.pp("4"))?;
        let scale = vb.get(channels, "6.scale").map_err(Error::Candle)?;
        Ok(Self {
            label,
            conv1,
            norm1,
            conv2,
            norm2,
            scale,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let h = self.conv1.forward(x).map_err(Error::Candle)?;
        trace_peak(&format!("{}-conv1", self.label), &h)?;
        let h = self.norm1.forward(&h)?;
        trace_peak(&format!("{}-norm1", self.label), &h)?;
        let h = h.gelu_erf().map_err(Error::Candle)?;
        let h = self.conv2.forward(&h).map_err(Error::Candle)?;
        trace_peak(&format!("{}-conv2", self.label), &h)?;
        let h = self.norm2.forward(&h)?;
        trace_peak(&format!("{}-norm2", self.label), &h)?;
        let h = glu(&h, 1)?;
        trace_peak(&format!("{}-glu", self.label), &h)?;
        let c = self.scale.dim(0).map_err(Error::Candle)?;
        let scale = self.scale.reshape((1, c, 1)).map_err(Error::Candle)?;
        let h = h.broadcast_mul(&scale).map_err(Error::Candle)?;
        trace_peak(&format!("{}-scaled", self.label), &h)?;
        Ok(h)
    }
}

#[derive(Debug)]
struct DConv {
    label: String,
    blocks: Vec<DConvBlock>,
}

impl DConv {
    fn load(channels: usize, compress: usize, depth: usize, label: String, vb: VarBuilder) -> Result<Self, Error> {
        let mut blocks = Vec::with_capacity(depth);
        for d in 0..depth {
            let dilate = 1usize << d;
            blocks.push(DConvBlock::load(
                channels,
                compress,
                dilate,
                format!("{label}.layers.{d}"),
                vb.pp("layers").pp(d.to_string()),
            )?);
        }
        Ok(Self { label, blocks })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let mut x = x.clone();
        for (idx, b) in self.blocks.iter().enumerate() {
            let y = b.forward(&x)?;
            x = (&x + &y).map_err(Error::Candle)?;
            trace_peak(&format!("{}-block{idx}", self.label), &x)?;
        }
        Ok(x)
    }
}

// ---------------------------------------------------------------------------
// Encoder / decoder layers
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct HEncLayer {
    label: String,
    freq: bool,
    empty: bool,
    chout: usize,
    conv: ConvOrFreq,
    rewrite: Option<ConvOrFreq>,
    dconv: Option<DConv>,
}

#[derive(Debug)]
enum ConvOrFreq {
    Freq(FreqConv),
    Square(FreqConv2d),
    Time(Conv1dLayer),
}

impl HEncLayer {
    #[allow(clippy::too_many_arguments)]
    fn load(
        cfg: &DemucsConfig,
        chin: usize,
        chout: usize,
        freq: bool,
        empty: bool,
        kernel: usize,
        stride: usize,
        pad: usize,
        context: usize,
        label: String,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let conv = if freq {
            ConvOrFreq::Freq(FreqConv::load(chin, chout, kernel, stride, pad, vb.pp("conv"))?)
        } else {
            ConvOrFreq::Time(Conv1dLayer::load(chin, chout, kernel, stride, pad, vb.pp("conv"))?)
        };
        if empty {
            return Ok(Self {
                label,
                freq,
                empty,
                chout,
                conv,
                rewrite: None,
                dconv: None,
            });
        }
        let kw = 1 + 2 * context;
        let rewrite = if freq {
            ConvOrFreq::Freq(FreqConv::load(
                chout,
                2 * chout,
                kw,
                1,
                context,
                vb.pp("rewrite"),
            )?)
        } else {
            ConvOrFreq::Time(Conv1dLayer::load(
                chout,
                2 * chout,
                kw,
                1,
                context,
                vb.pp("rewrite"),
            )?)
        };
        let dconv = Some(DConv::load(chout, cfg.dconv_comp, cfg.dconv_depth, format!("{label}.dconv"), vb.pp("dconv"))?);
        Ok(Self {
            label,
            freq,
            empty,
            chout,
            conv,
            rewrite: Some(rewrite),
            dconv,
        })
    }

    fn forward(&self, x: &Tensor, inject: Option<&Tensor>) -> Result<Tensor, Error> {
        let y = match &self.conv {
            ConvOrFreq::Freq(c) => c.forward(x)?,
            ConvOrFreq::Square(c) => c.forward(x)?,
            ConvOrFreq::Time(c) => c.forward(x)?,
        };
        trace_peak(&format!("{}-conv", self.label), &y)?;
        if self.empty {
            return Ok(y);
        }
        let mut y = y;
        if let Some(inject) = inject {
            let inject = if inject.dims().len() == 3 && y.dims().len() == 4 {
                inject.unsqueeze(2).map_err(Error::Candle)?
            } else {
                inject.clone()
            };
            y = (&y + &inject).map_err(Error::Candle)?;
        }
        let y = y.gelu_erf().map_err(Error::Candle)?;
        trace_peak(&format!("{}-gelu", self.label), &y)?;
        let y = match &self.dconv {
            Some(dconv) => {
                if self.freq {
                    let (b, c, f, t) = y.dims4().map_err(Error::Candle)?;
                    let y3 = y
                        .permute((0, 2, 1, 3))
                        .map_err(Error::Candle)?
                        .contiguous()
                        .map_err(Error::Candle)?
                        .reshape((b * f, c, t))
                        .map_err(Error::Candle)?;
                    let y3 = dconv.forward(&y3)?;
                    let y = y3
                        .reshape((b, f, c, t))
                        .map_err(Error::Candle)?
                        .permute((0, 2, 1, 3))
                        .map_err(Error::Candle)?
                        .contiguous()
                        .map_err(Error::Candle)?;
                    trace_peak(&format!("{}-dconv", self.label), &y)?;
                    y
                } else {
                    let y = dconv.forward(&y)?;
                    trace_peak(&format!("{}-dconv", self.label), &y)?;
                    y
                }
            }
            None => y,
        };
        match &self.rewrite {
            Some(r) => {
                let z = match r {
                    ConvOrFreq::Freq(c) => c.forward(&y)?,
                    ConvOrFreq::Square(c) => c.forward(&y)?,
                    ConvOrFreq::Time(c) => c.forward(&y)?,
                };
                let out = glu(&z, 1)?;
                trace_peak(&format!("{}-out", self.label), &out)?;
                Ok(out)
            }
            None => Ok(y),
        }
    }
}

#[derive(Debug)]
struct HDecLayer {
    freq: bool,
    empty: bool,
    last: bool,
    pad: usize,
    chin: usize,
    conv_tr: DecConv,
    rewrite: Option<ConvOrFreq>,
    dconv: Option<DConv>,
}

#[derive(Debug)]
enum DecConv {
    Freq(FreqConvTranspose),
    Time(ConvTranspose1dLayer),
}

impl HDecLayer {
    #[allow(clippy::too_many_arguments)]
    fn load(
        cfg: &DemucsConfig,
        chin: usize,
        chout: usize,
        freq: bool,
        empty: bool,
        last: bool,
        kernel: usize,
        stride: usize,
        pad: usize,
        context: usize,
        vb: VarBuilder,
    ) -> Result<Self, Error> {
        let conv_tr = if freq {
            DecConv::Freq(FreqConvTranspose::load(
                chin,
                chout,
                kernel,
                stride,
                vb.pp("conv_tr"),
            )?)
        } else {
            DecConv::Time(ConvTranspose1dLayer::load(
                chin,
                chout,
                kernel,
                stride,
                vb.pp("conv_tr"),
            )?)
        };
        if empty {
            return Ok(Self {
                freq,
                empty,
                last,
                pad,
                chin,
                conv_tr,
                rewrite: None,
                dconv: None,
            });
        }
        let kw = 1 + 2 * context;
        let rewrite = if freq {
            ConvOrFreq::Square(FreqConv2d::load(chin, 2 * chin, kw, context, vb.pp("rewrite"))?)
        } else {
            ConvOrFreq::Time(Conv1dLayer::load(chin, 2 * chin, kw, 1, context, vb.pp("rewrite"))?)
        };
        let dconv = Some(DConv::load(chin, cfg.dconv_comp, cfg.dconv_depth, "decoder.dconv".to_string(), vb.pp("dconv"))?);
        Ok(Self {
            freq,
            empty,
            last,
            pad,
            chin,
            conv_tr,
            rewrite: Some(rewrite),
            dconv,
        })
    }

    /// Returns `(z, pre)` where `pre` is the tensor before the transposed
    /// convolution (used by `HTDemucs.forward` to feed the time decoder).
    fn forward(
        &self,
        x: &Tensor,
        skip: Option<&Tensor>,
        length: usize,
    ) -> Result<(Tensor, Tensor), Error> {
        let mut x = x.clone();
        if self.freq && x.dims().len() == 3 {
            let (b, c, t) = x.dims3().map_err(Error::Candle)?;
            x = x.reshape((b, self.chin, c / self.chin, t)).map_err(Error::Candle)?;
        }
        let y = if !self.empty {
            let skip = skip.ok_or_else(|| {
                Error::Config(ConfigError::Invalid("decoder layer needs a skip tensor".into()))
            })?;
            let x = (&x + skip).map_err(Error::Candle)?;
            let y = match self.rewrite.as_ref().unwrap() {
                ConvOrFreq::Freq(c) => c.forward(&x)?,
                ConvOrFreq::Square(c) => c.forward(&x)?,
                ConvOrFreq::Time(c) => c.forward(&x)?,
            };
            let y = glu(&y, 1)?;
            match &self.dconv {
                Some(dconv) => {
                    if self.freq {
                        let (b, c, f, t) = y.dims4().map_err(Error::Candle)?;
                        let y3 = y
                            .permute((0, 2, 1, 3))
                            .map_err(Error::Candle)?
                            .contiguous()
                            .map_err(Error::Candle)?
                            .reshape((b * f, c, t))
                            .map_err(Error::Candle)?;
                        let y3 = dconv.forward(&y3)?;
                        y3.reshape((b, f, c, t))
                            .map_err(Error::Candle)?
                            .permute((0, 2, 1, 3))
                            .map_err(Error::Candle)?
                            .contiguous()
                            .map_err(Error::Candle)?
                    } else {
                        dconv.forward(&y)?
                    }
                }
                None => y,
            }
        } else {
            x
        };
        let z = match &self.conv_tr {
            DecConv::Freq(c) => c.forward(&y)?,
            DecConv::Time(c) => c.forward(&y)?,
        };
        let z = if self.freq {
            let f = z.dim(2).map_err(Error::Candle)?;
            if self.pad > 0 {
                z.narrow(2, self.pad, f - 2 * self.pad).map_err(Error::Candle)?
            } else {
                z
            }
        } else {
            z.narrow(2, self.pad, length).map_err(Error::Candle)?
        };
        let z = if self.last {
            z
        } else {
            z.gelu_erf().map_err(Error::Candle)?
        };
        Ok((z, y))
    }
}

// ---------------------------------------------------------------------------
// Transformer
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Mha {
    dim: usize,
    heads: usize,
    in_proj_weight: Tensor,
    in_proj_bias: Tensor,
    out_proj: Linear,
}

impl Mha {
    fn load(dim: usize, heads: usize, vb: VarBuilder) -> Result<Self, Error> {
        let in_proj_weight = vb.get((3 * dim, dim), "in_proj_weight").map_err(Error::Candle)?;
        let in_proj_bias = vb.get(3 * dim, "in_proj_bias").map_err(Error::Candle)?;
        let out_proj = candle_nn::linear(dim, dim, vb.pp("out_proj")).map_err(Error::Candle)?;
        Ok(Self {
            dim,
            heads,
            in_proj_weight,
            in_proj_bias,
            out_proj,
        })
    }

    fn project(&self, x: &Tensor, which: usize) -> Result<Tensor, Error> {
        // x: (b, t, dim); take rows [which*dim .. (which+1)*dim] of in_proj_weight.
        let w = self
            .in_proj_weight
            .narrow(0, which * self.dim, self.dim)
            .map_err(Error::Candle)?;
        let b = self
            .in_proj_bias
            .narrow(0, which * self.dim, self.dim)
            .map_err(Error::Candle)?;
        let y = x
            .broadcast_matmul(&w.t().map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        y.broadcast_add(&b).map_err(Error::Candle)
    }

    fn forward(&self, q: &Tensor, k: &Tensor, v: &Tensor) -> Result<Tensor, Error> {
        let (b, tq, _) = q.dims3().map_err(Error::Candle)?;
        let (_, tk, _) = k.dims3().map_err(Error::Candle)?;
        let hd = self.dim / self.heads;
        let qp = self.project(q, 0)?;
        let kp = self.project(k, 1)?;
        let vp = self.project(v, 2)?;
        let qp = qp
            .reshape((b, tq, self.heads, hd))
            .map_err(Error::Candle)?
            .permute((0, 2, 1, 3))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let kp = kp
            .reshape((b, tk, self.heads, hd))
            .map_err(Error::Candle)?
            .permute((0, 2, 1, 3))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let vp = vp
            .reshape((b, tk, self.heads, hd))
            .map_err(Error::Candle)?
            .permute((0, 2, 1, 3))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let scale = (hd as f64).powf(-0.5);
        let scale_t = Tensor::new(scale as f32, q.device()).map_err(Error::Candle)?;
        let kt = kp.transpose(2, 3).map_err(Error::Candle)?.contiguous().map_err(Error::Candle)?;

        // Blocked over the query axis to bound the attention matrix.
        const BLOCK: usize = 512;
        let mut blocks = Vec::with_capacity(tq.div_ceil(BLOCK));
        let mut start = 0usize;
        while start < tq {
            let len = BLOCK.min(tq - start);
            let q_blk = qp.narrow(2, start, len).map_err(Error::Candle)?;
            let scores = q_blk.matmul(&kt).map_err(Error::Candle)?;
            let scores = scores.broadcast_mul(&scale_t).map_err(Error::Candle)?;
            let attn = candle_nn::ops::softmax(&scores, D::Minus1).map_err(Error::Candle)?;
            blocks.push(attn.matmul(&vp).map_err(Error::Candle)?);
            start += len;
        }
        let out = Tensor::cat(&blocks.iter().collect::<Vec<_>>(), 2).map_err(Error::Candle)?;
        let out = out
            .permute((0, 2, 1, 3))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?
            .reshape((b, tq, self.dim))
            .map_err(Error::Candle)?;
        self.out_proj.forward(&out).map_err(Error::Candle)
    }
}

#[derive(Debug)]
struct SelfLayer {
    attn: Mha,
    linear1: Linear,
    linear2: Linear,
    norm1: LayerNorm,
    norm2: LayerNorm,
    norm_out: GroupNorm,
    gamma1: Tensor,
    gamma2: Tensor,
}

impl SelfLayer {
    fn load(dim: usize, heads: usize, hidden: usize, vb: VarBuilder) -> Result<Self, Error> {
        let cfg = LayerNormConfig {
            eps: LAYER_NORM_EPS,
            remove_mean: true,
            affine: true,
        };
        Ok(Self {
            attn: Mha::load(dim, heads, vb.pp("self_attn"))?,
            linear1: candle_nn::linear(dim, hidden, vb.pp("linear1")).map_err(Error::Candle)?,
            linear2: candle_nn::linear(hidden, dim, vb.pp("linear2")).map_err(Error::Candle)?,
            norm1: layer_norm(dim, cfg, vb.pp("norm1")).map_err(Error::Candle)?,
            norm2: layer_norm(dim, cfg, vb.pp("norm2")).map_err(Error::Candle)?,
            norm_out: GroupNorm::load(dim, 1, vb.pp("norm_out"))?,
            gamma1: vb.get(dim, "gamma_1.scale").map_err(Error::Candle)?,
            gamma2: vb.get(dim, "gamma_2.scale").map_err(Error::Candle)?,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let h = self.norm1.forward(x).map_err(Error::Candle)?;
        let a = self.attn.forward(&h, &h, &h)?;
        let a = a.broadcast_mul(&self.gamma1).map_err(Error::Candle)?;
        let x = (x + a).map_err(Error::Candle)?;
        let h = self.norm2.forward(&x).map_err(Error::Candle)?;
        let f = self.feed_forward(&h)?;
        let f = f.broadcast_mul(&self.gamma2).map_err(Error::Candle)?;
        let x = (&x + f).map_err(Error::Candle)?;
        group_norm_last(&self.norm_out, &x)
    }

    fn feed_forward(&self, x: &Tensor) -> Result<Tensor, Error> {
        let h = self.linear1.forward(x).map_err(Error::Candle)?;
        let h = h.gelu_erf().map_err(Error::Candle)?;
        self.linear2.forward(&h).map_err(Error::Candle)
    }
}

#[derive(Debug)]
struct CrossLayer {
    cross_attn: Mha,
    linear1: Linear,
    linear2: Linear,
    norm1: LayerNorm,
    norm2: LayerNorm,
    norm3: LayerNorm,
    norm_out: GroupNorm,
    gamma1: Tensor,
    gamma2: Tensor,
}

impl CrossLayer {
    fn load(dim: usize, heads: usize, hidden: usize, vb: VarBuilder) -> Result<Self, Error> {
        let cfg = LayerNormConfig {
            eps: LAYER_NORM_EPS,
            remove_mean: true,
            affine: true,
        };
        Ok(Self {
            cross_attn: Mha::load(dim, heads, vb.pp("cross_attn"))?,
            linear1: candle_nn::linear(dim, hidden, vb.pp("linear1")).map_err(Error::Candle)?,
            linear2: candle_nn::linear(hidden, dim, vb.pp("linear2")).map_err(Error::Candle)?,
            norm1: layer_norm(dim, cfg, vb.pp("norm1")).map_err(Error::Candle)?,
            norm2: layer_norm(dim, cfg, vb.pp("norm2")).map_err(Error::Candle)?,
            norm3: layer_norm(dim, cfg, vb.pp("norm3")).map_err(Error::Candle)?,
            norm_out: GroupNorm::load(dim, 1, vb.pp("norm_out"))?,
            gamma1: vb.get(dim, "gamma_1.scale").map_err(Error::Candle)?,
            gamma2: vb.get(dim, "gamma_2.scale").map_err(Error::Candle)?,
        })
    }

    fn forward(&self, q: &Tensor, k: &Tensor) -> Result<Tensor, Error> {
        let qn = self.norm1.forward(q).map_err(Error::Candle)?;
        let kn = self.norm2.forward(k).map_err(Error::Candle)?;
        let a = self.cross_attn.forward(&qn, &kn, &kn)?;
        let a = a.broadcast_mul(&self.gamma1).map_err(Error::Candle)?;
        let x = (q + a).map_err(Error::Candle)?;
        let h = self.norm3.forward(&x).map_err(Error::Candle)?;
        let h = self.linear1.forward(&h).map_err(Error::Candle)?;
        let h = h.gelu_erf().map_err(Error::Candle)?;
        let h = self.linear2.forward(&h).map_err(Error::Candle)?;
        let h = h.broadcast_mul(&self.gamma2).map_err(Error::Candle)?;
        let x = (&x + h).map_err(Error::Candle)?;
        group_norm_last(&self.norm_out, &x)
    }
}

#[derive(Debug)]
enum TLayer {
    SelfAttn(SelfLayer),
    Cross(CrossLayer),
}

impl TLayer {
    fn forward(&self, x: &Tensor, other: Option<&Tensor>) -> Result<Tensor, Error> {
        match self {
            TLayer::SelfAttn(l) => l.forward(x),
            TLayer::Cross(l) => {
                let other = other.ok_or_else(|| {
                    Error::Config(ConfigError::Invalid("cross layer needs a second stream".into()))
                })?;
                l.forward(x, other)
            }
        }
    }
}

#[derive(Debug)]
struct CrossTransformerEncoder {
    dim: usize,
    max_period: f64,
    weight_pos_embed: f64,
    norm_in: LayerNorm,
    norm_in_t: LayerNorm,
    layers: Vec<TLayer>,
    layers_t: Vec<TLayer>,
}

impl CrossTransformerEncoder {
    fn load(cfg: &DemucsConfig, dim: usize, vb: VarBuilder) -> Result<Self, Error> {
        let hidden = dim * cfg.t_hidden_scale;
        let ln_cfg = LayerNormConfig {
            eps: LAYER_NORM_EPS,
            remove_mean: true,
            affine: true,
        };
        let mut layers = Vec::with_capacity(cfg.t_layers);
        let mut layers_t = Vec::with_capacity(cfg.t_layers);
        for idx in 0..cfg.t_layers {
            // cross_first = false -> even index is a self-attention layer.
            let (a, b) = if idx % 2 == 0 {
                (
                    TLayer::SelfAttn(SelfLayer::load(
                        dim,
                        cfg.t_heads,
                        hidden,
                        vb.pp("layers").pp(idx.to_string()),
                    )?),
                    TLayer::SelfAttn(SelfLayer::load(
                        dim,
                        cfg.t_heads,
                        hidden,
                        vb.pp("layers_t").pp(idx.to_string()),
                    )?),
                )
            } else {
                (
                    TLayer::Cross(CrossLayer::load(
                        dim,
                        cfg.t_heads,
                        hidden,
                        vb.pp("layers").pp(idx.to_string()),
                    )?),
                    TLayer::Cross(CrossLayer::load(
                        dim,
                        cfg.t_heads,
                        hidden,
                        vb.pp("layers_t").pp(idx.to_string()),
                    )?),
                )
            };
            layers.push(a);
            layers_t.push(b);
        }
        Ok(Self {
            dim,
            max_period: cfg.t_max_period,
            weight_pos_embed: 1.0,
            norm_in: layer_norm(dim, ln_cfg, vb.pp("norm_in")).map_err(Error::Candle)?,
            norm_in_t: layer_norm(dim, ln_cfg, vb.pp("norm_in_t")).map_err(Error::Candle)?,
            layers,
            layers_t,
        })
    }

    /// `x`: `(b, c, f, t)`, `xt`: `(b, c, t2)`.
    fn forward(&self, x: &Tensor, xt: &Tensor) -> Result<(Tensor, Tensor), Error> {
        let (b, c, f, t1) = x.dims4().map_err(Error::Candle)?;
        let pos2d = sin_2d_embedding(c, f, t1, self.max_period, x.device())?;
        let x = x
            .permute((0, 3, 2, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?
            .reshape((b, t1 * f, c))
            .map_err(Error::Candle)?;
        let x = self.norm_in.forward(&x).map_err(Error::Candle)?;
        let pos2d = pos2d
            .broadcast_mul(&Tensor::new(self.weight_pos_embed as f32, x.device()).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let mut x = x.broadcast_add(&pos2d).map_err(Error::Candle)?;

        let t2 = xt.dim(2).map_err(Error::Candle)?;
        let posx = sin_1d_embedding(t2, c, self.max_period, xt.device())?;
        let mut xt = xt
            .permute((0, 2, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        xt = self.norm_in_t.forward(&xt).map_err(Error::Candle)?;
        xt = xt.broadcast_add(&posx).map_err(Error::Candle)?;

        for idx in 0..self.layers.len() {
            if idx % 2 == 0 {
                x = self.layers[idx].forward(&x, None)?;
                xt = self.layers_t[idx].forward(&xt, None)?;
            } else {
                let old_x = x.clone();
                x = self.layers[idx].forward(&x, Some(&xt))?;
                xt = self.layers_t[idx].forward(&xt, Some(&old_x))?;
            }
        }

        let x = x
            .reshape((b, t1, f, c))
            .map_err(Error::Candle)?
            .permute((0, 3, 2, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        let xt = xt
            .permute((0, 2, 1))
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        Ok((x, xt))
    }
}

/// `create_2d_sin_embedding(d_model, height=freq, width=time)` reshaped to
/// `(1, time*freq, dim)` — i.e. the token layout `b (t1 fr) c`.
fn sin_2d_embedding(
    dim: usize,
    height: usize,
    width: usize,
    max_period: f64,
    device: &Device,
) -> Result<Tensor, Error> {
    let half = dim / 2;
    let div: Vec<f64> = (0..half / 2)
        .map(|k| (-(2.0 * k as f64) * max_period.ln() / half as f64).exp())
        .collect();
    let mut data = vec![0f32; width * height * dim];
    // token layout: (time, freq) -> dim
    for t in 0..width {
        for fr in 0..height {
            let tok = t * height + fr;
            for k in 0..half / 2 {
                let sin_w = (t as f64 * div[k]).sin() as f32;
                let cos_w = (t as f64 * div[k]).cos() as f32;
                let sin_h = (fr as f64 * div[k]).sin() as f32;
                let cos_h = (fr as f64 * div[k]).cos() as f32;
                data[tok * dim + 2 * k] = sin_w;
                data[tok * dim + 2 * k + 1] = cos_w;
                data[tok * dim + half + 2 * k] = sin_h;
                data[tok * dim + half + 2 * k + 1] = cos_h;
            }
        }
    }
    Tensor::from_vec(data, (1, width * height, dim), device).map_err(Error::Candle)
}

/// `create_sin_embedding(length, dim, shift=0)` -> `(1, length, dim)`.
fn sin_1d_embedding(
    length: usize,
    dim: usize,
    max_period: f64,
    device: &Device,
) -> Result<Tensor, Error> {
    let half = dim / 2;
    let phase_div: Vec<f64> = (0..half)
        .map(|k| max_period.powf(k as f64 / (half as f64 - 1.0)))
        .collect();
    let mut data = vec![0f32; length * dim];
    for t in 0..length {
        for k in 0..half {
            let p = t as f64 / phase_div[k];
            data[t * dim + k] = p.cos() as f32;
            data[t * dim + half + k] = p.sin() as f32;
        }
    }
    Tensor::from_vec(data, (1, length, dim), device).map_err(Error::Candle)
}




// ---------------------------------------------------------------------------
// Layer plan (mirrors the HTDemucs.__init__ loop one for one)
// ---------------------------------------------------------------------------

struct LayerPlan {
    freq: bool,
    freq_bins: usize,
    enc_ker: usize,
    enc_stride: usize,
    enc_pad: usize,
    last_freq: bool,
    enc_chin: usize,
    enc_chout: usize,
    tenc_chin: usize,
    tenc_chout: usize,
    dec_chin: usize,
    dec_chout: usize,
    tdec_chin: usize,
    tdec_chout: usize,
    last: bool,
}

fn layer_plans(cfg: &DemucsConfig) -> Vec<LayerPlan> {
    let cac = 2usize;
    let mut plans = Vec::with_capacity(cfg.depth);
    let mut chin = cfg.audio_channels;
    let mut chin_z = chin * cac;
    let mut chout = cfg.channels;
    let mut chout_z = cfg.channels;
    let mut freqs = NFFT / 2;
    for index in 0..cfg.depth {
        let freq = freqs > 1;
        let mut ker = cfg.kernel_size;
        let mut stri = cfg.stride;
        let mut pad = cfg.kernel_size / 4;
        let mut last_freq = false;
        if !freq {
            // Only reachable once the frequency axis has collapsed to 1; kept
            // for completeness of the reference rule.
            ker = 2 * 2;
            stri = 2;
        }
        if freq && freqs <= cfg.kernel_size {
            ker = freqs;
            pad = 0;
            last_freq = true;
        }
        if last_freq {
            chout_z = chout.max(chout_z);
            chout = chout_z;
        }
        let enc_chin = chin_z;
        let enc_chout = chout_z;
        let tenc_chin = chin;
        let tenc_chout = chout;
        if index == 0 {
            chin = cfg.audio_channels * SOURCES.len();
            chin_z = chin * cac;
        }
        let dec_chin = chout_z;
        let dec_chout = chin_z;
        let tdec_chin = chout;
        let tdec_chout = chin;
        plans.push(LayerPlan {
            freq,
            freq_bins: freqs,
            enc_ker: ker,
            enc_stride: stri,
            enc_pad: pad,
            last_freq,
            enc_chin,
            enc_chout,
            tenc_chin,
            tenc_chout,
            dec_chin,
            dec_chout,
            tdec_chin,
            tdec_chout,
            last: index == 0,
        });
        chin = chout;
        chin_z = chout_z;
        chout *= cfg.growth;
        chout_z *= cfg.growth;
        if freq {
            freqs = if freqs <= cfg.kernel_size {
                1
            } else {
                freqs / cfg.stride
            };
        }
    }
    plans
}

// ---------------------------------------------------------------------------
// Expected tensor schema
// ---------------------------------------------------------------------------

fn push_dconv(out: &mut Vec<(String, Vec<usize>)>, prefix: &str, channels: usize, compress: usize, depth: usize) {
    let hidden = channels / compress;
    for d in 0..depth {
        let p = format!("{prefix}.layers.{d}");
        out.push((format!("{p}.0.weight"), vec![hidden, channels, 3]));
        out.push((format!("{p}.0.bias"), vec![hidden]));
        out.push((format!("{p}.1.weight"), vec![hidden]));
        out.push((format!("{p}.1.bias"), vec![hidden]));
        out.push((format!("{p}.3.weight"), vec![2 * channels, hidden, 1]));
        out.push((format!("{p}.3.bias"), vec![2 * channels]));
        out.push((format!("{p}.4.weight"), vec![2 * channels]));
        out.push((format!("{p}.4.bias"), vec![2 * channels]));
        out.push((format!("{p}.6.scale"), vec![channels]));
    }
}

fn push_transformer_layer(
    out: &mut Vec<(String, Vec<usize>)>,
    prefix: &str,
    dim: usize,
    hidden: usize,
    cross: bool,
) {
    let attn = if cross { "cross_attn" } else { "self_attn" };
    out.push((format!("{prefix}.{attn}.in_proj_weight"), vec![3 * dim, dim]));
    out.push((format!("{prefix}.{attn}.in_proj_bias"), vec![3 * dim]));
    out.push((format!("{prefix}.{attn}.out_proj.weight"), vec![dim, dim]));
    out.push((format!("{prefix}.{attn}.out_proj.bias"), vec![dim]));
    out.push((format!("{prefix}.linear1.weight"), vec![hidden, dim]));
    out.push((format!("{prefix}.linear1.bias"), vec![hidden]));
    out.push((format!("{prefix}.linear2.weight"), vec![dim, hidden]));
    out.push((format!("{prefix}.linear2.bias"), vec![dim]));
    out.push((format!("{prefix}.norm1.weight"), vec![dim]));
    out.push((format!("{prefix}.norm1.bias"), vec![dim]));
    out.push((format!("{prefix}.norm2.weight"), vec![dim]));
    out.push((format!("{prefix}.norm2.bias"), vec![dim]));
    if cross {
        out.push((format!("{prefix}.norm3.weight"), vec![dim]));
        out.push((format!("{prefix}.norm3.bias"), vec![dim]));
    }
    out.push((format!("{prefix}.norm_out.weight"), vec![dim]));
    out.push((format!("{prefix}.norm_out.bias"), vec![dim]));
    out.push((format!("{prefix}.gamma_1.scale"), vec![dim]));
    out.push((format!("{prefix}.gamma_2.scale"), vec![dim]));
}

/// The exact tensor schema of this profile: name -> shape, in the order the
/// builder consumes it. Mirrors the Python module tree for the pinned config.
pub fn expected_key_shapes(cfg: &DemucsConfig) -> Vec<(String, Vec<usize>)> {
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    let plans = layer_plans(cfg);
    let context_enc = cfg.context_enc;
    let context = cfg.context;

    for (i, p) in plans.iter().enumerate() {
        let enc = format!("encoder.{i}");
        out.push((format!("{enc}.conv.weight"), vec![p.enc_chout, p.enc_chin, p.enc_ker, 1]));
        out.push((format!("{enc}.conv.bias"), vec![p.enc_chout]));
        out.push((
            format!("{enc}.rewrite.weight"),
            vec![2 * p.enc_chout, p.enc_chout, 1 + 2 * context_enc, 1],
        ));
        out.push((format!("{enc}.rewrite.bias"), vec![2 * p.enc_chout]));
        push_dconv(&mut out, &format!("{enc}.dconv"), p.enc_chout, cfg.dconv_comp, cfg.dconv_depth);

        let tenc = format!("tencoder.{i}");
        out.push((
            format!("{tenc}.conv.weight"),
            vec![p.tenc_chout, p.tenc_chin, cfg.kernel_size],
        ));
        out.push((format!("{tenc}.conv.bias"), vec![p.tenc_chout]));
        if !p.last_freq {
            out.push((
                format!("{tenc}.rewrite.weight"),
                vec![2 * p.tenc_chout, p.tenc_chout, 1 + 2 * context_enc],
            ));
            out.push((format!("{tenc}.rewrite.bias"), vec![2 * p.tenc_chout]));
            push_dconv(
                &mut out,
                &format!("{tenc}.dconv"),
                p.tenc_chout,
                cfg.dconv_comp,
                cfg.dconv_depth,
            );
        }
    }

    if cfg.depth > 0 {
        // freq_emb is created after index 0: (freqs_after_first_stride, channels)
        out.push((
            "freq_emb.embedding.weight".to_string(),
            vec![NFFT / 2 / cfg.stride, cfg.channels],
        ));
    }

    let transformer_dim = cfg.bottom_channels;
    for name in ["channel_upsampler", "channel_downsampler", "channel_upsampler_t", "channel_downsampler_t"] {
        let (chin, chout) = if name.starts_with("channel_up") {
            (cfg.channels * cfg.growth.pow((cfg.depth - 1) as u32), transformer_dim)
        } else {
            (transformer_dim, cfg.channels * cfg.growth.pow((cfg.depth - 1) as u32))
        };
        out.push((format!("{name}.weight"), vec![chout, chin, 1]));
        out.push((format!("{name}.bias"), vec![chout]));
    }

    out.push(("crosstransformer.norm_in.weight".into(), vec![transformer_dim]));
    out.push(("crosstransformer.norm_in.bias".into(), vec![transformer_dim]));
    out.push(("crosstransformer.norm_in_t.weight".into(), vec![transformer_dim]));
    out.push(("crosstransformer.norm_in_t.bias".into(), vec![transformer_dim]));
    let hidden = transformer_dim * cfg.t_hidden_scale;
    for idx in 0..cfg.t_layers {
        let cross = idx % 2 == 1;
        for branch in ["layers", "layers_t"] {
            push_transformer_layer(
                &mut out,
                &format!("crosstransformer.{branch}.{idx}"),
                transformer_dim,
                hidden,
                cross,
            );
        }
    }

    // Decoder layers are stored in reverse creation order (insert(0, ..)).
    for (i, p) in plans.iter().enumerate().rev() {
        // decoder.{j} holds the layer created at plan index i = depth - 1 - j
        let j = cfg.depth - 1 - i;
        let dec = format!("decoder.{j}");
        out.push((format!("{dec}.conv_tr.weight"), vec![p.dec_chin, p.dec_chout, p.enc_ker, 1]));
        out.push((format!("{dec}.conv_tr.bias"), vec![p.dec_chout]));
        out.push((
            format!("{dec}.rewrite.weight"),
            vec![2 * p.dec_chin, p.dec_chin, 1 + 2 * context, 1 + 2 * context],
        ));
        out.push((format!("{dec}.rewrite.bias"), vec![2 * p.dec_chin]));
        push_dconv(&mut out, &format!("{dec}.dconv"), p.dec_chin, cfg.dconv_comp, cfg.dconv_depth);

        let tdec = format!("tdecoder.{j}");
        out.push((
            format!("{tdec}.conv_tr.weight"),
            vec![p.tdec_chin, p.tdec_chout, cfg.kernel_size],
        ));
        out.push((format!("{tdec}.conv_tr.bias"), vec![p.tdec_chout]));
        if !p.last_freq {
            out.push((
                format!("{tdec}.rewrite.weight"),
                vec![2 * p.tdec_chin, p.tdec_chin, 1 + 2 * context],
            ));
            out.push((format!("{tdec}.rewrite.bias"), vec![2 * p.tdec_chin]));
            push_dconv(
                &mut out,
                &format!("{tdec}.dconv"),
                p.tdec_chin,
                cfg.dconv_comp,
                cfg.dconv_depth,
            );
        }
    }
    out
}

/// Strict schema validation: every expected tensor must be present with the
/// exact shape and dtype, and nothing else may be present.
pub fn validate_state(
    tensors: &std::collections::HashMap<String, Tensor>,
    cfg: &DemucsConfig,
) -> Result<(), Error> {
    let expected = expected_key_shapes(cfg);
    let mut missing: Vec<String> = Vec::new();
    let mut mis_shaped: Vec<String> = Vec::new();
    for (name, shape) in &expected {
        match tensors.get(name) {
            None => missing.push(name.clone()),
            Some(t) => {
                if t.dims() != shape.as_slice() {
                    mis_shaped.push(format!("{name}: {:?} != {shape:?}", t.dims()));
                } else if t.dtype() != DType::F32 {
                    mis_shaped.push(format!("{name}: dtype {:?} != F32", t.dtype()));
                }
            }
        }
    }
    let expected_names: std::collections::HashSet<&str> =
        expected.iter().map(|(n, _)| n.as_str()).collect();
    let mut unexpected: Vec<String> = tensors
        .keys()
        .filter(|k| !expected_names.contains(k.as_str()))
        .cloned()
        .collect();
    unexpected.sort();
    if !missing.is_empty() || !unexpected.is_empty() || !mis_shaped.is_empty() {
        return Err(Error::Config(ConfigError::Invalid(format!(
            "HTDemucs checkpoint does not match the expected schema: {} missing, {} unexpected, \
             {} mis-shaped. first missing: {:?}; first unexpected: {:?}; first mis-shaped: {:?}",
            missing.len(),
            unexpected.len(),
            mis_shaped.len(),
            missing.first(),
            unexpected.first(),
            mis_shaped.first()
        ))));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The network
// ---------------------------------------------------------------------------

/// A loaded HTDemucs network (4 stems: drums, bass, other, vocals).
pub struct HTDemucs {
    pub cfg: DemucsConfig,
    encoder: Vec<HEncLayer>,
    tencoder: Vec<HEncLayer>,
    /// Stored in application order (`decoder.0` first).
    decoder: Vec<HDecLayer>,
    tdecoder: Vec<HDecLayer>,
    freq_emb: Option<Tensor>,
    freq_emb_scale: f64,
    upsample: Conv1dLayer,
    downsample: Conv1dLayer,
    upsample_t: Conv1dLayer,
    downsample_t: Conv1dLayer,
    transformer: CrossTransformerEncoder,
    transformer_dim: usize,
    device: Device,
}

impl HTDemucs {
    pub fn load(
        tensors: std::collections::HashMap<String, Tensor>,
        cfg: DemucsConfig,
        device: &Device,
    ) -> Result<Self, Error> {
        validate_state(&tensors, &cfg)?;
        let vb = VarBuilder::from_tensors(tensors, DType::F32, device);
        Self::new(&cfg, vb)
    }

    fn new(cfg: &DemucsConfig, vb: VarBuilder) -> Result<Self, Error> {
        let plans = layer_plans(cfg);
        let mut encoder = Vec::with_capacity(plans.len());
        let mut tencoder = Vec::with_capacity(plans.len());
        let mut decoders_created: Vec<HDecLayer> = Vec::with_capacity(plans.len());
        let mut tdecoders_created: Vec<HDecLayer> = Vec::with_capacity(plans.len());

        for (i, p) in plans.iter().enumerate() {
            let enc = HEncLayer::load(
                cfg,
                p.enc_chin,
                p.enc_chout,
                p.freq,
                false,
                p.enc_ker,
                p.enc_stride,
                p.enc_pad,
                cfg.context_enc,
                format!("encoder.{i}"),
                vb.pp("encoder").pp(i.to_string()),
            )?;
            encoder.push(enc);
            let tenc = HEncLayer::load(
                cfg,
                p.tenc_chin,
                p.tenc_chout,
                false,
                p.last_freq,
                cfg.kernel_size,
                cfg.stride,
                cfg.kernel_size / 4,
                cfg.context_enc,
                format!("tencoder.{i}"),
                vb.pp("tencoder").pp(i.to_string()),
            )?;
            tencoder.push(tenc);
            let dec = HDecLayer::load(
                cfg,
                p.dec_chin,
                p.dec_chout,
                p.freq,
                false,
                p.last,
                p.enc_ker,
                p.enc_stride,
                p.enc_pad,
                cfg.context,
                vb.pp("decoder").pp((cfg.depth - 1 - i).to_string()),
            )?;
            decoders_created.push(dec);
            let tdec = HDecLayer::load(
                cfg,
                p.tdec_chin,
                p.tdec_chout,
                false,
                p.last_freq,
                p.last,
                cfg.kernel_size,
                cfg.stride,
                cfg.kernel_size / 4,
                cfg.context,
                vb.pp("tdecoder").pp((cfg.depth - 1 - i).to_string()),
            )?;
            tdecoders_created.push(tdec);
        }
        // The reference inserts each new decoder/tdecoder at index 0.
        decoders_created.reverse();
        tdecoders_created.reverse();

        let freq_emb = if cfg.freq_emb_scale > 0.0 {
            let w = vb
                .get(
                    (NFFT / 2 / cfg.stride, cfg.channels),
                    "freq_emb.embedding.weight",
                )
                .map_err(Error::Candle)?;
            // ScaledEmbedding.forward multiplies by `scale` (10.0); the stored
            // weight was divided by it at init time, so the effective table is
            // weight * scale.
            let w = (w * cfg.emb_scale).map_err(Error::Candle)?;
            Some(w.t().map_err(Error::Candle)?.contiguous().map_err(Error::Candle)?)
        } else {
            None
        };

        let h = cfg.channels * cfg.growth.pow((cfg.depth - 1) as u32);
        let up = Conv1dLayer::load(h, cfg.bottom_channels, 1, 1, 0, vb.pp("channel_upsampler"))?;
        let down = Conv1dLayer::load(cfg.bottom_channels, h, 1, 1, 0, vb.pp("channel_downsampler"))?;
        let up_t = Conv1dLayer::load(h, cfg.bottom_channels, 1, 1, 0, vb.pp("channel_upsampler_t"))?;
        let down_t =
            Conv1dLayer::load(cfg.bottom_channels, h, 1, 1, 0, vb.pp("channel_downsampler_t"))?;

        let transformer =
            CrossTransformerEncoder::load(cfg, cfg.bottom_channels, vb.pp("crosstransformer"))?;

        Ok(Self {
            cfg: cfg.clone(),
            encoder,
            tencoder,
            decoder: decoders_created,
            tdecoder: tdecoders_created,
            freq_emb,
            freq_emb_scale: cfg.freq_emb_scale,
            upsample: up,
            downsample: down,
            upsample_t: up_t,
            downsample_t: down_t,
            transformer,
            transformer_dim: cfg.bottom_channels,
            device: vb.device().clone(),
        })
    }

    pub fn segment_samples(&self) -> usize {
        self.cfg.segment_samples()
    }

    pub fn num_sources(&self) -> usize {
        SOURCES.len()
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Run one `segment_samples`-sample window and return the four stems as
    /// `(batch, sources, channels, segment_samples)`.
    pub fn forward_segment(&self, mix: &Tensor) -> Result<Tensor, Error> {
        let (b, channels, len) = mix.dims3().map_err(Error::Candle)?;
        if channels != self.cfg.audio_channels {
            return Err(Error::Config(ConfigError::Invalid(format!(
                "HTDemucs expects {} channels, got {channels}",
                self.cfg.audio_channels
            ))));
        }
        if len != self.segment_samples() {
            return Err(Error::Config(ConfigError::Invalid(format!(
                "HTDemucs expects a {}-sample segment, got {len}",
                self.segment_samples()
            ))));
        }
        let device = mix.device().clone();

        // ---- spectrogram branch input -------------------------------------
        let (spec_re, spec_im, frames) = self.spec(mix)?; // (b, ch, 2048, T) each
        trace_peak("spec-re", &spec_re)?;
        let z = Tensor::stack(&[&spec_re, &spec_im], 4).map_err(Error::Candle)?; // (b, ch, Fq, T, 2)
        let (b, ch, fq, t) = spec_re.dims4().map_err(Error::Candle)?;
        let mag = z
            .permute((0, 1, 4, 2, 3))
            .map_err(Error::Candle)?
            .reshape((b, 2 * ch, fq, t))
            .map_err(Error::Candle)?;
        let (_, c2, fq, t) = mag.dims4().map_err(Error::Candle)?;
        let count = (c2 * fq * t) as f64;
        let sum = mag.sum_all().map_err(Error::Candle)?;
        let mean = (sum / count).map_err(Error::Candle)?;
        // torch std default: unbiased (N - 1).
        let diff = mag.broadcast_sub(&mean).map_err(Error::Candle)?;
        let sq = diff.sqr().map_err(Error::Candle)?.sum_all().map_err(Error::Candle)?;
        let std = (sq / (count - 1.0)).map_err(Error::Candle)?.sqrt().map_err(Error::Candle)?;
        let denom = (std.clone() + 1e-5).map_err(Error::Candle)?;
        let x = diff.broadcast_div(&denom).map_err(Error::Candle)?;
        trace_peak("spec-norm", &x)?;

        // ---- waveform branch input ----------------------------------------
        let tcount = (channels * len) as f64;
        let tsum = mix.sum_all().map_err(Error::Candle)?;
        let tmean = (tsum / tcount).map_err(Error::Candle)?;
        let tdiff = mix.broadcast_sub(&tmean).map_err(Error::Candle)?;
        let tsq = tdiff.sqr().map_err(Error::Candle)?.sum_all().map_err(Error::Candle)?;
        let tstd = (tsq / (tcount - 1.0)).map_err(Error::Candle)?.sqrt().map_err(Error::Candle)?;
        let tdenom = (tstd.clone() + 1e-5).map_err(Error::Candle)?;
        let mut xt = tdiff.broadcast_div(&tdenom).map_err(Error::Candle)?;

        // ---- encoder -------------------------------------------------------
        let mut saved: Vec<Tensor> = Vec::with_capacity(self.encoder.len());
        let mut saved_t: Vec<Tensor> = Vec::with_capacity(self.tencoder.len());
        let mut lengths: Vec<usize> = Vec::with_capacity(self.encoder.len());
        let mut lengths_t: Vec<usize> = Vec::with_capacity(self.tencoder.len());
        let mut x = x;
        for idx in 0..self.encoder.len() {
            lengths.push(x.dim(3).map_err(Error::Candle)?);            let mut inject: Option<Tensor> = None;
            if idx < self.tencoder.len() {
                lengths_t.push(xt.dim(2).map_err(Error::Candle)?);
                xt = self.tencoder[idx].forward(&xt, None)?;
                if !self.tencoder[idx].empty {
                    saved_t.push(xt.clone());
                } else {
                    inject = Some(xt.clone());
                }
            }
            x = self.encoder[idx].forward(&x, inject.as_ref())?;
            if idx == 0 {
                if let Some(emb) = &self.freq_emb {
                    let (b, c, f, t) = x.dims4().map_err(Error::Candle)?;
                    let emb = emb
                        .reshape((1, c, f, 1))
                        .map_err(Error::Candle)?
                        .broadcast_as((b, c, f, t))
                        .map_err(Error::Candle)?;
                    let emb = (emb * self.freq_emb_scale).map_err(Error::Candle)?;
                    x = (&x + emb).map_err(Error::Candle)?;
                }
            }
            trace_peak(&format!("encoder-{idx}"), &x)?;
            saved.push(x.clone());
        }

        // ---- transformer ---------------------------------------------------
        let (b, c, f, t) = x.dims4().map_err(Error::Candle)?;
        let xf = x
            .reshape((b, c, f * t))
            .map_err(Error::Candle)?;
        let xf = self.upsample.forward(&xf)?;
        let xf = xf.reshape((b, self.transformer_dim, f, t)).map_err(Error::Candle)?;
        trace_peak("tr-in-freq", &xf)?;
        let xtf = self.upsample_t.forward(&xt)?;
        let (xf, xtf) = self.transformer.forward(&xf, &xtf)?;
        trace_peak("tr-out-freq", &xf)?;
        let x = xf.reshape((b, self.transformer_dim, f * t)).map_err(Error::Candle)?;
        let x = self.downsample.forward(&x)?;
        let mut x = x.reshape((b, c, f, t)).map_err(Error::Candle)?;
        xt = self.downsample_t.forward(&xtf)?;

        // ---- decoder -------------------------------------------------------
        for idx in 0..self.decoder.len() {
            let skip = saved.pop().ok_or_else(|| {
                Error::Config(ConfigError::Invalid("missing decoder skip tensor".into()))
            })?;
            let length = lengths.pop().ok_or_else(|| {
                Error::Config(ConfigError::Invalid("missing decoder length".into()))
            })?;
            let (z, pre) = self.decoder[idx].forward(&x, Some(&skip), length)?;
            x = z;
            trace_peak(&format!("decoder-{idx}"), &x)?;
            let offset = self.cfg.depth - self.tdecoder.len();
            if idx >= offset {
                let tdec = &self.tdecoder[idx - offset];
                let length_t = lengths_t.pop().ok_or_else(|| {
                    Error::Config(ConfigError::Invalid("missing time decoder length".into()))
                })?;
                if tdec.empty {
                    let pre_in = pre
                        .narrow(2, 0, 1)
                        .map_err(Error::Candle)?
                        .squeeze(2)
                        .map_err(Error::Candle)?;
                    let (z, _) = tdec.forward(&pre_in, None, length_t)?;
                    xt = z;
                } else {
                    let skip_t = saved_t.pop().ok_or_else(|| {
                        Error::Config(ConfigError::Invalid("missing time skip tensor".into()))
                    })?;
                    let (z, _) = tdec.forward(&xt, Some(&skip_t), length_t)?;
                    xt = z;
                }
            }
        }

        // ---- output --------------------------------------------------------
        let s = SOURCES.len();
        let masks = x
            .reshape((b, s, channels * 2, fq, t))
            .map_err(Error::Candle)?;
        let masks = masks
            .broadcast_mul(&std.reshape((1, 1, 1, 1, 1)).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let masks = masks
            .broadcast_add(&mean.reshape((1, 1, 1, 1, 1)).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let spec_wave = self.ispec(&masks, self.segment_samples())?;
        let time_wave = xt
            .reshape((b, s, channels, len))
            .map_err(Error::Candle)?;
        let time_wave = time_wave
            .broadcast_mul(&tstd.reshape((1, 1, 1, 1)).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let time_wave = time_wave
            .broadcast_add(&tmean.reshape((1, 1, 1, 1)).map_err(Error::Candle)?)
            .map_err(Error::Candle)?;
        let out = spec_wave
            .broadcast_add(&time_wave)
            .map_err(Error::Candle)?
            .contiguous()
            .map_err(Error::Candle)?;
        if std::env::var("DEMUCS_TRACE").is_ok() {
            let peak = |t: &Tensor| -> Result<f32, Error> {
                let v = t
                    .to_device(&Device::Cpu)
                    .map_err(Error::Candle)?
                    .flatten_all()
                    .map_err(Error::Candle)?
                    .to_vec1::<f32>()
                    .map_err(Error::Candle)?;
                Ok(v.iter().fold(0f32, |m, x| m.max(x.abs())))
            };
            eprintln!(
                "[demucs-trace] spec_peak={:.6} time_peak={:.6} out_peak={:.6}",
                peak(&spec_wave)?,
                peak(&time_wave)?,
                peak(&out)?
            );
        }
        Ok(out)
    }

    /// `HTDemucs._spec` for a `(b, ch, len)` waveform: returns the real and
    /// imaginary parts of the CaC magnitude, each `(b, ch, 2048, frames)`.
    fn spec(&self, mix: &Tensor) -> Result<(Tensor, Tensor, usize), Error> {
        let (b, ch, len) = mix.dims3().map_err(Error::Candle)?;
        let data = mix
            .to_device(&Device::Cpu)
            .map_err(Error::Candle)?
            .flatten_all()
            .map_err(Error::Candle)?
            .to_dtype(DType::F32)
            .map_err(Error::Candle)?
            .to_vec1::<f32>()
            .map_err(Error::Candle)?;
        let le = len.div_ceil(HOP);
        let pad = HOP / 2 * 3;
        let pad_right = pad + le * HOP - len;
        let mut re = vec![0f32; b * ch * (NFFT / 2) * le];
        let mut im = vec![0f32; b * ch * (NFFT / 2) * le];
        for bi in 0..b {
            for ci in 0..ch {
                let off = (bi * ch + ci) * len;
                let signal = &data[off..off + len];
                let padded = reflect_pad(signal, pad, pad_right)?;
                let padded = reflect_pad(&padded, NFFT / 2, NFFT / 2)?;
                let frames = (padded.len() - NFFT) / HOP + 1;
                if frames < le + 2 {
                    return Err(Error::Config(ConfigError::Invalid(format!(
                        "spectrogram has {frames} frames, expected at least {}",
                        le + 2
                    ))));
                }
                let window = hann_periodic(NFFT);
                let mut planner = rustfft::FftPlanner::<f32>::new();
                let fft = planner.plan_fft_forward(NFFT);
                let scale = 1.0 / (NFFT as f32).sqrt();
                for fi in 0..le {
                    let start = (fi + 2) * HOP;
                    let mut buf = vec![rustfft::num_complex::Complex::new(0f32, 0f32); NFFT];
                    for i in 0..NFFT {
                        buf[i].re = padded[start + i] * window[i] * scale;
                    }
                    fft.process(&mut buf);
                    for k in 0..NFFT / 2 {
                        let dst = ((bi * ch + ci) * (NFFT / 2) + k) * le + fi;
                        re[dst] = buf[k].re;
                        im[dst] = buf[k].im;
                    }
                }
            }
        }
        let re = Tensor::from_vec(re, (b, ch, NFFT / 2, le), mix.device()).map_err(Error::Candle)?;
        let im = Tensor::from_vec(im, (b, ch, NFFT / 2, le), mix.device()).map_err(Error::Candle)?;
        Ok((re, im, le))
    }

    /// `HTDemucs._ispec` + `_mask` for the de-normalised CaC masks
    /// `(b, sources, channels * 2, 2048, frames)`.
    fn ispec(&self, masks: &Tensor, length: usize) -> Result<Tensor, Error> {
        let (b, s, c2, fq, t) = masks.dims5().map_err(Error::Candle)?;
        if c2 % 2 != 0 {
            return Err(Error::Config(ConfigError::Invalid(
                "CaC masks must have an even channel count".into(),
            )));
        }
        let ch = c2 / 2;
        let data = masks
            .to_device(&Device::Cpu)
            .map_err(Error::Candle)?
            .flatten_all()
            .map_err(Error::Candle)?
            .to_dtype(DType::F32)
            .map_err(Error::Candle)?
            .to_vec1::<f32>()
            .map_err(Error::Candle)?;
        let pad = HOP / 2 * 3;
        let le = HOP * length.div_ceil(HOP) + 2 * pad;
        let mut out = vec![0f32; b * s * ch * length];
        for bi in 0..b {
            for si in 0..s {
                for ci in 0..ch {
                    // (2048, T) re/im; `_ispec` pads one zero frequency bin and
                    // two frames on each side.
                    let frames = t + 4;
                    let bins = fq + 1;
                    let mut re = vec![0f32; bins * frames];
                    let mut im = vec![0f32; bins * frames];
                    for k in 0..fq {
                        for fi in 0..t {
                            let src = (((bi * s + si) * c2 + 2 * ci) * fq + k) * t + fi;
                            let src_i = (((bi * s + si) * c2 + 2 * ci + 1) * fq + k) * t + fi;
                            let dst = k * frames + (fi + 2);
                            re[dst] = data[src];
                            im[dst] = data[src_i];
                        }
                    }
                    let wave = istft_segment(&re, &im, bins, frames, le)?;
                    let start = pad;
                    let dst = ((bi * s + si) * ch + ci) * length;
                    out[dst..dst + length].copy_from_slice(&wave[start..start + length]);
                }
            }
        }
        Tensor::from_vec(out, (b, s, ch, length), masks.device()).map_err(Error::Candle)
    }
}

/// torch `F.pad(x, (left, right), mode="reflect")` on a 1-D slice.
fn reflect_pad(x: &[f32], left: usize, right: usize) -> Result<Vec<f32>, Error> {
    let n = x.len();
    if left >= n || right >= n {
        return Err(Error::Config(ConfigError::Invalid(format!(
            "reflect padding ({left}, {right}) needs an input longer than the pad, got {n}"
        ))));
    }
    let mut out = Vec::with_capacity(n + left + right);
    for i in 0..left {
        out.push(x[left - i]);
    }
    out.extend_from_slice(x);
    for i in 0..right {
        out.push(x[n - 2 - i]);
    }
    Ok(out)
}

fn hann_periodic(n: usize) -> Vec<f32> {
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos()))
        .collect()
}

/// torch `istft(..., n_fft=4096, hop=1024, window=hann, win_length=4096,
/// normalized=True, center=True, length=le)` for one signal.
fn istft_segment(
    re: &[f32],
    im: &[f32],
    bins: usize,
    frames: usize,
    length: usize,
) -> Result<Vec<f32>, Error> {
    let n = NFFT;
    let hop = HOP;
    let expected = (frames - 1) * hop + n;
    let window = hann_periodic(n);
    let mut y = vec![0f32; expected];
    let mut env = vec![0f32; expected];
    let mut planner = rustfft::FftPlanner::<f32>::new();
    let ifft = planner.plan_fft_inverse(n);
    // normalized=True: the inverse transform carries 1/sqrt(n_fft).
    let scale = 1.0 / (n as f32).sqrt();
    let half = n / 2;
    let mut buf = vec![rustfft::num_complex::Complex::new(0f32, 0f32); n];
    for f in 0..frames {
        for k in 0..n {
            buf[k] = rustfft::num_complex::Complex::new(0f32, 0f32);
        }
        for k in 0..bins {
            let v = rustfft::num_complex::Complex::new(re[k * frames + f], im[k * frames + f]);
            buf[k] = v;
            if k > 0 && k < half {
                buf[n - k] = v.conj();
            }
        }
        // irfft ignores the imaginary part of the Nyquist bin.
        buf[half].im = 0.0;
        ifft.process(&mut buf);
        for i in 0..n {
            let t = buf[i].re * scale;
            let w = window[i];
            y[f * hop + i] += t * w;
            env[f * hop + i] += w * w;
        }
    }
    let start = n / 2;
    if start + length > y.len() {
        return Err(Error::Config(ConfigError::Invalid(format!(
            "istft output too short: {} < {}",
            y.len(),
            start + length
        ))));
    }
    let mut out = vec![0f32; length];
    for i in 0..length {
        let e = env[start + i];
        out[i] = if e > 1e-12 { y[start + i] / e } else { 0.0 };
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_has_the_pinned_tensor_count() {
        let cfg = DemucsConfig::default();
        let keys = expected_key_shapes(&cfg);
        assert_eq!(keys.len(), 533, "pinned checkpoint has 533 tensors");
        assert_eq!(cfg.segment_samples(), 343_980);
        // decoder list is stored in reverse creation order
        assert_eq!(
            keys.iter()
                .find(|(n, _)| n == "decoder.3.conv_tr.weight")
                .map(|(_, s)| s.clone()),
            Some(vec![48, 16, 8, 1])
        );
        assert_eq!(
            keys.iter()
                .find(|(n, _)| n == "tdecoder.3.conv_tr.weight")
                .map(|(_, s)| s.clone()),
            Some(vec![48, 8, 8])
        );
        assert_eq!(
            keys.iter()
                .find(|(n, _)| n == "freq_emb.embedding.weight")
                .map(|(_, s)| s.clone()),
            Some(vec![512, 48])
        );
    }

    /// `nn.GroupNorm(groups, channels)` pools the mean *and the variance* over
    /// the whole group, i.e. over every channel of the group plus every trailing
    /// dimension — not per channel. The two are numerically distinct whenever the
    /// channels of a group have different scales, which is exactly the case in
    /// `DConvBlock` and in the transformer's output norm.
    ///
    /// Expected values are `torch.nn.GroupNorm(2, 4, eps=1e-5)` /
    /// `torch.nn.GroupNorm(1, 2, eps=1e-5)` on the same inputs with unit affine
    /// parameters (f32; produced by the pinned reference stack).
    #[test]
    fn group_norm_pools_variance_over_the_whole_group() {
        let dev = Device::Cpu;
        let values: Vec<f32> = (1..=12).map(|v| v as f32).collect();
        let identity_affine = |c: usize| (Tensor::ones(c, DType::F32, &dev).unwrap(), Tensor::zeros(c, DType::F32, &dev).unwrap());

        // 3-D input path: (batch=1, channels=4, time=3), 2 groups of 2 channels.
        let (weight, bias) = identity_affine(4);
        let norm = GroupNorm {
            weight,
            bias,
            groups: 2,
        };
        let x = Tensor::from_vec(values.clone(), (1, 4, 3), &dev).unwrap();
        let got = norm.forward(&x).unwrap().flatten_all().unwrap().to_vec1::<f32>().unwrap();
        let expect_3d = [
            -1.463_847_6f32,
            -0.878_308_65,
            -0.292_769_67,
            0.292_769_43,
            0.878_308_5,
            1.463_847_4,
            -1.463_847_6,
            -0.878_308_8,
            -0.292_769_9,
            0.292_769_43,
            0.878_308_3,
            1.463_847_2,
        ];
        for (i, (a, b)) in got.iter().zip(expect_3d.iter()).enumerate() {
            assert!(
                (a - b).abs() < 1e-5,
                "3-D GroupNorm[{}]: got {} expected {} (per-channel variance would give -2.0701896)",
                i,
                a,
                b
            );
        }

        // 4-D input path: (batch=1, channels=2, freq=2, time=3), one group.
        let (weight, bias) = identity_affine(2);
        let norm = GroupNorm {
            weight,
            bias,
            groups: 1,
        };
        let x = Tensor::from_vec(values, (1, 2, 2, 3), &dev).unwrap();
        let got = norm.forward(&x).unwrap().flatten_all().unwrap().to_vec1::<f32>().unwrap();
        let expect_4d = [
            -1.593_254_2f32,
            -1.303_571_7,
            -1.013_889_1,
            -0.724_206_45,
            -0.434_523_82,
            -0.144_841_31,
            0.144_841_31,
            0.434_523_94,
            0.724_206_6,
            1.013_889_2,
            1.303_571_8,
            1.593_254_2,
        ];
        for (i, (a, b)) in got.iter().zip(expect_4d.iter()).enumerate() {
            assert!(
                (a - b).abs() < 1e-5,
                "4-D GroupNorm[{}]: got {} expected {}",
                i,
                a,
                b
            );
        }
    }

    /// `FreqConvTranspose` must reproduce `nn.ConvTranspose2d(kernel (k,1),
    /// stride (s,1), no padding)` along the frequency axis.
    #[test]
    fn freq_transposed_conv_matches_the_definition() {
        let dev = Device::Cpu;
        let (c_in, c_out, k, s, f, t) = (2usize, 3usize, 4usize, 2usize, 5usize, 2usize);
        let wdata: Vec<f32> = (0..c_in * c_out * k)
            .map(|i| ((i % 7) as f32 - 3.0) * 0.25)
            .collect();
        let bdata: Vec<f32> = (0..c_out).map(|i| i as f32 * 0.5).collect();
        let xdata: Vec<f32> = (0..c_in * f * t).map(|i| (i % 5) as f32 - 2.0).collect();
        let x = Tensor::from_vec(xdata.clone(), (1, c_in, f, t), &dev).unwrap();

        let conv = FreqConvTranspose {
            weight: Tensor::from_vec(wdata.clone(), (c_in, c_out, k), &dev).unwrap(),
            bias: Tensor::from_vec(bdata.clone(), c_out, &dev).unwrap(),
            kernel: k,
            stride: s,
        };
        let got = conv
            .forward(&x)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();

        let f_out = (f - 1) * s + k;
        assert_eq!(got.len(), c_out * f_out * t);
        for co in 0..c_out {
            for o in 0..f_out {
                for ti in 0..t {
                    let mut acc = bdata[co];
                    for ci in 0..c_in {
                        for i in 0..f {
                            for m in 0..k {
                                if i * s + m == o {
                                    let w = wdata[(ci * c_out + co) * k + m];
                                    let xv = xdata[(ci * f + i) * t + ti];
                                    acc += w * xv;
                                }
                            }
                        }
                    }
                    let idx = (co * f_out + o) * t + ti;
                    assert!(
                        (got[idx] - acc).abs() < 1e-5,
                        "conv_tr[{co}][{o}][{ti}]: got {} expected {}",
                        got[idx],
                        acc
                    );
                }
            }
        }
    }

    /// `FreqConv2d` must be a plain `(k,k)` convolution on the `(freq, time)`
    /// plane, which is what the decoder rewrites use.
    #[test]
    fn square_rewrite_conv_matches_the_definition() {
        let dev = Device::Cpu;
        let (c_in, c_out, k, f, t) = (2usize, 3usize, 3usize, 4usize, 3usize);
        let wdata: Vec<f32> = (0..c_out * c_in * k * k)
            .map(|i| ((i % 5) as f32 - 2.0) * 0.1)
            .collect();
        let bdata: Vec<f32> = (0..c_out).map(|i| (i as f32) * 0.25).collect();
        let xdata: Vec<f32> = (0..c_in * f * t).map(|i| (i % 6) as f32 - 2.5).collect();
        let x = Tensor::from_vec(xdata.clone(), (1, c_in, f, t), &dev).unwrap();
        let conv = FreqConv2d::load(
            c_in,
            c_out,
            k,
            1,
            VarBuilder::from_tensors(
                [
                    (
                        "weight".to_string(),
                        Tensor::from_vec(wdata.clone(), (c_out, c_in, k, k), &dev).unwrap(),
                    ),
                    ("bias".to_string(), Tensor::from_vec(bdata.clone(), c_out, &dev).unwrap()),
                ]
                .into_iter()
                .collect(),
                DType::F32,
                &dev,
            ),
        )
        .unwrap();
        let got = conv
            .forward(&x)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();

        let pad = 1usize;
        for co in 0..c_out {
            for fi in 0..f {
                for ti in 0..t {
                    let mut acc = bdata[co];
                    for ci in 0..c_in {
                        for kf in 0..k {
                            for kt in 0..k {
                                let src_f = fi as isize + kf as isize - pad as isize;
                                let src_t = ti as isize + kt as isize - pad as isize;
                                if src_f < 0 || src_t < 0 || src_f >= f as isize || src_t >= t as isize
                                {
                                    continue;
                                }
                                let w = wdata[((co * c_in + ci) * k + kf) * k + kt];
                                let xv = xdata[(ci * f + src_f as usize) * t + src_t as usize];
                                acc += w * xv;
                            }
                        }
                    }
                    let idx = (co * f + fi) * t + ti;
                    assert!(
                        (got[idx] - acc).abs() < 1e-5,
                        "conv2d[{co}][{fi}][{ti}]: got {} expected {}",
                        got[idx],
                        acc
                    );
                }
            }
        }
    }

    #[test]
    fn sin_embeddings_match_torch_formula() {
        let dev = Device::Cpu;
        let e = sin_1d_embedding(3, 8, 10000.0, &dev).unwrap();
        let v = e.flatten_all().unwrap().to_vec1::<f32>().unwrap();
        // t=0 -> cos(0)=1 .. sin(0)=0
        for k in 0..4 {
            assert!((v[k] - 1.0).abs() < 1e-6);
        }
        for k in 4..8 {
            assert!(v[k].abs() < 1e-6);
        }
        let e2 = sin_2d_embedding(8, 2, 3, 10000.0, &dev).unwrap();
        assert_eq!(e2.dims(), &[1, 6, 8]);
    }
}


