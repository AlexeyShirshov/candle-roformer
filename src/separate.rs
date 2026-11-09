use crate::config::InferenceConfig;
use crate::error::Error;
use crate::model::MelBandRoformer;
use candle_core::Tensor;

/// Upstream audio.chunk_size for the v7 profile (8 s at 44100 Hz).
pub const CHUNK_SIZE: usize = 352800;
/// Chunk size derived from the model's STFT hop: 800 hops per chunk.
/// For the v7 profile (hop = 441) this is exactly 352800 samples.
pub fn chunk_size_for(model: &MelBandRoformer) -> usize {
    // Upstream uses 8 s chunks (800 hops). CANDLE_ROFORMER_CHUNK_HOPS is a
    // debugging/tuning override; it changes overlap-add granularity only.
    let hops = std::env::var("CANDLE_ROFORMER_CHUNK_HOPS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(800);
    model.hop.max(1) * hops
}

pub fn separate(
    model: &MelBandRoformer,
    input: &[f32],
    channels: usize,
) -> Result<Vec<f32>, Error> {
    separate_with_config(model, input, channels, &InferenceConfig::default())
}

pub fn separate_with_config(
    model: &MelBandRoformer,
    input: &[f32],
    channels: usize,
    cfg: &InferenceConfig,
) -> Result<Vec<f32>, Error> {
    cfg.validate()?;
    let n = input.len() / channels;
    if n == 0 {
        return Err(Error::Audio(crate::error::AudioError::EmptyInput(
            "empty input".into(),
        )));
    }
    if channels != model.audio_channels {
        return Err(Error::Audio(crate::error::AudioError::UnsupportedFormat(
            format!(
                "model expects {} channels, got {}",
                model.audio_channels, channels
            ),
        )));
    }

    let sr = cfg.sample_rate as u32;

    // Pitch shift: shift the input to a shifted sample rate, run inference on
    // that shifted-rate signal, then map the outputs back to the original
    // length/rate. A default of 0.0 skips entirely (bit-identical).
    if cfg.pitch_shift.abs() > 1e-9 {
        let factor = 2f64.powf(cfg.pitch_shift as f64 / 12.0);
        let shifted_sr = ((sr as f64) * factor).round().max(1.0) as u32;
        let shifted_input = crate::resample::resample_planar(input, channels, sr, shifted_sr);
        let mut inner_cfg = cfg.clone();
        inner_cfg.pitch_shift = 0.0;
        let shifted_out = separate_with_config(model, &shifted_input, channels, &inner_cfg)?;
        let mut back = crate::resample::resample_planar(&shifted_out, channels, shifted_sr, sr);
        let target = channels * n;
        if back.len() > target {
            back.truncate(target);
        } else {
            back.resize(target, 0.0);
        }
        return Ok(back);
    }

    let chunk = cfg.chunk_size(model);
    let step = cfg.step_for(chunk);
    let fade = cfg.fade_for(chunk);
    let batch_size = cfg.batch_size;
    let run = |pcm: &[f32]| -> Result<Vec<f32>, Error> {
        infer_chunk(model, cfg, pcm, channels, chunk)
    };
    ola_with_batch(&input, channels, chunk, step, fade, batch_size, &run)
}

pub fn chunk_size_for_config(model: &MelBandRoformer, cfg: &InferenceConfig) -> usize {
    cfg.chunk_size(model)
}

/// Weighted overlap-add over chunks. `model_fn` receives a chunk padded to
/// `chunk` frames (planar, `channels` x `chunk`) and must return the same shape.
pub fn ola<F>(
    input: &[f32],
    channels: usize,
    chunk: usize,
    model_fn: &F,
) -> Result<Vec<f32>, Error>
where
    F: Fn(&[f32]) -> Result<Vec<f32>, Error>,
{
    ola_with(input, channels, chunk, (chunk / 2).max(1), (chunk / 10).max(1), model_fn)
}

pub fn ola_with<F>(
    input: &[f32],
    channels: usize,
    chunk: usize,
    step: usize,
    fade: usize,
    model_fn: &F,
) -> Result<Vec<f32>, Error>
where
    F: Fn(&[f32]) -> Result<Vec<f32>, Error>,
{
    ola_with_batch(input, channels, chunk, step, fade, 1, model_fn)
}

pub fn ola_with_batch<F>(
    input: &[f32],
    channels: usize,
    chunk: usize,
    step: usize,
    fade: usize,
    batch_size: usize,
    model_fn: &F,
) -> Result<Vec<f32>, Error>
where
    F: Fn(&[f32]) -> Result<Vec<f32>, Error>,
{
    if chunk == 0 {
        return Err(Error::Audio(crate::error::AudioError::EmptyInput(
            "chunk size is zero".into(),
        )));
    }
    let step = step.max(1);
    let fade = fade.max(1).min(chunk.max(1));
    let batch_size = batch_size.max(1);
    let n = input.len() / channels;
    if n == 0 {
        return Ok(Vec::new());
    }
    let mut out_acc = vec![0f32; channels * n];
    let mut weight_acc = vec![0f32; n];
    let num_chunks = (n + step - 1) / step;

    let mut fade_w = vec![1f32; chunk];
    for (t, w) in fade_w.iter_mut().enumerate() {
        let mut fi = 1.0f32;
        if t < fade {
            fi = (t as f32 + 1.0) / fade as f32;
        }
        let mut fo = 1.0f32;
        if t >= chunk - fade {
            fo = (chunk - t) as f32 / fade as f32;
        }
        *w = fi.min(fo).max(1.0 / fade as f32);
    }

    let mut ci = 0usize;
    while ci < num_chunks {
        // collect a batch of chunk indices
        let mut indices: Vec<usize> = Vec::new();
        let mut b = 0usize;
        while b < batch_size && ci + b < num_chunks {
            let start = (ci + b) * step;
            if start >= n {
                break;
            }
            indices.push(ci + b);
            b += 1;
        }
        if indices.is_empty() {
            break;
        }
        let actual = indices.len();
        let mut pcm_batch = vec![0f32; channels * chunk * actual];
        let mut seg_lens = vec![0usize; actual];
        for (bi, &chunk_idx) in indices.iter().enumerate() {
            let start = chunk_idx * step;
            let seg_len = (n - start).min(chunk);
            seg_lens[bi] = seg_len;
            for ch in 0..channels {
                for i in 0..seg_len {
                    pcm_batch[(bi * channels + ch) * chunk + i] = input[ch * n + start + i];
                }
            }
        }
        let model_out = model_fn(&pcm_batch)?;
        if model_out.len() != channels * chunk * actual {
            return Err(Error::Audio(crate::error::AudioError::UnsupportedFormat(
                format!(
                    "model returned {} samples, expected {}",
                    model_out.len(),
                    channels * chunk * actual
                ),
            )));
        }
        for (bi, &chunk_idx) in indices.iter().enumerate() {
            let start = chunk_idx * step;
            let seg_len = seg_lens[bi];
            let out_slice = &model_out[bi * channels * chunk..(bi + 1) * channels * chunk];
            for ch in 0..channels {
                for i in 0..seg_len {
                    let t = start + i;
                    let w = fade_w[i];
                    out_acc[ch * n + t] += out_slice[ch * chunk + i] * w;
                    if ch == 0 {
                        weight_acc[t] += w;
                    }
                }
            }
        }
        ci += actual;
    }

    for ch in 0..channels {
        for t in 0..n {
            let w = weight_acc[t];
            if w > 1e-8 {
                out_acc[ch * n + t] /= w;
            }
        }
    }
    Ok(out_acc)
}

pub fn ola_overlap<F>(
    input: &[f32],
    channels: usize,
    chunk: usize,
    overlap: usize,
    model_fn: &F,
) -> Result<Vec<f32>, Error>
where
    F: Fn(&[f32]) -> Result<Vec<f32>, Error>,
{
    let k = overlap.max(1);
    let step = (chunk / k).max(1);
    ola_with(input, channels, chunk, step, (chunk / 10).max(1), model_fn)
}

pub fn infer_chunk(
    model: &MelBandRoformer,
    cfg: &InferenceConfig,
    pcm: &[f32],
    channels: usize,
    chunk: usize,
) -> Result<Vec<f32>, Error> {
    cfg.validate().map_err(Error::Config)?;
    let device = model.device.clone();
    let denoise = cfg.denoise;
    let tta = cfg.tta;
        let b = pcm.len() / (channels * chunk).max(1);
        let b = b.max(1);
        if denoise {
            let t = Tensor::from_vec(pcm.to_vec(), (b, channels, chunk), &device)
                .map_err(Error::Candle)?;
            let out_pos = model.forward(&t).map_err(Error::Candle)?;
            let vpos: Vec<f32> = out_pos.flatten_all().map_err(Error::Candle)?.to_vec1().map_err(Error::Candle)?;
            let neg: Vec<f32> = pcm.iter().map(|x| -x).collect();
            let t2 = Tensor::from_vec(neg, (b, channels, chunk), &device).map_err(Error::Candle)?;
            let out_neg = model.forward(&t2).map_err(Error::Candle)?;
            let vneg: Vec<f32> = out_neg.flatten_all().map_err(Error::Candle)?.to_vec1().map_err(Error::Candle)?;
            let v: Vec<f32> = vpos.iter().zip(vneg.iter()).map(|(a, b)| (a - b) * 0.5).collect();
            Ok(v)
        } else if tta {
            let mut acc = vec![0f32; b * channels * chunk];
            let mut ncount = 0usize;
            // identity
            {
                let t = Tensor::from_vec(pcm.to_vec(), (b, channels, chunk), &device).map_err(Error::Candle)?;
                let out = model.forward(&t).map_err(Error::Candle)?;
                let v: Vec<f32> = out.flatten_all().map_err(Error::Candle)?.to_vec1().map_err(Error::Candle)?;
                for i in 0..acc.len() { acc[i] += v[i]; }
                ncount += 1;
            }
            // phase inversion
            {
                let neg: Vec<f32> = pcm.iter().map(|x| -x).collect();
                let t = Tensor::from_vec(neg, (b, channels, chunk), &device).map_err(Error::Candle)?;
                let out = model.forward(&t).map_err(Error::Candle)?;
                let v: Vec<f32> = out.flatten_all().map_err(Error::Candle)?.to_vec1().map_err(Error::Candle)?;
                for i in 0..acc.len() { acc[i] -= v[i]; }
                ncount += 1;
            }
            // channel swap (if stereo+)
            if channels >= 2 {
                let mut swapped = vec![0f32; b * channels * chunk];
                for bi in 0..b {
                    for ch in 0..channels {
                        for i in 0..chunk {
                            swapped[bi * channels * chunk + ch * chunk + i] =
                                pcm[bi * channels * chunk + (channels - 1 - ch) * chunk + i];
                        }
                    }
                }
                let t = Tensor::from_vec(swapped, (b, channels, chunk), &device).map_err(Error::Candle)?;
                let out = model.forward(&t).map_err(Error::Candle)?;
                let v: Vec<f32> = out.flatten_all().map_err(Error::Candle)?.to_vec1().map_err(Error::Candle)?;
                let mut unswapped = vec![0f32; b * channels * chunk];
                for bi in 0..b {
                    for ch in 0..channels {
                        for i in 0..chunk {
                            unswapped[bi * channels * chunk + ch * chunk + i] =
                                v[bi * channels * chunk + (channels - 1 - ch) * chunk + i];
                        }
                    }
                }
                for i in 0..acc.len() { acc[i] += unswapped[i]; }
                ncount += 1;
            }
            for x in &mut acc { *x /= ncount as f32; }
            Ok(acc)
        } else {
            let t = Tensor::from_vec(pcm.to_vec(), (b, channels, chunk), &device)
                .map_err(Error::Candle)?;
            let out = model.forward(&t).map_err(Error::Candle)?;
            let v: Vec<f32> = out
                .flatten_all()
                .map_err(Error::Candle)?
                .to_vec1()
                .map_err(Error::Candle)?;
            Ok(v)
        }
}

#[cfg(test)]
mod debug_tests {
    use super::*;
    #[test]
    fn det() {
        // can't easily build model here; skip
    }
}
