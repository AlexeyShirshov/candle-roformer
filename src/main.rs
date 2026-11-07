use std::env;
use std::process;

use candle_roformer::device::DeviceChoice;
use candle_roformer::pipeline::run_pipeline;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("candle-roformer {}", VERSION);
    println!("Usage:");
    println!("  candle-roformer separate --model <safetensors> --config <v7.yaml> --input <file> --output <dir>
                          [--device cpu|cuda|metal] [--overwrite] [--duration <seconds>]");
    println!("  candle-roformer --version");
    println!("  candle-roformer --help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print_help();
        process::exit(0);
    }
    match args[1].as_str() {
        "--version" | "-V" => {
            println!("{}", VERSION);
            process::exit(0);
        }
        "--help" | "-h" => {
            print_help();
            process::exit(0);
        }
        "separate" => {
            let mut model = None;
            let mut config = None;
            let mut input = None;
            let mut output = None;
            let mut device = "cpu".to_string();
            let mut overwrite = false;
            let mut duration: Option<f64> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--model" => { model = Some(args[i+1].clone()); i += 2; }
                    "--config" => { config = Some(args[i+1].clone()); i += 2; }
                    "--input" => { input = Some(args[i+1].clone()); i += 2; }
                    "--output" => { output = Some(args[i+1].clone()); i += 2; }
                    "--device" => { device = args[i+1].clone(); i += 2; }
                    "--overwrite" => { overwrite = true; i += 1; }
                    "--duration" => {
                        duration = Some(match args[i+1].parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => { eprintln!("bad --duration"); process::exit(2); }
                        });
                        i += 2;
                    }
                    _ => {
                        eprintln!("unknown arg: {}", args[i]);
                        process::exit(2);
                    }
                }
            }
            let model = match model { Some(v) => v, None => { eprintln!("--model required"); process::exit(2); } };
            let config = match config { Some(v) => v, None => { eprintln!("--config required"); process::exit(2); } };
            let input = match input { Some(v) => v, None => { eprintln!("--input required"); process::exit(2); } };
            let output = match output { Some(v) => v, None => { eprintln!("--output required"); process::exit(2); } };
            let dev = match device.as_str() {
                "cpu" => DeviceChoice::Cpu,
                "cuda" => DeviceChoice::Cuda,
                "metal" => DeviceChoice::Metal,
                _ => { eprintln!("bad device"); process::exit(2); }
            };
            if let Err(e) = run_pipeline(&model, &config, &input, &output, dev, overwrite, duration) {
                eprintln!("error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("unknown command: {}", args[1]);
            print_help();
            process::exit(2);
        }
    }
}
