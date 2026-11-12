//! BS-RoFormer ("band split RoFormer") implementation.
//!
//! The architecture is the reference one used by the published UVR
//! `model_bs_roformer_*.ckpt` family as implemented by
//! `audio_separator.separator.uvr_lib_v5.roformer.bs_roformer` (audio-separator
//! 0.48.0): a band split over explicit consecutive frequency widths
//! (`freqs_per_bands`), per-layer time and frequency transformers with rotary
//! position embeddings, a final RMSNorm and one GLU mask estimator per stem.
//!
//! Differences from [`super::MelBandRoformer`] worth knowing:
//!
//! * bands are *contiguous* STFT bins, not mel membership, so the per-band mask
//!   is concatenated straight back into the (freq, channel) axis (no
//!   scatter-add averaging);
//! * the transformers carry no output norm and no per-band membership table;
//!   one `final_norm` is applied once after all layers;
//! * rotary tables are per-attention `rotary_embed.freqs` buffers (the
//!   published checkpoints store them, shared between blocks) and the
//!   feed-forward uses the exact `erf` GELU.
//!
//! `zero_dc` is a training-side flag in the reference and is not applied to the
//! forward pass, so it is not applied here either.

use candle_core::{Device, Module, Tensor, D};
use candle_nn::{linear, Linear, VarBuilder};

use super::{
    run_freq_transformer, run_time_transformer, Attention, BandSplit, FeedForward, RmsNorm,
    BAND_BLOCK, T_BLOCK,
};
use crate::config::SeparatorConfig;

// ---------- MLP (reference layout: depth linear layers, Tanh between) ----------

#[derive(Debug, Clone)]
struct BsMlp {
    linears: Vec<Linear>,
}

impl BsMlp {
    /// `dims = (dim_in, hidden * (depth - 1), dim_out)` exactly like the
    /// reference `MLP(dim_in, dim_out, dim_hidden, depth)` helper, so the
    /// checkpoint keys are `0`, `2`, `4`, ... (PyTorch Sequential indices).
    fn new(
        dim_in: usize,
        dim_out: usize,
        dim_hidden: usize,
        depth: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let depth = depth.max(1);
        let mut dims = vec![dim_in];
        for _ in 0..depth.saturating_sub(1) {
            dims.push(dim_hidden);
        }
        dims.push(dim_out);
        let mut linears = Vec::with_capacity(dims.len() - 1);
        for (idx, w) in dims.windows(2).enumerate() {
            linears.push(linear(w[0], w[1], vb.pp((idx * 2).to_string()))?);
        }
        Ok(Self { linears })
    }

    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let n = self.linears.len();
        let mut h = x.clone();
        for (i, lin) in self.linears.iter().enumerate() {
            h = lin.forward(&h)?;
            if i + 1 < n {
                h = h.tanh()?;
            }
        }
        Ok(h)
    }
}

// ---------- Mask estimator ----------

#[derive(Debug, Clone)]
struct BsMaskEstimator {
    to_freqs: Vec<BsMlp>,
}

impl BsMaskEstimator {
    fn new(
        dim: usize,
        dim_inputs: &[usize],
        depth: usize,
        mlp_expansion_factor: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let dim_hidden = dim * mlp_expansion_factor;
        let mut to_freqs = Vec::with_capacity(dim_inputs.len());
        for (i, &dim_in) in dim_inputs.iter().enumerate() {
            let mlp = BsMlp::new(
                dim,
                dim_in * 2,
                dim_hidden,
                depth,
                vb.pp("to_freqs").pp(i.to_string()).pp("0"),
            )?;
            to_freqs.push(mlp);
        }
        Ok(Self { to_freqs })
    }

    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let bands = x.dim(2)?;
        let mut outs = Vec::with_capacity(bands);
        for i in 0..bands {
            // A band slice of (b, t, bands, dim) is a strided view. Candle's
            // Metal matmul requires contiguous operands, and this slice feeds an
            // MLP directly, so materialise it. A copy, not a numerical change.
            let band_x = x.narrow(2, i, 1)?.squeeze(2)?.contiguous()?; // (b, t, dim)
            let out = self.to_freqs[i].forward(&band_x)?; // (b, t, dim_in * 2)
            let half = out.dim(D::Minus1)? / 2;
            let a = out.narrow(D::Minus1, 0, half)?.contiguous()?;
            let b = out.narrow(D::Minus1, half, half)?.contiguous()?;
            let gate = candle_nn::ops::sigmoid(&b)?;
            outs.push(a.broadcast_mul(&gate)?);
        }
        Tensor::cat(&outs, D::Minus1)
    }
}

// ---------- Transformer ----------

#[derive(Debug, Clone)]
struct BsTransformer {
    layers: Vec<(Attention, FeedForward)>,
}

impl BsTransformer {
    fn new(
        dim: usize,
        depth: usize,
        heads: usize,
        dim_head: usize,
        ff_mult: usize,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let mut layers = Vec::with_capacity(depth);
        for d in 0..depth {
            let vb_layer = vb.pp("layers").pp(d.to_string());
            let attn = Attention::new(dim, heads, dim_head, vb_layer.pp("0"))?;
            let ff = FeedForward::new_erf(dim, ff_mult, vb_layer.pp("1"))?;
            layers.push((attn, ff));
        }
        Ok(Self { layers })
    }

    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let mut x = x.clone();
        for (attn, ff) in &self.layers {
            let a = attn.forward(&x)?;
            x = (x + a)?;
            let f = ff.forward(&x)?;
            x = (x + f)?;
        }
        Ok(x)
    }
}

// ---------- expected keys ----------

/// Parameter names and shapes of a BS-RoFormer profile. Mirrors the reference
/// module structure (`band_split`, `layers.<i>.<time|freq>.layers.<d>`,
/// `final_norm`, `mask_estimators.<s>.to_freqs.<band>.0`).
pub fn expected_key_shapes(cfg: &SeparatorConfig) -> Vec<(String, Vec<usize>)> {
    let dim = cfg.dim;
    let audio_channels = if cfg.stereo { 2 } else { 1 };
    let freqs_per_bands = cfg.freqs_per_bands.clone().unwrap_or_default();
    let inner = cfg.heads * cfg.dim_head;
    let ff_mult = cfg.mlp_expansion_factor;
    let hidden = dim * ff_mult;
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();

    for (i, &bins) in freqs_per_bands.iter().enumerate() {
        let dim_in = 2 * bins * audio_channels;
        out.push((format!("band_split.to_features.{i}.0.gamma"), vec![dim_in]));
        out.push((
            format!("band_split.to_features.{i}.1.weight"),
            vec![dim, dim_in],
        ));
        out.push((format!("band_split.to_features.{i}.1.bias"), vec![dim]));
    }

    let push_transformer = |out: &mut Vec<(String, Vec<usize>)>, prefix: &str, depth: usize| {
        for d in 0..depth.max(1) {
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
    };

    for li in 0..cfg.depth {
        push_transformer(
            &mut out,
            &format!("layers.{li}.0"),
            cfg.time_transformer_depth,
        );
        push_transformer(
            &mut out,
            &format!("layers.{li}.1"),
            cfg.freq_transformer_depth,
        );
    }

    out.push(("final_norm.gamma".to_string(), vec![dim]));

    for s in 0..cfg.num_stems {
        for (i, &bins) in freqs_per_bands.iter().enumerate() {
            let dim_in = 2 * bins * audio_channels;
            let p = format!("mask_estimators.{s}.to_freqs.{i}.0");
            let depth = cfg.mask_estimator_depth.max(1);
            for j in 0..depth {
                let in_dim = if j == 0 { dim } else { hidden };
                let out_dim = if j + 1 == depth { 2 * dim_in } else { hidden };
                out.push((format!("{p}.{}.weight", 2 * j), vec![out_dim, in_dim]));
                out.push((format!("{p}.{}.bias", 2 * j), vec![out_dim]));
            }
        }
    }
    out
}

// ---------- model ----------

pub struct BSRoformer {
    pub stereo: bool,
    pub audio_channels: usize,
    pub num_stems: usize,
    pub depth: usize,
    pub freqs_per_bands: Vec<usize>,
    /// One (time, frequency) transformer pair per model layer.
    layers: Vec<(BsTransformer, BsTransformer)>,
    pub band_split: BandSplit,
    pub final_norm: RmsNorm,
    mask_estimators: Vec<BsMaskEstimator>,
    pub n_fft: usize,
    pub hop: usize,
    pub win: usize,
    pub freqs: usize,
    pub device: Device,
}

impl BSRoformer {
    pub fn new(cfg: &SeparatorConfig, vb: VarBuilder) -> candle_core::Result<Self> {
        let freqs_per_bands = cfg.freqs_per_bands.clone().ok_or_else(|| {
            candle_core::Error::Msg("bs_roformer config requires freqs_per_bands".into())
        })?;
        let audio_channels = if cfg.stereo { 2 } else { 1 };
        let dim = cfg.dim;
        let heads = cfg.heads;
        let dim_head = cfg.dim_head;
        let ff_mult = cfg.mlp_expansion_factor;
        let device = vb.device().clone();

        let dim_inputs: Vec<usize> = freqs_per_bands
            .iter()
            .map(|f| 2 * f * audio_channels)
            .collect();

        let mut layers = Vec::with_capacity(cfg.depth);
        for li in 0..cfg.depth {
            let vb_l = vb.pp("layers").pp(li.to_string());
            let time = BsTransformer::new(
                dim,
                cfg.time_transformer_depth,
                heads,
                dim_head,
                ff_mult,
                vb_l.pp("0"),
            )?;
            let freq = BsTransformer::new(
                dim,
                cfg.freq_transformer_depth,
                heads,
                dim_head,
                ff_mult,
                vb_l.pp("1"),
            )?;
            layers.push((time, freq));
        }

        let band_split = BandSplit::new(dim, dim_inputs.clone(), vb.pp("band_split"))?;
        let final_norm = RmsNorm::new(dim, vb.pp("final_norm"))?;

        let mut mask_estimators = Vec::with_capacity(cfg.num_stems);
        for si in 0..cfg.num_stems {
            mask_estimators.push(BsMaskEstimator::new(
                dim,
                &dim_inputs,
                cfg.mask_estimator_depth,
                ff_mult,
                vb.pp("mask_estimators").pp(si.to_string()),
            )?);
        }

        Ok(Self {
            stereo: cfg.stereo,
            audio_channels,
            num_stems: cfg.num_stems,
            depth: cfg.depth,
            freqs_per_bands,
            layers,
            band_split,
            final_norm,
            mask_estimators,
            n_fft: cfg.stft_n_fft,
            hop: cfg.stft_hop_length,
            win: cfg.stft_win_length,
            freqs: cfg.stft_n_fft / 2 + 1,
            device,
        })
    }

    pub fn forward(&self, raw_audio: &Tensor) -> candle_core::Result<Tensor> {
        let raw_audio = if raw_audio.dims().len() == 2 {
            raw_audio.unsqueeze(1)?
        } else {
            raw_audio.clone()
        };
        let (batch, channels, raw_len) = raw_audio.dims3()?;
        if channels != self.audio_channels {
            return Err(candle_core::Error::Msg(format!(
                "bs_roformer expects {} channels, got {}",
                self.audio_channels, channels
            )));
        }
        crate::model::dump_tensor("bs_00_pcm", &raw_audio);
        let device = raw_audio.device();
        let samples_per = raw_len;
        let freqs = self.freqs;

        // STFT per (batch, channel), identical to the reference torch.stft
        // (periodic Hann window, center=True, reflect padding).
        let raw_vec: Vec<f32> = raw_audio.flatten_all()?.to_vec1()?;
        let total_bc = batch * self.audio_channels;
        let mut stft_results: Vec<crate::dsp::StftResult> = Vec::with_capacity(total_bc);
        let mut frames = 0usize;
        for idx in 0..total_bc {
            let off = idx * samples_per;
            let sig = &raw_vec[off..off + samples_per];
            let res = crate::dsp::stft(sig, self.n_fft, self.hop, self.win)
                .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
            if frames == 0 {
                frames = res.frames;
            }
            stft_results.push(res);
        }

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
        let stft_repr = Tensor::stack(&[&re_t, &im_t], D::Minus1)?; // (b, ch, f, t, 2)

        // "b s f t c -> b (f s) t c": frequency leading, channel interleaved.
        let freqs_c = freqs * self.audio_channels;
        let stft_repr = stft_repr
            .permute((0, 2, 1, 3, 4))?
            .reshape((batch, freqs_c, frames, 2))?;
        // "b f t c -> b t (f c)"
        let x = stft_repr
            .permute((0, 2, 1, 3))?
            .reshape((batch, frames, freqs_c * 2))?;

        let mut cur = self.band_split.forward(&x)?; // (b, t, bands, dim)
        crate::model::dump_tensor("bs_01_bandsplit", &cur);

        for (i, (time, freq)) in self.layers.iter().enumerate() {
            cur = run_time_transformer(&cur, BAND_BLOCK, |x| time.forward(x))?;
            cur = run_freq_transformer(&cur, T_BLOCK, |x| freq.forward(x))?;
            if i == 0 {
                crate::model::dump_tensor("bs_02_layer0", &cur);
            }
        }
        let cur = self.final_norm.forward(&cur)?;

        // One mask per stem: (b, n, t, freqs_c * 2).
        let mut stacked = Vec::with_capacity(self.mask_estimators.len());
        for me in &self.mask_estimators {
            stacked.push(me.forward(&cur)?.unsqueeze(1)?);
        }
        let num_stems = self.mask_estimators.len();
        let masks = Tensor::cat(&stacked, 1)?;
        // "b n t (f c) -> b n f t c"
        let masks = masks
            .reshape((batch, num_stems, frames, freqs_c, 2))?
            .permute((0, 1, 3, 2, 4))?;
        crate::model::dump_tensor("bs_03_mask", &masks);

        let mask_re = masks.narrow(4, 0, 1)?.squeeze(4)?; // (b, n, f, t)
        let mask_im = masks.narrow(4, 1, 1)?.squeeze(4)?;
        let spec_re = stft_repr.narrow(3, 0, 1)?.squeeze(3)?; // (b, f, t)
        let spec_im = stft_repr.narrow(3, 1, 1)?.squeeze(3)?;
        let spec_re = spec_re
            .unsqueeze(1)?
            .broadcast_as((batch, num_stems, freqs_c, frames))?;
        let spec_im = spec_im
            .unsqueeze(1)?
            .broadcast_as((batch, num_stems, freqs_c, frames))?;

        // (a + ib) * (c + id)
        let out_re = (spec_re.broadcast_mul(&mask_re)? - spec_im.broadcast_mul(&mask_im)?)?;
        let out_im = (spec_re.broadcast_mul(&mask_im)? + spec_im.broadcast_mul(&mask_re)?)?;
        crate::model::dump_tensor("bs_04_out_re", &out_re);

        // iSTFT per (batch, stem, channel).
        let total_signals = batch * num_stems * self.audio_channels;
        let mut istft_re = vec![0f32; total_signals * freqs * frames];
        let mut istft_im = vec![0f32; total_signals * freqs * frames];
        let out_re_vec: Vec<f32> = out_re.contiguous()?.flatten_all()?.to_vec1()?;
        let out_im_vec: Vec<f32> = out_im.contiguous()?.flatten_all()?.to_vec1()?;
        for b in 0..batch {
            for n in 0..num_stems {
                for f in 0..freqs {
                    for c in 0..self.audio_channels {
                        let full_idx = f * self.audio_channels + c;
                        for fr in 0..frames {
                            let src = ((b * num_stems + n) * freqs_c + full_idx) * frames + fr;
                            let dst =
                                ((b * num_stems + n) * self.audio_channels + c) * freqs * frames
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
            let spec = crate::dsp::StftResult {
                re: istft_re[off..off + freqs * frames].to_vec(),
                im: istft_im[off..off + freqs * frames].to_vec(),
                frames,
                bins: freqs,
            };
            let y = crate::dsp::istft(&spec, self.n_fft, self.hop, self.win, raw_len)
                .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
            recon[sig * raw_len..(sig + 1) * raw_len].copy_from_slice(&y);
        }
        let recon_t = Tensor::from_vec(
            recon,
            (batch, num_stems, self.audio_channels, raw_len),
            device,
        )?;
        crate::model::dump_tensor("bs_05_recon", &recon_t);
        if num_stems == 1 {
            return recon_t.squeeze(1);
        }
        Ok(recon_t)
    }
}

impl super::RoformerModel for BSRoformer {
    fn forward(&self, raw_audio: &Tensor) -> candle_core::Result<Tensor> {
        BSRoformer::forward(self, raw_audio)
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
