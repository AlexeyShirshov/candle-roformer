//! Transcribe an audio file with a local Whisper checkpoint.
//!
//! ```sh
//! cargo run --release --example transcribe -- \
//!   /path/to/audio.wav /path/to/whisper-dir [language]
//! ```
//!
//! `whisper-dir` must contain `config.json`, `tokenizer.json` and
//! `model.safetensors`. No network access is performed by this example.

use candle_roformer::wav::read_audio;
use candle_roformer::whisper::{TranscribeOptions, Transcriber};
use candle_roformer::{DeviceChoice, LocalWhisperAssets};

fn main() {
    let mut args = std::env::args().skip(1);
    let audio = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("usage: transcribe <audio> <whisper-dir> [language]");
            std::process::exit(2);
        }
    };
    let dir = match args.next() {
        Some(dir) => std::path::PathBuf::from(dir),
        None => {
            eprintln!("usage: transcribe <audio> <whisper-dir> [language]");
            std::process::exit(2);
        }
    };
    let language = args.next();
    let assets = LocalWhisperAssets::new(
        dir.join("config.json"),
        dir.join("tokenizer.json"),
        dir.join("model.safetensors"),
    );

    let mut transcriber = match Transcriber::load(&assets, DeviceChoice::Cpu) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let audio = match read_audio(&audio) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let options = TranscribeOptions {
        language,
        ..Default::default()
    };
    match transcriber.transcribe_with(&audio, &options) {
        Ok(transcript) => {
            println!("language: {:?}", transcript.language);
            for segment in &transcript.segments {
                println!(
                    "[{:7.2} -> {:7.2}] {}",
                    segment.start_seconds, segment.end_seconds, segment.text
                );
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
