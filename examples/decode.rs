use candle_roformer::wav::read_audio;
fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap();
    let start: Option<f64> = args.next().and_then(|v| v.parse().ok());
    let dur: Option<f64> = args.next().and_then(|v| v.parse().ok());
    match read_audio(&path) {
        Ok(b) => {
            let mut planar = b.planar.clone();
            let ch = b.channels as usize;
            if let Some(s) = start {
                let frames = planar.len() / ch;
                let skip = ((s * b.sample_rate as f64).round() as usize).min(frames);
                let full = planar.clone();
                for c in (0..ch).rev() { planar.drain(c * frames..c * frames + skip); }
                let new_frames = planar.len() / ch;
                for c in 0..ch {
                    let a = &planar[c * new_frames..(c + 1) * new_frames];
                    let bb = &full[c * frames + skip..(c + 1) * frames];
                    let mut dot = 0f64; let mut na = 0f64; let mut nb = 0f64;
                    for (x, y) in a.iter().zip(bb.iter()) { dot += *x as f64 * *y as f64; na += (*x as f64).powi(2); nb += (*y as f64).powi(2); }
                    println!("trim ch{} corr {}", c, dot / (na.sqrt() * nb.sqrt()));
                }
            }
            if let Some(d) = dur {
                let want = ((d * b.sample_rate as f64).round() as usize).min(planar.len() / ch);
                planar.truncate(want * ch);
            }
            let frames = planar.len() / ch;
            println!("channels={} rate={} frames={}", ch, b.sample_rate, frames);
            let mut bytes = Vec::with_capacity(planar.len() * 4);
            for x in &planar { bytes.extend_from_slice(&x.to_le_bytes()); }
            std::fs::write("/tmp/opencode/example_planar.f32", bytes).unwrap();
            std::fs::write("/tmp/opencode/example_planar.shape", format!("{},{}", ch, frames)).unwrap();
        }
        Err(e) => println!("error: {}", e),
    }
}
