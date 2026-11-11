//! Ensemble operators and preset-driven multi-model separation.
//!
//! The numerics mirror `audio-separator` 0.48.0 exactly
//! (`audio_separator/separator/ensembler.py`, `Ensembler.ensemble`), which is
//! the reference named by issue #8:
//!
//! * `avg_wave` — accumulate `waveform * weight` over the models **in
//!   application order** into an accumulator with the waveforms' dtype. The
//!   reference writes `ensembled = np.zeros_like(waveforms[0])` and then
//!   `ensembled += w * weight`, where the weights are a `float64` NumPy array:
//!   the product and the sum are evaluated in `float64` and each step is written
//!   back into the `float32` accumulator, i.e. one round-to-`f32` per step. The
//!   final `ensembled / np.sum(weights)` is `float32 / float64-scalar`, which
//!   NumPy evaluates in `float64`; the result is cast back to the crate's `f32`
//!   buffer once.
//! * `median_wave` — per sample (planar, channel-major, no axis folding) the
//!   median of the model values, i.e. the middle value for an odd model count
//!   and the `f32` mean of the two middle values for an even one. Weights are
//!   ignored by the reference for this operator and are ignored here too.
//! * Lengths are aligned by zero-padding the shorter waveforms **per channel**,
//!   in frames, to the longest one's frame count, exactly like
//!   `np.pad(.., (0, max_length - w.shape[1]))` on the reference's channel-major
//!   `[channels, frames]` array: channel `c` contributes its own `frames` samples
//!   followed by zeros, so a short channel never shifts another channel's data.
//!   Nothing is trimmed and no output is normalized or rescaled.
//!
//! Deliberate difference, documented in `docs/ensemble-presets.md`: the
//! reference returns `None` for an empty waveform list and falls back to equal
//! weights (with a warning) when the supplied weights do not match the model
//! count. This crate returns the typed
//! [`EnsembleError::EmptyWaveforms`]/[`EnsembleError::InvalidWeights`] instead,
//! so a mis-specified ensemble fails loudly rather than changing semantics
//! silently.

use std::cmp::Ordering;

use crate::config::{EnsembleAlgorithm, EnsemblePreset};
use crate::error::{EnsembleError, Error};
use crate::separator::{Separator, Stems};
use crate::wav::AudioBuffer;

/// Channel count, sample rate and longest frame count of a waveform list.
fn layout(waveforms: &[AudioBuffer]) -> Result<(u32, usize, usize), Error> {
    let first = &waveforms[0];
    let channels = first.channels as usize;
    let mut frames = first.frames();
    for other in &waveforms[1..] {
        if other.channels as usize != channels {
            return Err(Error::Ensemble(EnsembleError::ChannelMismatch(format!(
                "{} channels vs {} channels",
                channels, other.channels
            ))));
        }
        if other.sample_rate != first.sample_rate {
            return Err(Error::Ensemble(EnsembleError::SampleRateMismatch(format!(
                "{} Hz vs {} Hz",
                first.sample_rate, other.sample_rate
            ))));
        }
        frames = frames.max(other.frames());
    }
    Ok((first.sample_rate, channels, frames))
}

/// Resolve the per-model weights, mirroring the reference default of unit
/// weights for `None`; a wrong count is a typed error (see the module docs).
fn resolve_weights(weights: Option<&[f32]>, models: usize) -> Result<Vec<f32>, Error> {
    match weights {
        None => Ok(vec![1.0; models]),
        Some(w) => {
            if w.len() != models {
                return Err(Error::Ensemble(EnsembleError::InvalidWeights(format!(
                    "{} weights for {} waveforms",
                    w.len(),
                    models
                ))));
            }
            if w.iter().any(|v| !v.is_finite()) {
                return Err(Error::Ensemble(EnsembleError::InvalidWeights(
                    "weights must be finite".into(),
                )));
            }
            let sum: f64 = w.iter().map(|v| *v as f64).sum();
            if sum == 0.0 {
                return Err(Error::Ensemble(EnsembleError::InvalidWeights(
                    "weights sum to zero".into(),
                )));
            }
            Ok(w.to_vec())
        }
    }
}

/// Sample value of `buffer` at frame `frame` of `channel`, or zero past that
/// channel's own frame count.
///
/// The padding is **per channel**: the reference pads a `[channels, frames]`
/// array with `np.pad(.., (0, max_length - w.shape[1]))`, which appends zeros to
/// the last axis of every row, so channel `c` contributes its own `frames`
/// samples and then zeros. Padding the flattened buffer instead would read one
/// channel's data at another channel's offset.
#[inline]
fn padded_sample(buffer: &AudioBuffer, channel: usize, frame: usize) -> f32 {
    if frame >= buffer.frames() {
        return 0.0;
    }
    buffer
        .planar
        .get(channel * buffer.frames() + frame)
        .copied()
        .unwrap_or(0.0)
}

/// The median of `values`, in the reference's `f32` arithmetic: middle element
/// for an odd count, `(a + b) / 2` for an even one.
fn median_of(values: &mut [f32]) -> f32 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

/// Apply `algorithm` across `waveforms` and return the ensembled waveform.
///
/// `waveforms` are in application order (the order a preset lists its models);
/// every buffer must share the channel count and sample rate. A single waveform
/// is returned unchanged, exactly like the reference (`waveforms[0]`), including
/// when `weights` is supplied.
pub fn ensemble_waveforms(
    algorithm: EnsembleAlgorithm,
    weights: Option<&[f32]>,
    waveforms: &[AudioBuffer],
) -> Result<AudioBuffer, Error> {
    if waveforms.is_empty() {
        return Err(Error::Ensemble(EnsembleError::EmptyWaveforms));
    }
    for w in waveforms {
        w.validate()?;
    }
    if waveforms.len() == 1 {
        return Ok(waveforms[0].clone());
    }

    let (sample_rate, channels, frames) = layout(waveforms)?;
    let weights = resolve_weights(weights, waveforms.len())?;
    let total = frames * channels;
    let mut out = vec![0f32; total];

    match algorithm {
        EnsembleAlgorithm::AvgWave => {
            // `ensembled += w * weight` with a float32 accumulator and float64
            // weights: the product and the addition are evaluated in f64 and the
            // store rounds once to f32, per model, in application order.
            for (w, weight) in waveforms.iter().zip(weights.iter()) {
                let weight = *weight as f64;
                for channel in 0..channels {
                    for frame in 0..frames {
                        let i = channel * frames + frame;
                        out[i] =
                            (out[i] as f64 + padded_sample(w, channel, frame) as f64 * weight)
                                as f32;
                    }
                }
            }
            let sum: f64 = weights.iter().map(|v| *v as f64).sum();
            for v in out.iter_mut() {
                *v = (*v as f64 / sum) as f32;
            }
        }
        EnsembleAlgorithm::MedianWave => {
            let mut bucket: Vec<f32> = Vec::with_capacity(waveforms.len());
            for channel in 0..channels {
                for frame in 0..frames {
                    bucket.clear();
                    for w in waveforms {
                        bucket.push(padded_sample(w, channel, frame));
                    }
                    out[channel * frames + frame] = median_of(&mut bucket);
                }
            }
        }
    }

    Ok(AudioBuffer {
        sample_rate,
        channels: channels as u16,
        planar: out,
    })
}

/// Separate `audio` with every preset member and combine the members' target
/// stems with the preset's operator.
///
/// `separators` must be in `preset.models` order and have the same length. The
/// returned stem set is the preset's documented set: the ensembled target stem
/// (`vocals`) and the input residual (`instrumental`, i.e. `input - vocals`,
/// pointwise), so the two stems reconstruct the mixture.
pub fn separate_with_preset(
    separators: &[Separator],
    preset: &EnsemblePreset,
    audio: &AudioBuffer,
) -> Result<Stems, Error> {
    audio.validate()?;
    preset.validate()?;
    if separators.len() != preset.models.len() {
        return Err(Error::Ensemble(EnsembleError::MemberCount(format!(
            "preset `{}` has {} models but {} separators were loaded",
            preset.id,
            preset.models.len(),
            separators.len()
        ))));
    }
    for name in &preset.stems {
        if name != "vocals" && name != "instrumental" {
            return Err(Error::Ensemble(EnsembleError::InvalidPreset(format!(
                "preset `{}` requests unsupported stem `{}`",
                preset.id, name
            ))));
        }
    }

    let mut member_stems = Vec::with_capacity(separators.len());
    for (separator, member) in separators.iter().zip(preset.models.iter()) {
        let stems = separator.separate(audio)?;
        let target = stems
            .get(&member.role)
            .cloned()
            .unwrap_or_else(|| stems.vocals.clone());
        member_stems.push(target);
    }

    let vocals = ensemble_waveforms(preset.algorithm, preset.weights.as_deref(), &member_stems)?;

    let channels = audio.channels as usize;
    let frames = audio.frames();
    let mut instrumental = vec![0f32; frames * channels];
    for (i, value) in instrumental.iter_mut().enumerate() {
        let vocal = vocals.planar.get(i).copied().unwrap_or(0.0);
        *value = audio.planar.get(i).copied().unwrap_or(0.0) - vocal;
    }
    let instrumental = AudioBuffer {
        sample_rate: audio.sample_rate,
        channels: audio.channels,
        planar: instrumental,
    };

    let mut stems = Vec::with_capacity(preset.stems.len());
    for name in &preset.stems {
        let buffer = if name == "vocals" {
            vocals.clone()
        } else {
            instrumental.clone()
        };
        stems.push((name.clone(), buffer));
    }
    Ok(Stems {
        vocals,
        instrumental,
        stems,
    })
}
