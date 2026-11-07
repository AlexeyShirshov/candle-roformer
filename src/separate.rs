use crate::error::Error;
use crate::model::MelBandRoformer;
use candle_core::Tensor;

/// Upstream audio.chunk_size for the v7 profile (8 s at 44100 Hz).
pub const CHUNK_SIZE: usize = 352800;
/// Chunk size derived from the model's STFT hop: 800 hops per chunk.
/// For the v7 profile (hop = 441) this is exactly 352800 samples.
pub fn chunk_size_for(model: &MelBandRoformer) -> usize {
    model.hop.max(1) * 800
}

pub fn separate(
    model: &MelBandRoformer,
    input: &[f32],
    channels: usize,
) -> Result<Vec<f32>, Error> {
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
    let chunk = chunk_size_for(model);
    let device = model.device.clone();
    let run = |chunk_pcm: &[f32]| -> Result<Vec<f32>, Error> {
        let t = Tensor::from_vec(chunk_pcm.to_vec(), (1, channels, chunk), &device)
            .map_err(Error::Candle)?;
        let out = model.forward(&t).map_err(Error::Candle)?;
        let v: Vec<f32> = out
            .flatten_all()
            .map_err(Error::Candle)?
            .to_vec1()
            .map_err(Error::Candle)?;
        Ok(v)
    };
    ola(input, channels, chunk, &run)
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
    if chunk == 0 {
        return Err(Error::Audio(crate::error::AudioError::EmptyInput(
            "chunk size is zero".into(),
        )));
    }
    let n = input.len() / channels;
    if n == 0 {
        return Ok(Vec::new());
    }
    let step = (chunk / 2).max(1);
    let fade = (chunk / 10).max(1);
    let mut out_acc = vec![0f32; channels * n];
    let mut weight_acc = vec![0f32; n];
    let num_chunks = (n + step - 1) / step;
    for ci in 0..num_chunks {
        let start = ci * step;
        if start >= n {
            break;
        }
        let seg_len = (n - start).min(chunk);
        let mut pcm = vec![0f32; channels * chunk];
        for ch in 0..channels {
            for i in 0..seg_len {
                pcm[ch * chunk + i] = input[ch * n + start + i];
            }
        }
        let model_out = model_fn(&pcm)?;
        if model_out.len() != channels * chunk {
            return Err(Error::Audio(crate::error::AudioError::UnsupportedFormat(
                format!(
                    "model returned {} samples, expected {}",
                    model_out.len(),
                    channels * chunk
                ),
            )));
        }
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
        for ch in 0..channels {
            for i in 0..seg_len {
                let t = start + i;
                let w = fade_w[i];
                out_acc[ch * n + t] += model_out[ch * chunk + i] * w;
                if ch == 0 {
                    weight_acc[t] += w;
                }
            }
        }
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
