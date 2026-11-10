use std::fs;
use std::io::Write;
use candle_roformer::config::SeparatorConfig;
use candle_roformer::wav::{write_wav_f32, AudioBuffer};
use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};

fn tiny_config() -> SeparatorConfig {
    SeparatorConfig {
        dim: 8, depth: 1, stereo: true, num_stems: 1, num_bands: 4,
        dim_head: 4, heads: 1, flash_attn: false, dim_freqs_in: 9,
        sample_rate: 44100, stft_n_fft: 16, stft_hop_length: 4,
        stft_win_length: 16, stft_normalized: false, mask_estimator_depth: 2,
        use_torch_checkpoint: false, time_transformer_depth: 1,
        freq_transformer_depth: 1, mlp_expansion_factor: 4,
        linear_transformer_depth: 0, norm_output: true, skip_connection: false,
        zero_dc: true,
    }
}

fn vmhwm_kib() -> u64 {
    let s = fs::read_to_string("/proc/self/status").unwrap_or_default();
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if let Some(num) = parts.first() { return num.parse().unwrap_or(0); }
        }
    }
    0
}

fn sha256_file(path: &std::path::Path) -> String {
    use std::process::Command;
    let out = Command::new("sha256sum").arg(path).output().unwrap();
    String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string()
}

#[test]
fn rss() {
    let mode = std::env::var("MODE").unwrap_or_else(|_| "measure".into());
    let dur: f64 = std::env::var("DUR").unwrap_or_else(|_| "10".into()).parse().unwrap();
    let cd: f64 = std::env::var("CD").unwrap_or_else(|_| "0.07256".into()).parse().unwrap();
    let model = std::env::var("MODEL").unwrap_or_else(|_| "/tmp/opencode/tiny_model/tiny.safetensors".into());
    let fixture = std::env::var("FIXTURE").expect("FIXTURE env var required");
    let fixture_path = std::path::PathBuf::from(&fixture);

    if mode == "gen" {
        // Generate fixture; fail if already exists? No, overwrite.
        let sr = 44100usize;
        let frames = (dur * sr as f64).round() as usize;
        let mut planar = vec![0f32; 2 * frames];
        for i in 0..frames {
            let s = (i as f32 * 0.037).sin() * 0.3;
            planar[i] = s;
            planar[frames + i] = s * 0.5;
        }
        let audio = AudioBuffer { sample_rate: sr as u32, channels: 2, planar };
        write_wav_f32(&fixture_path, &audio).unwrap();
        // metadata sidecar
        let meta_path = fixture_path.with_extension("meta.txt");
        let mut f = fs::File::create(&meta_path).unwrap();
        writeln!(f, "duration_s={}", dur).unwrap();
        writeln!(f, "sample_rate={}", sr).unwrap();
        writeln!(f, "channels=2").unwrap();
        writeln!(f, "frames={}", frames).unwrap();
        writeln!(f, "samples={}", frames * 2).unwrap();
        writeln!(f, "sha256={}", sha256_file(&fixture_path)).unwrap();
        eprintln!("GEN_RESULT DUR={} FRAMES={} SAMPLES={} SHA256={} FIXTURE={}", dur, frames, frames * 2, sha256_file(&fixture_path), fixture_path.display());
        return;
    }

    // measure mode: fail loudly if fixture missing
    if !fixture_path.exists() {
        panic!("FIXTURE file missing: {} (measure mode requires pre-generated file)", fixture_path.display());
    }
    let fixture_sha = sha256_file(&fixture_path);

    let ckpt = std::path::PathBuf::from(&model);
    let assets = LocalSeparationAssets::new(&ckpt, ckpt.with_extension("yaml"));
    let separator = Separator::load(&assets, DeviceChoice::Cpu).unwrap();

    let dir = std::env::temp_dir().join(format!("rssmeas_{}", dur));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let vpath = dir.join("v.wav");
    let ipath = dir.join("i.wav");

    let start = std::time::Instant::now();
    separator.separate_file(&fixture_path, &vpath, &ipath, cd).unwrap();
    let wall = start.elapsed();

    eprintln!("MEASURE_RESULT DUR={} CD={} HWM_KIB={} HWM_MIB={:.2} WALL_S={:.2} FIXTURE_SHA256={} MODEL={} EXIT=0", dur, cd, vmhwm_kib(), vmhwm_kib() as f64 / 1024.0, wall.as_secs_f64(), fixture_sha, model);
}

