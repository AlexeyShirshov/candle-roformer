use std::fs::File;
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::error::{AudioError, Error};

#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    pub channels: u16,
    pub planar: Vec<f32>,
}

pub fn read_audio(path: impl AsRef<Path>) -> Result<AudioBuffer, Error> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| Error::Audio(AudioError::Io(e.to_string())))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| Error::Audio(AudioError::UnsupportedFormat(e.to_string())))?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| Error::Audio(AudioError::UnsupportedFormat("no audio track".into())))?;

    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let sample_rate = codec_params
        .sample_rate
        .ok_or_else(|| Error::Audio(AudioError::InvalidSampleRate("missing sample rate".into())))?;
    if sample_rate == 0 || sample_rate > 384000 {
        return Err(Error::Audio(AudioError::InvalidSampleRate(format!(
            "absurd sample rate {}",
            sample_rate
        ))));
    }
    let channels = codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(1);
    if channels == 0 || channels > 2 {
        return Err(Error::Audio(AudioError::TooManyChannels(format!(
            "unsupported channel count {}",
            channels
        ))));
    }

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| Error::Audio(AudioError::UnsupportedFormat(e.to_string())))?;

    let mut planar: Vec<Vec<f32>> = vec![Vec::new(); channels as usize];

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(symphonia::core::errors::Error::ResetRequired) => {
                // reset decoder
                decoder.reset();
                continue;
            }
            Err(e) => {
                // end of stream or error
                if format!("{}", e).contains("end of stream") {
                    break;
                }
                return Err(Error::Audio(AudioError::UnsupportedFormat(e.to_string())));
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(Error::Audio(AudioError::UnsupportedFormat(e.to_string()))),
        };

        let spec = *decoded.spec();
        let _ = spec;
        let ch = decoded.spec().channels.count();
        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        sample_buf.copy_interleaved_ref(decoded);
        let samples = sample_buf.samples();
        // interleave -> planar
        let frames = samples.len() / ch;
        for f in 0..frames {
            for c in 0..channels as usize {
                let v = samples[f * ch + c];
                if !v.is_finite() {
                    return Err(Error::Audio(AudioError::NonFinite(
                        "non-finite sample".into(),
                    )));
                }
                planar[c].push(v);
            }
        }
    }

    // flatten planar
    let mut flat = Vec::new();
    for c in 0..channels as usize {
        if planar[c].is_empty() {
            // empty input
        }
        flat.extend_from_slice(&planar[c]);
    }
    // total frames
    let frames = planar[0].len();
    if frames == 0 {
        return Err(Error::Audio(AudioError::EmptyInput("empty audio".into())));
    }
    for c in 0..channels as usize {
        if planar[c].len() != frames {
            return Err(Error::Audio(AudioError::UnsupportedFormat(
                "channel length mismatch".into(),
            )));
        }
    }

    // rebuild planar as ch-major contiguous (already)
    let mut planar_flat = Vec::with_capacity(channels as usize * frames);
    for c in 0..channels as usize {
        planar_flat.extend_from_slice(&planar[c]);
    }

    Ok(AudioBuffer {
        sample_rate,
        channels,
        planar: planar_flat,
    })
}

pub fn write_wav_f32(path: impl AsRef<Path>, buf: &AudioBuffer) -> Result<(), Error> {
    use std::io::Write;
    let path = path.as_ref();
    let channels = buf.channels as u16;
    let sr = buf.sample_rate;
    let bits_per_sample: u16 = 32;
    let byte_rate = sr * channels as u32 * (bits_per_sample as u32 / 8);
    let block_align = channels * (bits_per_sample / 8);
    let data_size = (buf.planar.len() * 4) as u32;
    let file_size = 36 + data_size;
    let channels_usize = channels as usize;
    let frames = buf.planar.len() / channels_usize;

    let mut f = File::create(path)
        .map_err(|e| Error::Audio(AudioError::Io(e.to_string())))?;
    // RIFF header
    f.write_all(b"RIFF").unwrap();
    f.write_all(&file_size.to_le_bytes()).unwrap();
    f.write_all(b"WAVE").unwrap();
    // fmt chunk
    f.write_all(b"fmt ").unwrap();
    f.write_all(&16u32.to_le_bytes()).unwrap(); // chunk size
    f.write_all(&3u16.to_le_bytes()).unwrap(); // audio format IEEE float
    f.write_all(&channels.to_le_bytes()).unwrap();
    f.write_all(&sr.to_le_bytes()).unwrap();
    f.write_all(&byte_rate.to_le_bytes()).unwrap();
    f.write_all(&block_align.to_le_bytes()).unwrap();
    f.write_all(&bits_per_sample.to_le_bytes()).unwrap();
    // data chunk
    f.write_all(b"data").unwrap();
    f.write_all(&data_size.to_le_bytes()).unwrap();
    // WAV data is interleaved: frame-major, channel-minor.
    for frame in 0..frames {
        for c in 0..channels_usize {
            let s = buf.planar[c * frames + frame];
            if !s.is_finite() {
                return Err(Error::Audio(AudioError::NonFinite("non-finite output".into())));
            }
            f.write_all(&s.to_le_bytes()).unwrap();
        }
    }
    Ok(())
}
