use candle_core::{DType, Device, Module, Tensor, D};
use candle_nn::{linear, linear_no_bias, Linear, VarBuilder};

use crate::config::{ModelFamily, SeparatorConfig};
use crate::error::Error;
use crate::mel::mel_bin_membership;

pub mod bs_roformer;

pub use bs_roformer::BSRoformer;

// ---------- RMSNorm ----------

#[derive(Debug, Clone)]
pub struct RmsNorm {
    gamma: Tensor,
    scale: f64,
}

impl RmsNorm {
    pub fn new(dim: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        let gamma = vb.get(dim, "gamma")?;
        let scale = (dim as f64).sqrt();
        Ok(Self { gamma, scale })
    }
}

impl Module for RmsNorm {
    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let sq = x.sqr()?.sum_keepdim(D::Minus1)?;
        let denom = (sq + 1e-12)?.sqrt()?;
        let normed = x.broadcast_div(&denom)?;
        normed
            .broadcast_mul(&Tensor::new(self.scale as f32, x.device())?)?
            .broadcast_mul(&self.gamma)
    }
}

// ---------- RotaryEmbedding ----------

#[derive(Debug, Clone)]
pub struct RotaryEmbedding {
    freqs: Tensor,
}

impl RotaryEmbedding {
    pub fn new(dim: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        // dim = dim_head; freqs length = dim/2
        let freqs = vb.get(dim / 2, "freqs")?;
        Ok(Self { freqs })
    }

    pub fn rotate(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        // x: (..., seq, dim) or (b,h,seq,dim)
        let seq_len = x.dim(D::Minus2)?;
        let device = x.device();
        let dtype = x.dtype();
        let pos = Tensor::arange(0i64, seq_len as i64, device)?.to_dtype(DType::F32)?;
        let f = self.freqs.to_dtype(DType::F32)?;
        // angles: (seq, dim/2), then interleaved to (seq, dim) — rotary_embedding_torch
        // uses repeat(..., 'n -> (n r)', r=2), pairing with its interleaved rotate_half.
        let angles = pos.unsqueeze(1)?.broadcast_mul(&f.unsqueeze(0)?)?;
        let half = angles.dim(D::Minus1)?;
        let angles = angles
            .unsqueeze(D::Minus1)?
            .broadcast_as((seq_len, half, 2))?
            .reshape((seq_len, half * 2))?;
        let cos = angles.cos()?.unsqueeze(0)?.unsqueeze(0)?; // (1,1,seq,dim)
        let sin = angles.sin()?.unsqueeze(0)?.unsqueeze(0)?;
        let rot = rotate_half(x)?;
        let x = x.to_dtype(DType::F32)?;
        let out = (x.broadcast_mul(&cos)? + rot.broadcast_mul(&sin)?)?;
        out.to_dtype(dtype)
    }
}

fn rotate_half(x: &Tensor) -> candle_core::Result<Tensor> {
    let dims = x.dims().to_vec();
    let last = *dims.last().unwrap();
    let half = last / 2;
    let x = x.contiguous()?;
    let new_dims: Vec<usize> = dims[..dims.len() - 1]
        .iter()
        .copied()
        .chain(std::iter::once(half))
        .chain(std::iter::once(2))
        .collect();
    let x = x.reshape(new_dims)?;
    let x1 = x.narrow(D::Minus1, 0, 1)?;
    let x2 = x.narrow(D::Minus1, 1, 1)?;
    let neg_x2 = x2.neg()?;
    let stacked = Tensor::cat(&[&neg_x2, &x1], D::Minus1)?;
    stacked.reshape(dims)
}

// ---------- Attention ----------

#[derive(Debug, Clone)]
pub struct Attention {
    heads: usize,
    dim_head: usize,
    scale: f64,
    norm: RmsNorm,
    to_qkv: Linear,
    to_gates: Linear,
    to_out: Linear,
    rotary: RotaryEmbedding,
}

impl Attention {
    pub fn new(
        dim: usize,
        heads: usize,
        dim_head: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let rotary = RotaryEmbedding::new(dim_head, vb.pp("rotary_embed"))?;
        Self::new_with_rotary(dim, heads, dim_head, rotary, vb)
    }

    /// Build an attention block with an externally supplied rotary table. The
    /// BS-RoFormer checkpoints store no rotary buffer, so the table is derived
    /// from `dim_head`/`theta` instead of loaded.
    pub fn new_with_rotary(
        dim: usize,
        heads: usize,
        dim_head: usize,
        rotary: RotaryEmbedding,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let dim_inner = heads * dim_head;
        let norm = RmsNorm::new(dim, vb.pp("norm"))?;
        let to_qkv = linear_no_bias(dim, dim_inner * 3, vb.pp("to_qkv"))?;
        let to_gates = linear(dim, heads, vb.pp("to_gates"))?;
        let to_out = linear_no_bias(dim_inner, dim, vb.pp("to_out").pp("0"))?;
        let scale = (dim_head as f64).powf(-0.5);
        Ok(Self {
            heads,
            dim_head,
            scale,
            norm,
            to_qkv,
            to_gates,
            to_out,
            rotary,
        })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        // x: (b, seq, dim)
        let (b, seq, _dim) = x.dims3()?;
        let x_norm = self.norm.forward(x)?;
        let qkv = self.to_qkv.forward(&x_norm)?; // (b, seq, 3*inner)
        let inner = self.heads * self.dim_head;
        let qkv = qkv.reshape((b, seq, 3, self.heads, self.dim_head))?;
        let qkv = qkv.permute((2, 0, 3, 1, 4))?; // (3, b, h, seq, d)
        let q = qkv.narrow(0, 0, 1)?.squeeze(0)?;
        let k = qkv.narrow(0, 1, 1)?.squeeze(0)?;
        let v = qkv.narrow(0, 2, 1)?.squeeze(0)?;

        let q = self.rotary.rotate(&q)?.contiguous()?;
        let k = self.rotary.rotate(&k)?.contiguous()?;
        let v = v.contiguous()?;

        // attention, blocked over the query dimension to keep peak memory bounded
        // (60 bands x 8 heads x 801 x 801 would be ~1.2 GB per layer otherwise).
        let scale_t = Tensor::new(self.scale as f32, x.device())?;
        let kt = k.transpose(2, 3)?;
        const ATTN_BLOCK: usize = 128;
        let mut blocks = Vec::with_capacity(seq.div_ceil(ATTN_BLOCK));
        let mut start = 0usize;
        while start < seq {
            let len = ATTN_BLOCK.min(seq - start);
            let q_blk = q.narrow(2, start, len)?;
            let scores = q_blk.matmul(&kt)?; // (b,h,len,seq)
            let scores = scores.broadcast_mul(&scale_t)?;
            let attn = candle_nn::ops::softmax(&scores, D::Minus1)?;
            blocks.push(attn.matmul(&v)?); // (b,h,len,d)
            start += len;
        }
        let out = Tensor::cat(&blocks, 2)?;

        // gates
        let gates = self.to_gates.forward(&x_norm)?; // (b, seq, heads)
        let gates = gates.permute((0, 2, 1))?; // (b, heads, seq)
        let gates = gates.unsqueeze(D::Minus1)?; // (b,h,seq,1)
        let gates = candle_nn::ops::sigmoid(&gates)?;
        let out = out.broadcast_mul(&gates)?;

        let out = out.permute((0, 2, 1, 3))?; // (b, seq, h, d)
        let out = out.reshape((b, seq, inner))?;
        self.to_out.forward(&out)
    }
}

// ---------- FeedForward ----------

#[derive(Debug, Clone)]
pub struct FeedForward {
    net0: RmsNorm,
    net1: Linear,
    net4: Linear,
    gelu_erf: bool,
}

impl FeedForward {
    pub fn new(dim: usize, mult: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        Self::new_with_gelu(dim, mult, false, vb)
    }

    /// Feed-forward with the exact `erf` GELU (`nn.GELU()` default in
    /// PyTorch) instead of Candle's tanh approximation. The reference
    /// BS-RoFormer uses `nn.GELU()`.
    pub fn new_erf(dim: usize, mult: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        Self::new_with_gelu(dim, mult, true, vb)
    }

    fn new_with_gelu(
        dim: usize,
        mult: usize,
        gelu_erf: bool,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let dim_inner = dim * mult;
        let net0 = RmsNorm::new(dim, vb.pp("net").pp("0"))?;
        let net1 = linear(dim, dim_inner, vb.pp("net").pp("1"))?;
        let net4 = linear(dim_inner, dim, vb.pp("net").pp("4"))?;
        Ok(Self {
            net0,
            net1,
            net4,
            gelu_erf,
        })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let h = self.net0.forward(x)?;
        let h = self.net1.forward(&h)?;
        let h = if self.gelu_erf {
            h.gelu_erf()?
        } else {
            h.gelu()?
        };
        self.net4.forward(&h)
    }
}

// ---------- Transformer ----------

#[derive(Debug, Clone)]
pub struct Transformer {
    layers: Vec<(Attention, FeedForward)>,
    norm: Option<RmsNorm>,
}

impl Transformer {
    pub fn new(
        dim: usize,
        depth: usize,
        heads: usize,
        dim_head: usize,
        ff_mult: usize,
        norm_output: bool,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let mut layers = Vec::with_capacity(depth);
        for i in 0..depth {
            let vb_layer = vb.pp("layers").pp(i.to_string());
            let attn = Attention::new(dim, heads, dim_head, vb_layer.pp("0"))?;
            let ff = FeedForward::new(dim, ff_mult, vb_layer.pp("1"))?;
            layers.push((attn, ff));
        }
        let norm = if norm_output {
            Some(RmsNorm::new(dim, vb.pp("norm"))?)
        } else {
            None
        };
        Ok(Self { layers, norm })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let mut x = x.clone();
        for (attn, ff) in &self.layers {
            let a = attn.forward(&x)?;
            x = (x + a)?;
            let f = ff.forward(&x)?;
            x = (x + f)?;
        }
        if let Some(norm) = &self.norm {
            x = norm.forward(&x)?;
        }
        Ok(x)
    }
}

// ---------- BandSplit ----------

#[derive(Debug, Clone)]
pub struct BandSplit {
    to_features: Vec<(usize, RmsNorm, Linear)>, // (dim_in, norm, linear)
}

impl BandSplit {
    pub fn new(dim: usize, dim_inputs: Vec<usize>, vb: VarBuilder) -> candle_core::Result<Self> {
        let mut to_features = Vec::with_capacity(dim_inputs.len());
        for (i, &dim_in) in dim_inputs.iter().enumerate() {
            let vb_i = vb.pp("to_features").pp(i.to_string());
            let norm = RmsNorm::new(dim_in, vb_i.pp("0"))?;
            let linear = linear(dim_in, dim, vb_i.pp("1"))?;
            to_features.push((dim_in, norm, linear));
        }
        let _ = dim;
        Ok(Self { to_features })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        // x: (b, t, total)
        let mut outs: Vec<Tensor> = Vec::new();
        let mut offset = 0usize;
        for (dim_in, norm, lin) in &self.to_features {
            let chunk = x.narrow(D::Minus1, offset, *dim_in)?;
            offset += *dim_in;
            let h = norm.forward(&chunk)?;
            let h = lin.forward(&h)?;
            outs.push(h);
        }
        // stack along dim=-2 -> (b, t, num_bands, dim)
        let mut unsq: Vec<Tensor> = Vec::new();
        for o in &outs {
            unsq.push(o.unsqueeze(2)?);
        }
        let out = Tensor::cat(&unsq, 2)?;
        Ok(out)
    }
}

// ---------- MLP + MaskEstimator ----------

#[derive(Debug, Clone)]
pub struct Mlp {
    linears: Vec<Linear>,
}

impl Mlp {
    pub fn new(
        dim_in: usize,
        dim_out: usize,
        dim_hidden: usize,
        depth: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let mut dims = vec![dim_in];
        for _ in 0..depth {
            dims.push(dim_hidden);
        }
        dims.push(dim_out);
        let mut linears = Vec::new();
        for (idx, w) in dims.windows(2).enumerate() {
            let li = w[0];
            let lo = w[1];
            // Upstream MLP interleaves activations: Linear, Tanh, Linear, Tanh, Linear
            // => state_dict indices 0, 2, 4 (not 0, 1, 2).
            let lin = linear(li, lo, vb.pp((idx * 2).to_string()))?;
            linears.push(lin);
        }
        Ok(Self { linears })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let n = self.linears.len();
        let mut h = x.clone();
        for (i, lin) in self.linears.iter().enumerate() {
            h = lin.forward(&h)?;
            if i < n - 1 {
                h = h.tanh()?;
            }
        }
        Ok(h)
    }
}

#[derive(Debug, Clone)]
pub struct MaskEstimator {
    to_freqs: Vec<Mlp>,
    dim_inputs: Vec<usize>,
}

impl MaskEstimator {
    pub fn new(
        dim: usize,
        dim_inputs: Vec<usize>,
        depth: usize,
        mlp_expansion_factor: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let dim_hidden = dim * mlp_expansion_factor;
        let mut to_freqs = Vec::with_capacity(dim_inputs.len());
        for (i, &dim_in) in dim_inputs.iter().enumerate() {
            let vb_i = vb.pp("to_freqs").pp(i.to_string());
            // MLP(dim, dim_in*2, hidden, depth)
            let mlp = Mlp::new(dim, dim_in * 2, dim_hidden, depth, vb_i.pp("0"))?;
            to_freqs.push(mlp);
        }
        Ok(Self {
            to_freqs,
            dim_inputs,
        })
    }

    pub fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        // x: (b, t, bands, dim)
        // unbind bands -> for each band features (b, t, dim)
        let bands = x.dim(2)?;
        let mut outs = Vec::with_capacity(bands);
        for i in 0..bands {
            let band_x = x.narrow(2, i, 1)?.squeeze(2)?; // (b,t,dim)
            let mlp = &self.to_freqs[i];
            let out = mlp.forward(&band_x)?; // (b,t,dim_in*2)
                                             // GLU over last dim
            let half = out.dim(D::Minus1)? / 2;
            let a = out.narrow(D::Minus1, 0, half)?;
            let b = out.narrow(D::Minus1, half, half)?;
            let sig = candle_nn::ops::sigmoid(&b)?;
            let glu = a.broadcast_mul(&sig)?;
            outs.push(glu);
        }
        // cat over last dim -> (b, t, total_selected)
        let mut unsq = Vec::new();
        for o in &outs {
            unsq.push(o.unsqueeze(2)?);
        }
        // cat along last dim instead
        let mut cat_list = Vec::new();
        for o in &outs {
            cat_list.push(o.clone());
        }
        let out = Tensor::cat(&cat_list, D::Minus1)?;
        Ok(out)
    }
}

// ---------- MelBandRoformer ----------

pub struct MelBandRoformer {
    pub stereo: bool,
    pub audio_channels: usize,
    pub num_stems: usize,
    pub zero_dc: bool,
    pub layers: Vec<Vec<Transformer>>,
    pub band_split: BandSplit,
    pub mask_estimators: Vec<MaskEstimator>,
    pub freq_indices: Vec<usize>,
    pub num_bands_per_freq: Vec<usize>,
    pub freqs: usize,
    // stft params
    pub n_fft: usize,
    pub hop: usize,
    pub win: usize,
    pub device: candle_core::Device,
}

/// Expected parameter names and shapes for a config, derived from the same
/// structure the builder uses. Lets the loader validate a checkpoint without
/// materialising a dummy model (which would cost ~0.9 GB for a real profile).
pub fn expected_key_shapes(cfg: &SeparatorConfig) -> Vec<(String, Vec<usize>)> {
    match cfg.family() {
        ModelFamily::MelBandRoformer => expected_key_shapes_melband(cfg),
        ModelFamily::BSRoformer => bs_roformer::expected_key_shapes(cfg),
        // MDX-Net is an ONNX graph with no safetensors key schema; it is loaded
        // by `crate::mdx::MdxSeparator`.
        ModelFamily::MdxNet => Vec::new(),
    }
}

/// Expected key set of a MelBandRoformer profile (mel-band membership).
pub fn expected_key_shapes_melband(cfg: &SeparatorConfig) -> Vec<(String, Vec<usize>)> {
    let dim = cfg.dim;
    let audio_channels = if cfg.stereo { 2 } else { 1 };
    let membership = mel_bin_membership(cfg.stft_n_fft, cfg.sample_rate, cfg.num_bands);
    let ff_mult = cfg.mlp_expansion_factor;
    let inner = cfg.heads * cfg.dim_head;
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();

    for (i, bins) in membership.iter().enumerate() {
        let dim_in = 2 * bins.len() * audio_channels;
        out.push((format!("band_split.to_features.{i}.0.gamma"), vec![dim_in]));
        out.push((
            format!("band_split.to_features.{i}.1.weight"),
            vec![dim, dim_in],
        ));
        out.push((format!("band_split.to_features.{i}.1.bias"), vec![dim]));
    }

    let push_transformer = |out: &mut Vec<(String, Vec<usize>)>, prefix: &str, depth: usize| {
        for d in 0..depth {
            let p = format!("{prefix}.layers.{d}");
            out.push((format!("{p}.0.norm.gamma"), vec![dim]));
            out.push((format!("{p}.0.to_qkv.weight"), vec![3 * inner, dim]));
            out.push((format!("{p}.0.to_gates.weight"), vec![cfg.heads, dim]));
            out.push((format!("{p}.0.to_gates.bias"), vec![cfg.heads]));
            out.push((format!("{p}.0.to_out.0.weight"), vec![dim, inner]));
            out.push((format!("{p}.0.rotary_embed.freqs"), vec![cfg.dim_head / 2]));
            out.push((format!("{p}.1.net.0.gamma"), vec![dim]));
            out.push((format!("{p}.1.net.1.weight"), vec![dim * ff_mult, dim]));
            out.push((format!("{p}.1.net.1.bias"), vec![dim * ff_mult]));
            out.push((format!("{p}.1.net.4.weight"), vec![dim, dim * ff_mult]));
            out.push((format!("{p}.1.net.4.bias"), vec![dim]));
        }
        if cfg.norm_output {
            out.push((format!("{prefix}.norm.gamma"), vec![dim]));
        }
    };

    for li in 0..cfg.depth {
        let time_idx = if cfg.linear_transformer_depth > 0 {
            1
        } else {
            0
        };
        push_transformer(
            &mut out,
            &format!("layers.{li}.{time_idx}"),
            cfg.time_transformer_depth,
        );
        push_transformer(
            &mut out,
            &format!("layers.{li}.{}", time_idx + 1),
            cfg.freq_transformer_depth,
        );
    }

    let hidden = dim * ff_mult;
    for s in 0..cfg.num_stems {
        for (i, bins) in membership.iter().enumerate() {
            let dim_in = 2 * bins.len() * audio_channels;
            let p = format!("mask_estimators.{s}.to_freqs.{i}.0");
            for j in 0..=cfg.mask_estimator_depth {
                let in_dim = if j == 0 { dim } else { hidden };
                let out_dim = if j == cfg.mask_estimator_depth {
                    2 * dim_in
                } else {
                    hidden
                };
                out.push((format!("{p}.{}.weight", 2 * j), vec![out_dim, in_dim]));
                out.push((format!("{p}.{}.bias", 2 * j), vec![out_dim]));
            }
        }
    }
    out
}

/// Bands processed together in the time transformer. The band axis is a pure
/// batch axis, so grouping changes speed/memory only, not the result.
pub(crate) const BAND_BLOCK: usize = 8;
/// Frames processed together in the frequency transformer (also a batch axis).
pub(crate) const T_BLOCK: usize = 64;

pub(crate) fn run_time_transformer<F>(
    cur: &Tensor,
    block: usize,
    f: F,
) -> candle_core::Result<Tensor>
where
    F: Fn(&Tensor) -> candle_core::Result<Tensor>,
{
    let (b, t, bands, d) = cur.dims4()?;
    let cur_p = cur.permute((0, 2, 1, 3))?.contiguous()?; // (b, bands, t, d)
    let mut outs = Vec::new();
    let mut start = 0usize;
    while start < bands {
        let len = block.min(bands - start);
        let xb = cur_p
            .narrow(1, start, len)?
            .contiguous()?
            .reshape((b * len, t, d))?;
        let y = f(&xb)?.reshape((b, len, t, d))?;
        outs.push(y);
        start += len;
    }
    let cat = Tensor::cat(&outs, 1)?; // (b, bands, t, d)
    cat.permute((0, 2, 1, 3))?.contiguous()
}

pub(crate) fn run_freq_transformer<F>(
    cur: &Tensor,
    block: usize,
    f: F,
) -> candle_core::Result<Tensor>
where
    F: Fn(&Tensor) -> candle_core::Result<Tensor>,
{
    let (b, t, bands, d) = cur.dims4()?;
    let mut outs = Vec::new();
    for bi in 0..b {
        let xb = cur.narrow(0, bi, 1)?.squeeze(0)?.contiguous()?; // (t, bands, d)
        let mut touts = Vec::new();
        let mut t0 = 0usize;
        while t0 < t {
            let len = block.min(t - t0);
            let blk = xb.narrow(0, t0, len)?.contiguous()?;
            touts.push(f(&blk)?);
            t0 += len;
        }
        outs.push(Tensor::cat(&touts, 0)?.unsqueeze(0)?);
    }
    Tensor::cat(&outs, 0)
}

pub(crate) fn dump_tensor(name: &str, t: &Tensor) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SEQ: AtomicUsize = AtomicUsize::new(0);
    let Ok(dir) = std::env::var("CANDLE_ROFORMER_DUMP") else {
        return;
    };
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let name = &format!("{name}_{seq}");
    let Ok(v) = t.flatten_all().and_then(|x| x.to_vec1::<f32>()) else {
        return;
    };
    let dims: Vec<String> = t.dims().iter().map(|d| d.to_string()).collect();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(format!("{dir}/{name}.shape"), dims.join(","));
    let mut bytes = Vec::with_capacity(v.len() * 4);
    for x in v {
        bytes.extend_from_slice(&x.to_le_bytes());
    }
    let _ = std::fs::write(format!("{dir}/{name}.f32"), bytes);
}

impl MelBandRoformer {
    pub fn new(cfg: &SeparatorConfig, vb: VarBuilder) -> candle_core::Result<Self> {
        let stereo = cfg.stereo;
        let audio_channels = if stereo { 2 } else { 1 };
        let dim = cfg.dim;
        let depth = cfg.depth;
        let num_stems = cfg.num_stems;
        let num_bands = cfg.num_bands;
        let dim_head = cfg.dim_head;
        let heads = cfg.heads;
        let ff_mult = cfg.mlp_expansion_factor;
        let time_depth = cfg.time_transformer_depth;
        let freq_depth = cfg.freq_transformer_depth;
        let linear_depth = cfg.linear_transformer_depth;
        let norm_output = cfg.norm_output;
        let zero_dc = cfg.zero_dc;

        let membership = mel_bin_membership(cfg.stft_n_fft, cfg.sample_rate, num_bands);
        let freqs = cfg.stft_n_fft / 2 + 1;
        let mut freqs_per_band = vec![vec![false; freqs]; num_bands];
        for band in 0..num_bands {
            for &bin in &membership[band] {
                freqs_per_band[band][bin] = true;
            }
        }
        // freq_indices
        let mut repeated: Vec<usize> = Vec::new();
        for band in 0..num_bands {
            for f in 0..freqs {
                if freqs_per_band[band][f] {
                    repeated.push(f);
                }
            }
        }
        let mut freq_indices = repeated.clone();
        if stereo {
            let mut fi = Vec::new();
            for &f in &repeated {
                for s in 0..2usize {
                    fi.push(f * 2 + s);
                }
            }
            freq_indices = fi;
        }

        // num_freqs_per_band
        let mut num_freqs_per_band = vec![0usize; num_bands];
        for band in 0..num_bands {
            num_freqs_per_band[band] = membership[band].len();
        }
        let mut num_bands_per_freq = vec![0usize; freqs];
        for band in 0..num_bands {
            for &bin in &membership[band] {
                num_bands_per_freq[bin] += 1;
            }
        }

        let mut dim_inputs: Vec<usize> = Vec::new();
        for band in 0..num_bands {
            let f = num_freqs_per_band[band];
            dim_inputs.push(2 * f * audio_channels);
        }

        let mut layers = Vec::with_capacity(depth);
        for li in 0..depth {
            let vb_layer = vb.pp("layers").pp(li.to_string());
            let mut tran_modules = Vec::new();
            if linear_depth > 0 {
                let t = Transformer::new(
                    dim,
                    linear_depth,
                    heads,
                    dim_head,
                    ff_mult,
                    false,
                    vb_layer.pp("0"),
                )?;
                tran_modules.push(t);
            }
            let time_vb_idx = if linear_depth > 0 { 1 } else { 0 };
            let freq_vb_idx = time_vb_idx + 1;
            let time_tr = Transformer::new(
                dim,
                time_depth,
                heads,
                dim_head,
                ff_mult,
                norm_output,
                vb_layer.pp(time_vb_idx.to_string()),
            )?;
            let freq_tr = Transformer::new(
                dim,
                freq_depth,
                heads,
                dim_head,
                ff_mult,
                norm_output,
                vb_layer.pp(freq_vb_idx.to_string()),
            )?;
            tran_modules.push(time_tr);
            tran_modules.push(freq_tr);
            layers.push(tran_modules);
        }

        let band_split = BandSplit::new(dim, dim_inputs.clone(), vb.pp("band_split"))?;

        let mut mask_estimators = Vec::with_capacity(num_stems);
        for si in 0..num_stems {
            let me = MaskEstimator::new(
                dim,
                dim_inputs.clone(),
                cfg.mask_estimator_depth,
                ff_mult,
                vb.pp("mask_estimators").pp(si.to_string()),
            )?;
            mask_estimators.push(me);
        }

        Ok(Self {
            stereo,
            audio_channels,
            num_stems,
            zero_dc,
            layers,
            band_split,
            mask_estimators,
            freq_indices,
            num_bands_per_freq,
            freqs,
            n_fft: cfg.stft_n_fft,
            hop: cfg.stft_hop_length,
            win: cfg.stft_win_length,
            device: vb.device().clone(),
        })
    }

    pub fn forward(&self, raw_audio: &Tensor) -> candle_core::Result<Tensor> {
        let raw_audio = if raw_audio.dims().len() == 2 {
            raw_audio.unsqueeze(1)?
        } else {
            raw_audio.clone()
        };
        let (batch, _channels, raw_len) = raw_audio.dims3()?;
        dump_tensor("00_pcm", &raw_audio);
        let device = raw_audio.device();
        let samples_per = raw_len;

        let raw_vec: Vec<f32> = raw_audio.flatten_all()?.to_vec1()?;
        let mut stft_results: Vec<crate::dsp::StftResult> = Vec::new();
        let mut frames = 0usize;
        let total_bc = batch * self.audio_channels;
        for idx in 0..total_bc {
            let b = idx / self.audio_channels;
            let c = idx % self.audio_channels;
            let off = (b * self.audio_channels + c) * samples_per;
            let sig = &raw_vec[off..off + samples_per];
            let res = crate::dsp::stft(sig, self.n_fft, self.hop, self.win)
                .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
            if frames == 0 {
                frames = res.frames;
            }
            stft_results.push(res);
        }
        let freqs = self.freqs;
        let mut re_flat = vec![0f32; total_bc * freqs * frames];
        let mut im_flat = vec![0f32; total_bc * freqs * frames];
        for idx in 0..total_bc {
            let res = &stft_results[idx];
            for f in 0..frames {
                for fr in 0..freqs {
                    re_flat[idx * freqs * frames + fr * frames + f] = res.re[f * freqs + fr];
                    im_flat[idx * freqs * frames + fr * frames + f] = res.im[f * freqs + fr];
                }
            }
        }
        let re_t = Tensor::from_vec(re_flat, (total_bc, freqs, frames), device)?;
        let im_t = Tensor::from_vec(im_flat, (total_bc, freqs, frames), device)?;
        let re_t = re_t.reshape((batch, self.audio_channels, freqs, frames))?;
        let im_t = im_t.reshape((batch, self.audio_channels, freqs, frames))?;
        let stft_repr = Tensor::stack(&[&re_t, &im_t], D::Minus1)?; // (b,ch,freqs,frames,2)

        let stft_repr = stft_repr.permute((0, 2, 1, 3, 4))?; // (b, freqs, ch, frames, 2)
        let stft_repr = stft_repr.reshape((batch, freqs * self.audio_channels, frames, 2))?;

        dump_tensor("01_stft", &stft_repr);
        let fi_tensor = Tensor::from_vec(
            self.freq_indices
                .iter()
                .map(|&x| x as i64)
                .collect::<Vec<_>>(),
            self.freq_indices.len(),
            device,
        )?;
        let x = stft_repr.index_select(&fi_tensor, 1)?; // (b, selected, frames, 2)
        let selected = x.dim(1)?;
        let x = x.permute((0, 2, 1, 3))?; // (b, frames, selected, 2)
        let x = x.reshape((batch, frames, selected * 2))?;

        let mut cur = self.band_split.forward(&x)?; // (b, frames, num_bands, dim)
        dump_tensor("02_bandsplit", &cur);

        for (i, block) in self.layers.iter().enumerate() {
            if block.len() == 3 {
                let linear_tr = &block[0];
                let time_tr = &block[1];
                let freq_tr = &block[2];

                let (b, t, f, d) = cur.dims4()?;
                let flat = cur.reshape((b, t * f, d))?;
                let flat = linear_tr.forward(&flat)?;
                cur = flat.reshape((b, t, f, d))?;

                cur = run_time_transformer(&cur, BAND_BLOCK, |x| time_tr.forward(x))?;
                cur = run_freq_transformer(&cur, T_BLOCK, |x| freq_tr.forward(x))?;
            } else {
                let time_tr = &block[0];
                let freq_tr = &block[1];
                cur = run_time_transformer(&cur, BAND_BLOCK, |x| time_tr.forward(x))?;
                cur = run_freq_transformer(&cur, T_BLOCK, |x| freq_tr.forward(x))?;
            }
            if i == 0 {
                dump_tensor("03_layer0", &cur);
            }
            let _ = i;
        }

        // mask estimators
        let mut masks_list = Vec::new();
        for me in &self.mask_estimators {
            let m = me.forward(&cur)?; // (b, frames, selected*2? wait per band output)
            masks_list.push(m);
        }
        // stack stems: (b, num_stems, frames, total_selected)
        // each m shape (b, frames, total_selected) where total_selected = selected? Actually selected*2? Let's check: MaskEstimator outputs cat over bands of dim_in (2*f*ch) = selected*2? selected = sum f*ch, times 2 = selected*2. Yes.
        let mut stacked = Vec::new();
        for m in &masks_list {
            stacked.push(m.unsqueeze(1)?);
        }
        let masks = Tensor::cat(&stacked, 1)?; // (b, num_stems, frames, selected*2)
        let masks = masks.reshape((batch, self.mask_estimators.len(), frames, selected, 2))?; // wait need rearrange 'b n t (f c) -> b n f t c' with c=2. Let's do:
                                                                                              // masks currently (b, n, frames, selected*2). reshape to (b,n,frames,selected,2)
        let masks = masks.reshape((batch, self.mask_estimators.len(), frames, selected, 2))?;
        // permute to (b, n, selected, frames, 2)
        let masks = masks.permute((0, 1, 3, 2, 4))?;

        // stft_repr expand stems: (b, 1, freqs*ch, frames, 2)
        let stft_repr = stft_repr.unsqueeze(1)?; // (b,1,freqs*ch,frames,2)

        // complex multiply preparation: convert masks to complex via view? We'll do manual re/im multiplication.
        // stft_repr: (b,1,F,frames,2); masks: (b,n,selected,frames,2)
        // scatter masks into full freq dim
        // scatter_indices: (b, n, selected, frames) indices into freq dim
        let mut scatter_idx_vec =
            Vec::with_capacity(batch * self.mask_estimators.len() * selected * frames);
        for _b in 0..batch {
            for _n in 0..self.mask_estimators.len() {
                for s in 0..selected {
                    let fi = self.freq_indices[s];
                    for _f in 0..frames {
                        scatter_idx_vec.push(fi as i64);
                    }
                }
            }
        }
        // Need shape (b,n,selected,frames)
        let scatter_idx = Tensor::from_vec(
            scatter_idx_vec,
            (batch, self.mask_estimators.len(), selected, frames),
            device,
        )?;

        // masks_summed: zeros (b, n, freqs*ch, frames, 2) complex real/imag
        let full_freqs = freqs * self.audio_channels;
        let masks_summed_re = Tensor::zeros(
            (batch, self.mask_estimators.len(), full_freqs, frames),
            DType::F32,
            device,
        )?;
        let masks_summed_im = Tensor::zeros(
            (batch, self.mask_estimators.len(), full_freqs, frames),
            DType::F32,
            device,
        )?;

        // masks components: (b,n,selected,frames,2)
        let mask_re = masks.narrow(4, 0, 1)?.squeeze(4)?;
        let mask_im = masks.narrow(4, 1, 1)?.squeeze(4)?;
        // scatter_add along dim=2 (freq)
        // flatten last? scatter_add expects source same shape as index broadcast. index shape (b,n,selected,frames). source (b,n,selected,frames). output (b,n,full_freqs,frames).
        let ms_re = masks_summed_re.scatter_add(&scatter_idx, &mask_re.contiguous()?, 2)?;
        let ms_im = masks_summed_im.scatter_add(&scatter_idx, &mask_im.contiguous()?, 2)?;

        // denom: num_bands_per_freq repeated for channels
        let mut denom_vec = Vec::with_capacity(full_freqs);
        for f in 0..freqs {
            let cnt = self.num_bands_per_freq[f];
            for _c in 0..self.audio_channels {
                denom_vec.push(cnt as f32);
            }
        }
        // shape (full_freqs, 1) broadcast
        let denom = Tensor::from_vec(denom_vec, (full_freqs, 1), device)?;
        let denom = denom.reshape((1, 1, full_freqs, 1))?;

        dump_tensor("04_masks_sum", &ms_re);
        // masks_averaged
        let ms_re = ms_re.broadcast_div(&(denom.clone() + 1e-8)?)?;
        let ms_im = ms_im.broadcast_div(&(denom + 1e-8)?)?;

        // stft_repr components (b,1,full_freqs,frames,2) -> (b,full_freqs,frames)
        let sr_re = stft_repr.narrow(4, 0, 1)?.squeeze(4)?.squeeze(1)?;
        let sr_im = stft_repr.narrow(4, 1, 1)?.squeeze(4)?.squeeze(1)?;
        // expand stems: (b, n, full_freqs, frames)
        let sr_re = sr_re.unsqueeze(1)?.broadcast_as((
            batch,
            self.mask_estimators.len(),
            full_freqs,
            frames,
        ))?;
        let sr_im = sr_im.unsqueeze(1)?.broadcast_as((
            batch,
            self.mask_estimators.len(),
            full_freqs,
            frames,
        ))?;

        // complex multiply: (a+ib)*(c+id) = (ac-bd)+i(ad+bc)
        dump_tensor("05_mask_avg", &ms_re);
        let out_re = (sr_re.broadcast_mul(&ms_re)? - sr_im.broadcast_mul(&ms_im)?)?;
        let out_im = (sr_re.broadcast_mul(&ms_im)? + sr_im.broadcast_mul(&ms_re)?)?;

        // zero_dc: upstream zeroes frequency bin 0 for every (batch, stem, channel)
        // after de-interleaving. In the merged (f*ch) layout that is full_freqs
        // indices 0..audio_channels (f == 0, all channels).
        if self.zero_dc {
            let dc = self.audio_channels;
            let zeros_re = Tensor::zeros(
                (batch, self.mask_estimators.len(), dc, frames),
                DType::F32,
                device,
            )?;
            let rest_re = out_re.narrow(2, dc, full_freqs - dc)?;
            let new_re = Tensor::cat(&[&zeros_re, &rest_re], 2)?;
            let zeros_im = Tensor::zeros(
                (batch, self.mask_estimators.len(), dc, frames),
                DType::F32,
                device,
            )?;
            let rest_im = out_im.narrow(2, dc, full_freqs - dc)?;
            let new_im = Tensor::cat(&[&zeros_im, &rest_im], 2)?;
            let out_re_f = new_re;
            let out_im_f = new_im;

            // istft: rearrange (b, n, full_freqs, frames) -> (b*n*audio_channels? wait full_freqs = freqs*channels)
            // upstream: rearrange (b n (f s) t -> (b n s) f t). Need de-interleave channels.
            // Our full_freqs layout after merge: freq-major with channels interleaved? stft_repr merge: (b, freqs, ch, ...) -> reshape (b, freqs*ch, ...). So full_freqs index = f*ch + c. To get (b*n*ch, freqs, frames), need deinterleave.
            // Build arrays per batch/stem/channel.
            let nstems = self.mask_estimators.len();
            let total_signals = batch * nstems * self.audio_channels;
            let mut istft_re = vec![0f32; batch * nstems * self.audio_channels * freqs * frames];
            let mut istft_im = vec![0f32; batch * nstems * self.audio_channels * freqs * frames];
            let out_re_vec: Vec<f32> = out_re_f.flatten_all()?.to_vec1()?;
            let out_im_vec: Vec<f32> = out_im_f.flatten_all()?.to_vec1()?;
            for b in 0..batch {
                for n in 0..nstems {
                    for f in 0..freqs {
                        for c in 0..self.audio_channels {
                            let full_idx = f * self.audio_channels + c;
                            for fr in 0..frames {
                                let src = ((b * nstems + n) * full_freqs + full_idx) * frames + fr;
                                let dst =
                                    ((b * nstems + n) * self.audio_channels + c) * freqs * frames
                                        + fr * freqs
                                        + f;
                                istft_re[dst] = out_re_vec[src];
                                istft_im[dst] = out_im_vec[src];
                            }
                        }
                    }
                }
            }
            let spec_re =
                Tensor::from_vec(istft_re.clone(), (total_signals, frames, freqs), device)?;
            let spec_im =
                Tensor::from_vec(istft_im.clone(), (total_signals, frames, freqs), device)?;
            dump_tensor("07_spec_re", &spec_re);
            dump_tensor("07_spec_im", &spec_im);
            // Now istft per (batch*nstems*channels)
            let mut recon = vec![0f32; total_signals * raw_len];
            for sig in 0..total_signals {
                let off = sig * freqs * frames;
                let mut spec = crate::dsp::StftResult {
                    re: istft_re[off..off + freqs * frames].to_vec(),
                    im: istft_im[off..off + freqs * frames].to_vec(),
                    frames,
                    bins: freqs,
                };
                let y = crate::dsp::istft(&spec, self.n_fft, self.hop, self.win, raw_len)
                    .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
                for i in 0..raw_len {
                    recon[sig * raw_len + i] = y[i];
                }
            }
            // rearrange recon (b, n, ch, samples) -> if num_stems==1 squeeze stem
            let recon_t =
                Tensor::from_vec(recon, (batch, nstems, self.audio_channels, raw_len), device)?;
            dump_tensor("06_recon", &recon_t);
            if nstems == 1 {
                let recon_t = recon_t.squeeze(1)?; // (b, ch, samples)
                return Ok(recon_t);
            }
            Ok(recon_t)
        } else {
            // no zero_dc path (same but without zeroing)
            let out_re_f = out_re;
            let out_im_f = out_im;
            let nstems = self.mask_estimators.len();
            let total_signals = batch * nstems * self.audio_channels;
            let mut istft_re = vec![0f32; batch * nstems * self.audio_channels * freqs * frames];
            let mut istft_im = vec![0f32; batch * nstems * self.audio_channels * freqs * frames];
            let out_re_vec: Vec<f32> = out_re_f.flatten_all()?.to_vec1()?;
            let out_im_vec: Vec<f32> = out_im_f.flatten_all()?.to_vec1()?;
            for b in 0..batch {
                for n in 0..nstems {
                    for f in 0..freqs {
                        for c in 0..self.audio_channels {
                            let full_idx = f * self.audio_channels + c;
                            for fr in 0..frames {
                                let src = ((b * nstems + n) * full_freqs + full_idx) * frames + fr;
                                let dst =
                                    ((b * nstems + n) * self.audio_channels + c) * freqs * frames
                                        + fr * freqs
                                        + f;
                                istft_re[dst] = out_re_vec[src];
                                istft_im[dst] = out_im_vec[src];
                            }
                        }
                    }
                }
            }
            let mut recon = vec![0f32; total_signals * raw_len];
            for sig in 0..total_signals {
                let off = sig * freqs * frames;
                let mut spec = crate::dsp::StftResult {
                    re: istft_re[off..off + freqs * frames].to_vec(),
                    im: istft_im[off..off + freqs * frames].to_vec(),
                    frames,
                    bins: freqs,
                };
                let y = crate::dsp::istft(&spec, self.n_fft, self.hop, self.win, raw_len)
                    .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
                for i in 0..raw_len {
                    recon[sig * raw_len + i] = y[i];
                }
            }
            let recon_t =
                Tensor::from_vec(recon, (batch, nstems, self.audio_channels, raw_len), device)?;
            dump_tensor("06_recon", &recon_t);
            if nstems == 1 {
                let recon_t = recon_t.squeeze(1)?;
                return Ok(recon_t);
            }
            Ok(recon_t)
        }
    }
}

// ---------- Model family dispatch ----------

/// The operations the separation pipeline needs from a loaded separation
/// model, independent of the architecture family. Implemented by
/// [`MelBandRoformer`] and [`BSRoformer`], so `separate::*` runs either.
pub trait RoformerModel {
    /// Forward pass over `(batch, channels, samples)` PCM, returning the model
    /// output in the same layout (`(batch, channels, samples)` for single-stem
    /// profiles, `(batch, stems, channels, samples)` otherwise).
    fn forward(&self, raw_audio: &Tensor) -> candle_core::Result<Tensor>;
    fn audio_channels(&self) -> usize;
    fn num_stems(&self) -> usize;
    fn hop(&self) -> usize;
    fn device(&self) -> &Device;
}

impl RoformerModel for MelBandRoformer {
    fn forward(&self, raw_audio: &Tensor) -> candle_core::Result<Tensor> {
        MelBandRoformer::forward(self, raw_audio)
    }
    fn audio_channels(&self) -> usize {
        self.audio_channels
    }
    fn num_stems(&self) -> usize {
        self.num_stems
    }
    fn hop(&self) -> usize {
        self.hop
    }
    fn device(&self) -> &Device {
        &self.device
    }
}
