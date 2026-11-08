//! File orchestration. This is the thin I/O layer on top of [`Separator`]:
//! decode an input file, hand the buffers to the library, write the stems.
//!
//! Programs that already hold audio in memory should use [`Separator`]
//! directly instead of this module.

use std::fs;
use std::path::Path;

use crate::error::{AudioError, Error};
use crate::separator::Separator;
use crate::wav::{read_audio, write_wav_f32, AudioBuffer};

/// Output file names (relative to the output directory).
pub struct OutputNames<'a> {
    pub vocals: &'a str,
    pub instrumental: Option<&'a str>,
}

/// Separate a file and write `vocals.wav` / `instrumental.wav`.
#[allow(clippy::too_many_arguments)]
pub fn run_pipeline(
    model_path: &str,
    config_path: &str,
    input_path: &str,
    output_dir: &str,
    device: crate::device::DeviceChoice,
    overwrite: bool,
    start_seconds: Option<f64>,
    limit_seconds: Option<f64>,
) -> Result<(), Error> {
    run_pipeline_named(
        model_path,
        config_path,
        input_path,
        output_dir,
        device,
        overwrite,
        start_seconds,
        limit_seconds,
        &OutputNames {
            vocals: "vocals.wav",
            instrumental: Some("instrumental.wav"),
        },
    )
}

/// Separate a file with explicit output names.
#[allow(clippy::too_many_arguments)]
pub fn run_pipeline_named(
    model_path: &str,
    config_path: &str,
    input_path: &str,
    output_dir: &str,
    device: crate::device::DeviceChoice,
    overwrite: bool,
    start_seconds: Option<f64>,
    limit_seconds: Option<f64>,
    names: &OutputNames,
) -> Result<(), Error> {
    let assets = crate::assets::LocalSeparationAssets::new(model_path, config_path);
    let separator = Separator::load(&assets, device)?;

    let mut audio = read_audio(input_path)?;
    trim_window(&mut audio, start_seconds, limit_seconds)?;
    let stems = separator.separate(&audio)?;

    write_stems(output_dir, names, &stems, overwrite)
}

/// Trim a decoded buffer to a `[start, start + duration)` window. Buffers are
/// channel-major, so both ends are cut per channel.
pub fn trim_window(
    audio: &mut AudioBuffer,
    start_seconds: Option<f64>,
    limit_seconds: Option<f64>,
) -> Result<(), Error> {
    let ch = audio.channels as usize;
    let frames = audio.planar.len() / ch;
    if let Some(start) = start_seconds {
        if start < 0.0 || !start.is_finite() {
            return Err(Error::Audio(AudioError::InvalidSampleRate(
                "invalid start offset".into(),
            )));
        }
        let skip = ((start * audio.sample_rate as f64).round() as usize).min(frames);
        if skip >= frames {
            return Err(Error::Audio(AudioError::EmptyInput(
                "start offset is beyond the input length".into(),
            )));
        }
        let mut cut = Vec::with_capacity(audio.planar.len() - skip * ch);
        for c in 0..ch {
            cut.extend_from_slice(&audio.planar[c * frames + skip..(c + 1) * frames]);
        }
        audio.planar = cut;
    }
    if let Some(secs) = limit_seconds {
        if secs <= 0.0 || !secs.is_finite() {
            return Err(Error::Audio(AudioError::InvalidSampleRate(
                "invalid duration".into(),
            )));
        }
        let frames = audio.planar.len() / ch;
        let want = ((secs * audio.sample_rate as f64).round() as usize).min(frames);
        if want == 0 {
            return Err(Error::Audio(AudioError::EmptyInput(
                "duration trims the whole input".into(),
            )));
        }
        let mut cut = Vec::with_capacity(want * ch);
        for c in 0..ch {
            cut.extend_from_slice(&audio.planar[c * frames..c * frames + want]);
        }
        audio.planar = cut;
    }
    Ok(())
}

/// Write stems through staging files: both are written first, then renamed
/// (one by one, not a multi-file atomic transaction). On failure this removes
/// only its own staging files.
pub fn write_stems(
    output_dir: &str,
    names: &OutputNames,
    stems: &crate::separator::Stems,
    overwrite: bool,
) -> Result<(), Error> {
    let out_dir = Path::new(output_dir);
    fs::create_dir_all(out_dir).map_err(|e| Error::Audio(AudioError::Io(e.to_string())))?;

    let vocals_path = out_dir.join(names.vocals);
    let instr_path = names.instrumental.map(|n| out_dir.join(n));

    if !overwrite
        && (vocals_path.exists()
            || instr_path.as_ref().map(|p| p.exists()).unwrap_or(false))
    {
        return Err(Error::Audio(AudioError::Io(
            "output file exists; use --overwrite".into(),
        )));
    }

    let staging_v = out_dir.join(format!(".{}.tmp", names.vocals));
    let staging_i = names
        .instrumental
        .map(|n| out_dir.join(format!(".{}.tmp", n)));

    if let Err(e) = write_wav_f32(&staging_v, &stems.vocals) {
        let _ = fs::remove_file(&staging_v);
        return Err(e);
    }
    if let Some(instr_path) = &instr_path {
        let staging_i = staging_i.as_ref().expect("staging path for instrumental");
        if let Err(e) = write_wav_f32(staging_i, &stems.instrumental) {
            let _ = fs::remove_file(&staging_v);
            let _ = fs::remove_file(staging_i);
            return Err(e);
        }
        if let Err(e) = fs::rename(staging_i, instr_path) {
            let _ = fs::remove_file(staging_i);
            return Err(Error::Audio(AudioError::Io(e.to_string())));
        }
    }
    if let Err(e) = fs::rename(&staging_v, &vocals_path) {
        let _ = fs::remove_file(&staging_v);
        return Err(Error::Audio(AudioError::Io(e.to_string())));
    }
    Ok(())
}
