use std::fs;
use std::path::PathBuf;

use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};
use candle_roformer::wav::{write_wav_f32, AudioBuffer};

fn vmhwm_kib() -> u64 {
    let s = fs::read_to_string("/proc/self/status").unwrap_or_default();
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if let Some(num) = parts.first() {
                return num.parse().unwrap_or(0);
            }
        }
    }
    0
}

fn main() {
    eprintln!("RSS_MEASURE_START");
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 4 {
        eprintln!("usage: rss_measure <model.safetensors> <duration_s> <chunk_duration_s> <out_dir>");
        std::process::exit(2);
    }
    let ckpt = PathBuf::from(&args[0]);
    let dur: f64 = args[1].parse().unwrap();
    let cd: f64 = args[2].parse().unwrap();
    let out_dir = PathBuf::from(&args[3]);
    let _ = fs::create_dir_all(&out_dir);

    // Generate synthetic stereo sine input of given duration at 44100 Hz.
    let sr = 44100usize;
    let frames = (dur * sr as f64).round() as usize;
    let mut planar = vec![0f32; 2 * frames];
    for i in 0..frames {
        let s = (i as f32 * 0.037).sin() * 0.3;
        planar[i] = s;
        planar[frames + i] = s * 0.5;
    }
    let audio = AudioBuffer { sample_rate: sr as u32, channels: 2, planar: planar.clone() };
    let input_path = out_dir.join(format!("in_{}.wav", dur as u64));
    write_wav_f32(&input_path, &audio).unwrap();

    let assets = LocalSeparationAssets::new(&ckpt, ckpt.with_extension("yaml"));
    let separator = match Separator::load(&assets, DeviceChoice::Cpu) {
        Ok(s) => s,
        Err(e) => { eprintln!("load error: {}", e); std::process::exit(1); }
    };

    let vpath = out_dir.join(format!("v_{}.wav", dur as u64));
    let ipath = out_dir.join(format!("i_{}.wav", dur as u64));
    if let Err(e) = separator.separate_file(&input_path, &vpath, &ipath, cd) {
        eprintln!("sep error: {}", e);
        std::process::exit(1);
    }

    let hwm = vmhwm_kib();
    println!("DURATION={} HWM_KIB={} HWM_MIB={:.2}", dur, hwm, hwm as f64 / 1024.0);
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&vpath);
    let _ = fs::remove_file(&ipath);
}

