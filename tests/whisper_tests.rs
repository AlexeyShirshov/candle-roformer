//! Whisper layer tests.
//!
//! The real checkpoints are not present in this environment, so the end-to-end
//! test builds a tiny randomly-initialised Whisper model with a hand-written
//! tokenizer. That proves the loading, mel, decoding, timestamp-rule and
//! segment-assembly plumbing executes; it says nothing about transcription
//! quality (see `docs/gates-report.md`, Whisper gate).

use std::fs;
use std::path::{Path, PathBuf};

use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use candle_roformer::wav::AudioBuffer;
use candle_roformer::whisper::{TranscribeOptions, Transcriber, SAMPLE_RATE};
use candle_roformer::{DeviceChoice, LocalWhisperAssets, Transcript, TranscriptSegment};

fn tmpdir(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cr_whisper_{}", name));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

/// A minimal Whisper-shaped tokenizer: three text tokens, the required special
/// tokens, and 20 timestamp tokens. Text ids are below `timestamp_begin`, as in
/// the real tokenizer.
fn write_tokenizer(dir: &Path) -> PathBuf {
    use serde_json::json;

    let mut added: Vec<(u32, String)> = vec![
        (3, "<|endoftext|>".to_string()),
        (4, "<|startoftranscript|>".to_string()),
        (5, "<|transcribe|>".to_string()),
        (6, "<|nocaptions|>".to_string()),
        (7, "<|notimestamps|>".to_string()),
    ];
    for i in 0..20u32 {
        added.push((8 + i, format!("<|{:.2}|>", i as f32 / 50.0)));
    }
    let added_tokens: Vec<_> = added
        .iter()
        .map(|(id, content)| {
            json!({
                "id": id,
                "content": content,
                "single_word": false,
                "lstrip": false,
                "rstrip": false,
                "normalized": false,
                "special": true
            })
        })
        .collect();

    let tokenizer = json!({
        "version": "1.0",
        "truncation": null,
        "padding": null,
        "added_tokens": added_tokens,
        "normalizer": null,
        "pre_tokenizer": {"type": "Whitespace"},
        "post_processor": null,
        "decoder": null,
        "model": {
            "type": "WordLevel",
            "vocab": {"<unk>": 0, "hello": 1, "world": 2},
            "unk_token": "<unk>"
        }
    });
    let path = dir.join("tokenizer.json");
    fs::write(&path, serde_json::to_string(&tokenizer).unwrap()).unwrap();
    path
}

/// A tiny model with the same tensor layout as a real Whisper checkpoint.
fn write_model(dir: &Path) -> (PathBuf, PathBuf) {
    use candle_transformers::models::whisper::{model::Whisper, Config};

    let config = Config {
        num_mel_bins: 80,
        max_source_positions: 1500,
        d_model: 16,
        encoder_attention_heads: 2,
        encoder_layers: 1,
        vocab_size: 28,
        max_target_positions: 30,
        decoder_attention_heads: 2,
        decoder_layers: 1,
        suppress_tokens: vec![],
    };
    let config_path = dir.join("config.json");
    let config_json = serde_json::json!({
        "num_mel_bins": 80,
        "max_source_positions": 1500,
        "d_model": 16,
        "encoder_attention_heads": 2,
        "encoder_layers": 1,
        "vocab_size": 28,
        "max_target_positions": 30,
        "decoder_attention_heads": 2,
        "decoder_layers": 1,
        "suppress_tokens": []
    });
    fs::write(&config_path, serde_json::to_string(&config_json).unwrap()).unwrap();

    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let _ = Whisper::load(&vb, config).unwrap();
    let model_path = dir.join("model.safetensors");
    varmap.save(&model_path).unwrap();
    (config_path, model_path)
}

fn synth_assets(name: &str) -> LocalWhisperAssets {
    let dir = tmpdir(name);
    let config = write_model(&dir);
    let tokenizer = write_tokenizer(&dir);
    LocalWhisperAssets::new(config.0, tokenizer, config.1)
}

#[test]
fn transcriber_loads_and_runs_end_to_end() {
    let assets = synth_assets("e2e");
    let mut transcriber = Transcriber::load(&assets, DeviceChoice::Cpu).unwrap();
    assert!(!transcriber.is_multilingual());
    assert_eq!(transcriber.config().num_mel_bins, 80);

    let frames = SAMPLE_RATE as usize; // one second
    let seconds = 1.0;
    let planar: Vec<f32> = (0..frames).map(|i| (i as f32 * 0.01).sin() * 0.1).collect();
    let audio = AudioBuffer {
        sample_rate: SAMPLE_RATE,
        channels: 1,
        planar,
    };

    let transcript = transcriber.transcribe(&audio).unwrap();
    assert_eq!(transcript.language.as_deref(), Some("en"));

    let mut previous_end = 0.0f64;
    for segment in &transcript.segments {
        assert!(segment.start_seconds.is_finite());
        assert!(segment.end_seconds.is_finite());
        assert!(segment.start_seconds >= 0.0);
        assert!(segment.end_seconds >= segment.start_seconds);
        assert!(segment.end_seconds <= seconds + 1e-9);
        assert!(segment.start_seconds >= previous_end - 1e-9);
        previous_end = segment.end_seconds;
        assert!(serde_json::to_string(segment)
            .unwrap()
            .contains("start_seconds"));
    }
    // The random model produces arbitrary text; the only contract here is
    // that the assembled text is valid UTF-8 and does not panic.
    let _ = transcript.text();

    // An English-only model must refuse an explicit foreign language.
    let options = TranscribeOptions {
        language: Some("ru".to_string()),
        ..Default::default()
    };
    let err = transcriber.transcribe_with(&audio, &options).unwrap_err();
    assert!(err.to_string().contains("English-only"));
}

#[test]
fn transcriber_rejects_missing_assets() {
    let assets = LocalWhisperAssets::new(
        "/nope/config.json",
        "/nope/tokenizer.json",
        "/nope/model.safetensors",
    );
    let err = Transcriber::load(&assets, DeviceChoice::Cpu)
        .err()
        .expect("missing assets must fail");
    assert!(err.to_string().contains("not found"));
}

#[test]
fn transcript_serializes_with_stable_field_names() {
    let transcript = Transcript {
        language: Some("ru".to_string()),
        segments: vec![TranscriptSegment {
            start_seconds: 1.0,
            end_seconds: 2.5,
            text: "привет".to_string(),
        }],
    };
    let json = serde_json::to_string(&transcript).unwrap();
    assert!(json.contains(r#""language":"ru""#));
    assert!(json.contains(r#""start_seconds":1.0"#));
    assert!(json.contains(r#""end_seconds":2.5"#));
    assert!(json.contains(r#""text":"привет""#));
    assert_eq!(transcript.text(), "привет");
    assert_eq!(transcript.end_seconds(), 2.5);
}
