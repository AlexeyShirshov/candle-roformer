//! VR Arch (UVR) network: `CascadedASPPNet`.
//!
//! Port of the UVR `nets`/`layers` modules used by `audio-separator` for
//! `*.pth` VR models. The checkpoint is converted offline to safetensors by
//! `tools/convert_vr_arch.py`; this module never reads a pickle.
//!
//! Only the high-precision architectures (`123812`, `123821`) are implemented;
//! other architecture ids are rejected instead of being guessed.

use candle_core::{Module, Result, Tensor, D};
use candle_nn::{batch_norm, conv2d_no_bias, BatchNorm, Conv2d, Conv2dConfig, ModuleT, VarBuilder};

/// Batch-norm epsilon used by the UVR reference implementation (PyTorch default).
const BN_EPS: f64 = 1e-5;
/// LeakyReLU negative slope used by the UVR encoders (PyTorch default).
const LEAKY_SLOPE: f64 = 0.01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Activ {
    Relu,
    LeakyRelu,
}

fn activate(x: &Tensor, activ: Activ) -> Result<Tensor> {
    match activ {
        Activ::Relu => x.relu(),
        Activ::LeakyRelu => {
            let zero = Tensor::zeros_like(x)?;
            let positive = x.maximum(&zero)?;
            let negative = x.minimum(&zero)?.affine(LEAKY_SLOPE, 0.0)?;
            positive.add(&negative)
        }
    }
}

/// ``Conv2d`` - no bias - ``BatchNorm2d`` - activation, the UVR building block.
#[derive(Debug)]
struct ConvBn {
    conv: Conv2d,
    bn: BatchNorm,
    activ: Activ,
}

impl ConvBn {
    #[allow(clippy::too_many_arguments)]
    fn load(
        vb: VarBuilder,
        nin: usize,
        nout: usize,
        k: usize,
        stride: usize,
        pad: usize,
        dilation: usize,
        activ: Activ,
    ) -> Result<Self> {
        let cfg = Conv2dConfig {
            padding: pad,
            stride,
            dilation,
            groups: 1,
            cudnn_fwd_algo: None,
        };
        let conv = conv2d_no_bias(nin, nout, k, cfg, vb.pp("conv").pp("0"))?;
        let bn = batch_norm(nout, BN_EPS, vb.pp("conv").pp("1"))?;
        Ok(Self { conv, bn, activ })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let y = self.conv.forward(x)?;
        let y = self.bn.forward_t(&y, false)?;
        activate(&y, self.activ)
    }
}

/// Depthwise separable convolution followed by batch norm and an activation.
#[derive(Debug)]
struct SeparableConvBn {
    depthwise: Conv2d,
    pointwise: Conv2d,
    bn: BatchNorm,
    activ: Activ,
}

impl SeparableConvBn {
    fn load(
        vb: VarBuilder,
        nin: usize,
        nout: usize,
        k: usize,
        pad: usize,
        dilation: usize,
        activ: Activ,
    ) -> Result<Self> {
        let depthwise = conv2d_no_bias(
            nin,
            nin,
            k,
            Conv2dConfig {
                padding: pad,
                stride: 1,
                dilation,
                groups: nin,
                cudnn_fwd_algo: None,
            },
            vb.pp("conv").pp("0"),
        )?;
        let pointwise = conv2d_no_bias(
            nin,
            nout,
            1,
            Conv2dConfig {
                padding: 0,
                stride: 1,
                dilation: 1,
                groups: 1,
                cudnn_fwd_algo: None,
            },
            vb.pp("conv").pp("1"),
        )?;
        let bn = batch_norm(nout, BN_EPS, vb.pp("conv").pp("2"))?;
        Ok(Self {
            depthwise,
            pointwise,
            bn,
            activ,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let y = self.depthwise.forward(x)?;
        let y = self.pointwise.forward(&y)?;
        let y = self.bn.forward_t(&y, false)?;
        activate(&y, self.activ)
    }
}

fn crop_center(x: &Tensor, h: usize, w: usize) -> Result<Tensor> {
    let (n, c, xh, xw) = x.dims4()?;
    if xh < h || xw < w {
        candle_core::bail!(
            "h1_shape[3] must be greater than h2_shape[3]: skip {xh}x{xw}, target {h}x{w}"
        );
    }
    let top = (xh - h) / 2;
    let left = (xw - w) / 2;
    let x = x.narrow(2, top, h)?;
    let x = x.narrow(3, left, w)?;
    debug_assert_eq!(x.dims(), &[n, c, h, w]);
    Ok(x)
}

/// Encoder: two ``ConvBn`` layers; returns ``(hidden, skip)``.
#[derive(Debug)]
struct Encoder {
    conv1: ConvBn,
    conv2: ConvBn,
}

impl Encoder {
    fn load(
        vb: VarBuilder,
        nin: usize,
        nout: usize,
        k: usize,
        stride: usize,
        pad: usize,
    ) -> Result<Self> {
        let conv1 = ConvBn::load(vb.pp("conv1"), nin, nout, k, 1, pad, 1, Activ::LeakyRelu)?;
        let conv2 = ConvBn::load(
            vb.pp("conv2"),
            nout,
            nout,
            k,
            stride,
            pad,
            1,
            Activ::LeakyRelu,
        )?;
        Ok(Self { conv1, conv2 })
    }

    fn forward(&self, x: &Tensor) -> Result<(Tensor, Tensor)> {
        let skip = self.conv1.forward(x)?;
        let hidden = self.conv2.forward(&skip)?;
        Ok((hidden, skip))
    }
}

/// Decoder: bilinear x2 upsample, centre-crop the skip connection, concat, conv.
#[derive(Debug)]
struct Decoder {
    conv: ConvBn,
}

impl Decoder {
    fn load(vb: VarBuilder, nin: usize, nout: usize, k: usize, pad: usize) -> Result<Self> {
        let conv = ConvBn::load(vb.pp("conv"), nin, nout, k, 1, pad, 1, Activ::Relu)?;
        Ok(Self { conv })
    }

    fn forward(&self, x: &Tensor, skip: Option<&Tensor>) -> Result<Tensor> {
        let (_, _, h, w) = x.dims4()?;
        let mut out = x.upsample_bilinear2d(h * 2, w * 2, true)?;
        if let Some(skip) = skip {
            let (_, _, sh, sw) = out.dims4()?;
            let cropped = crop_center(skip, sh, sw)?;
            out = Tensor::cat(&[&out, &cropped], 1)?;
        }
        self.conv.forward(&out)
    }
}

/// ASPP module (five branches for the high-precision architectures).
#[derive(Debug)]
struct Aspp {
    conv1: ConvBn,
    conv2: ConvBn,
    conv3: SeparableConvBn,
    conv4: SeparableConvBn,
    conv5: SeparableConvBn,
    bottleneck: ConvBn,
}

impl Aspp {
    fn load(vb: VarBuilder, nin: usize, nout: usize) -> Result<Self> {
        let conv1 = ConvBn::load(vb.pp("conv1").pp("1"), nin, nin, 1, 1, 0, 1, Activ::Relu)?;
        let conv2 = ConvBn::load(vb.pp("conv2"), nin, nin, 1, 1, 0, 1, Activ::Relu)?;
        let conv3 = SeparableConvBn::load(vb.pp("conv3"), nin, nin, 3, 4, 4, Activ::Relu)?;
        let conv4 = SeparableConvBn::load(vb.pp("conv4"), nin, nin, 3, 8, 8, Activ::Relu)?;
        let conv5 = SeparableConvBn::load(vb.pp("conv5"), nin, nin, 3, 16, 16, Activ::Relu)?;
        let bottleneck = ConvBn::load(
            vb.pp("bottleneck").pp("0"),
            nin * 5,
            nout,
            1,
            1,
            0,
            1,
            Activ::Relu,
        )?;
        Ok(Self {
            conv1,
            conv2,
            conv3,
            conv4,
            conv5,
            bottleneck,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (_, _, h, w) = x.dims4()?;
        // AdaptiveAvgPool2d((1, None)) -> pool over the frequency axis only.
        let pooled = x.mean(D::Minus2)?.unsqueeze(D::Minus2)?;
        let feat1 = self
            .conv1
            .forward(&pooled)?
            .upsample_bilinear2d(h, w, true)?;
        let feat2 = self.conv2.forward(x)?;
        let feat3 = self.conv3.forward(x)?;
        let feat4 = self.conv4.forward(x)?;
        let feat5 = self.conv5.forward(x)?;
        let out = Tensor::cat(&[&feat1, &feat2, &feat3, &feat4, &feat5], 1)?;
        self.bottleneck.forward(&out)
    }
}

/// One stage of the cascade.
#[derive(Debug)]
struct BaseAsppNet {
    enc1: Encoder,
    enc2: Encoder,
    enc3: Encoder,
    enc4: Encoder,
    aspp: Aspp,
    dec4: Decoder,
    dec3: Decoder,
    dec2: Decoder,
    dec1: Decoder,
}

impl BaseAsppNet {
    fn load(vb: VarBuilder, nin: usize, ch: usize) -> Result<Self> {
        let enc1 = Encoder::load(vb.pp("enc1"), nin, ch, 3, 2, 1)?;
        let enc2 = Encoder::load(vb.pp("enc2"), ch, ch * 2, 3, 2, 1)?;
        let enc3 = Encoder::load(vb.pp("enc3"), ch * 2, ch * 4, 3, 2, 1)?;
        let enc4 = Encoder::load(vb.pp("enc4"), ch * 4, ch * 8, 3, 2, 1)?;
        let aspp = Aspp::load(vb.pp("aspp"), ch * 8, ch * 16)?;
        let dec4 = Decoder::load(vb.pp("dec4"), ch * 8 + ch * 16, ch * 8, 3, 1)?;
        let dec3 = Decoder::load(vb.pp("dec3"), ch * 4 + ch * 8, ch * 4, 3, 1)?;
        let dec2 = Decoder::load(vb.pp("dec2"), ch * 2 + ch * 4, ch * 2, 3, 1)?;
        let dec1 = Decoder::load(vb.pp("dec1"), ch + ch * 2, ch, 3, 1)?;
        Ok(Self {
            enc1,
            enc2,
            enc3,
            enc4,
            aspp,
            dec4,
            dec3,
            dec2,
            dec1,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (h1, s1) = self.enc1.forward(x)?;
        let (h2, s2) = self.enc2.forward(&h1)?;
        let (h3, s3) = self.enc3.forward(&h2)?;
        let (h4, s4) = self.enc4.forward(&h3)?;
        let h = self.aspp.forward(&h4)?;
        let h = self.dec4.forward(&h, Some(&s4))?;
        let h = self.dec3.forward(&h, Some(&s3))?;
        let h = self.dec2.forward(&h, Some(&s2))?;
        self.dec1.forward(&h, Some(&s1))
    }
}

/// Model capacities for the supported architecture ids (UVR `determine_model_capacity`).
#[derive(Debug, Clone, Copy)]
struct Capacity {
    stg1_low: (usize, usize),
    stg1_high: (usize, usize),
    stg2_bridge: (usize, usize),
    stg2_full: (usize, usize),
    stg3_bridge: (usize, usize),
    stg3_full: (usize, usize),
    out: (usize, usize),
}

fn capacity_for(nn_arch: usize) -> Result<Capacity> {
    match nn_arch {
        // hp_model_arch
        123821 | 123812 => Ok(Capacity {
            stg1_low: (2, 32),
            stg1_high: (2, 32),
            stg2_bridge: (34, 16),
            stg2_full: (16, 32),
            stg3_bridge: (66, 32),
            stg3_full: (32, 64),
            out: (64, 2),
        }),
        other => {
            candle_core::bail!("unsupported VR architecture id {other}; supported: 123812, 123821")
        }
    }
}

/// `CascadedASPPNet`: the UVR network that predicts a soft mask from a magnitude
/// spectrogram patch of shape `(batch, channels, bins, frames)`.
#[derive(Debug)]
pub struct CascadedAsppNet {
    stg1_low: BaseAsppNet,
    stg1_high: BaseAsppNet,
    stg2_bridge: ConvBn,
    stg2_full: BaseAsppNet,
    stg3_bridge: ConvBn,
    stg3_full: BaseAsppNet,
    out: Conv2d,
    max_bin: usize,
    output_bin: usize,
    offset: usize,
}

impl CascadedAsppNet {
    /// `n_fft_bins` is `bins * 2` from the model parameter file.
    pub fn load(vb: VarBuilder, n_fft_bins: usize, nn_arch: usize) -> Result<Self> {
        let cap = capacity_for(nn_arch)?;
        let stg1_low =
            BaseAsppNet::load(vb.pp("stg1_low_band_net"), cap.stg1_low.0, cap.stg1_low.1)?;
        let stg1_high = BaseAsppNet::load(
            vb.pp("stg1_high_band_net"),
            cap.stg1_high.0,
            cap.stg1_high.1,
        )?;
        let stg2_bridge = ConvBn::load(
            vb.pp("stg2_bridge"),
            cap.stg2_bridge.0,
            cap.stg2_bridge.1,
            1,
            1,
            0,
            1,
            Activ::Relu,
        )?;
        let stg2_full = BaseAsppNet::load(
            vb.pp("stg2_full_band_net"),
            cap.stg2_full.0,
            cap.stg2_full.1,
        )?;
        let stg3_bridge = ConvBn::load(
            vb.pp("stg3_bridge"),
            cap.stg3_bridge.0,
            cap.stg3_bridge.1,
            1,
            1,
            0,
            1,
            Activ::Relu,
        )?;
        let stg3_full = BaseAsppNet::load(
            vb.pp("stg3_full_band_net"),
            cap.stg3_full.0,
            cap.stg3_full.1,
        )?;
        let out = conv2d_no_bias(
            cap.out.0,
            cap.out.1,
            1,
            Conv2dConfig {
                padding: 0,
                stride: 1,
                dilation: 1,
                groups: 1,
                cudnn_fwd_algo: None,
            },
            vb.pp("out"),
        )?;
        Ok(Self {
            stg1_low,
            stg1_high,
            stg2_bridge,
            stg2_full,
            stg3_bridge,
            stg3_full,
            out,
            max_bin: n_fft_bins / 2,
            output_bin: n_fft_bins / 2 + 1,
            offset: 128,
        })
    }

    /// Frames of context trimmed from each side by `predict_mask`.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Number of frequency bins the network consumes.
    pub fn max_bin(&self) -> usize {
        self.max_bin
    }

    /// Raw network forward pass: returns the un-padded mask.
    pub fn forward(&self, input: &Tensor) -> Result<Tensor> {
        let x = input.narrow(2, 0, self.max_bin)?;
        let bandwidth = self.max_bin / 2;
        let low = self.stg1_low.forward(&x.narrow(2, 0, bandwidth)?)?;
        let high = self
            .stg1_high
            .forward(&x.narrow(2, bandwidth, self.max_bin - bandwidth)?)?;
        let aux1 = Tensor::cat(&[&low, &high], 2)?;

        let hidden = Tensor::cat(&[input, &aux1], 1)?;
        let aux2 = self
            .stg2_full
            .forward(&self.stg2_bridge.forward(&hidden)?)?;

        let hidden = Tensor::cat(&[input, &aux1, &aux2], 1)?;
        let hidden = self
            .stg3_full
            .forward(&self.stg3_bridge.forward(&hidden)?)?;
        candle_nn::ops::sigmoid(&self.out.forward(&hidden)?)
    }

    /// Mask prediction matching `nets.CascadedASPPNet.predict_mask`.
    pub fn predict_mask(&self, input: &Tensor) -> Result<Tensor> {
        let mask = self.forward(input)?;
        let (_, _, h, _) = mask.dims4()?;
        let mask = if self.output_bin > h {
            let last = mask.narrow(2, h - 1, 1)?;
            let repeats = self.output_bin - h;
            let mut parts: Vec<Tensor> = Vec::with_capacity(repeats + 1);
            parts.push(mask.clone());
            for _ in 0..repeats {
                parts.push(last.clone());
            }
            let refs: Vec<&Tensor> = parts.iter().collect();
            Tensor::cat(&refs, 2)?
        } else {
            mask
        };
        if self.offset > 0 {
            let w = mask.dim(3)?;
            mask.narrow(3, self.offset, w - 2 * self.offset)
        } else {
            Ok(mask)
        }
    }
}
