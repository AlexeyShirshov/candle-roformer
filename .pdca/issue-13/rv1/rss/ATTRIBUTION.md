# RSS attribution (issue #13, cycle N=2)

Measurement: synthetic tiny model, CPU, chunk_duration = 0.07256 s, stereo 44100 Hz,
input read from a pre-generated file, each duration in a separate process,
VmHWM from /proc/self/status. Release profile.

Attribution experiments:
- fixture generation only (genonly), 120 s: VmHWM = 47.02 MiB (whole-fixture allocation in process)
- model load + config only (loadonly), 120 s: VmHWM = 9.88 MiB
- separate_file on pre-generated file (separate):
  - 10 s  -> 17.35 MiB
  - 30 s  -> 18.80 MiB
  - 60 s  -> 18.51 MiB
  - 120 s -> 17.52 MiB

Conclusion: the earlier 25.09/38.76/58.57/97.74 MiB series was dominated by the
harness generating the fixture in-memory in the same process (O(T) allocation),
not by the product. separate_file itself is bounded: 10->120 s ratio = 1.01.

Code path (src/separator.rs):
- L597 AudioStreamReader::open: streaming read, O(chunk) buffered pending.
- L608 fallback (sr mismatch / multi-stem / invert_spect): read_audio whole input -> O(T) RAM, documented non-bounded.
- L638-657 pass 0 peak pre-pass: reads 65536-frame chunks, tracks peak0/total_frames, no storage -> O(chunk) RAM, O(T) time.
- L663-668 amp/norm scale: scalars.
- L670-681 temp files vraw/iraw: disk-backed -> O(T) disk, O(chunk) RAM.
- L683-684 peak_v/peak_i: scalars.
- L686-894 pass 1 OLA: model_buf/orig_buf FrameBuf (bounded chunk+step), acc_v (channels*capacity), acc_w (capacity), pcm (channels*chunk), fade_w (chunk) -> O(chunk) RAM; writes temp files frame-by-frame.
- L902-951 pass 2: reads temp files in 4096-frame chunks, writes WAV -> O(chunk) RAM.
- FrameBuf::append (L530): transient O(old+new) merge, bounded by chunk.

Final table (release, pre-generated fixture):
| input length | VmHWM (MiB) |
|---|---|
| 10 s  | 17.35 |
| 30 s  | 18.80 |
| 60 s  | 18.51 |
| 120 s | 17.52 |

Ratio 10->120 s = 17.52/17.35 = 1.01 <= 1.15. M1 holds. No product change needed.

---
Erratum (dated 2026-11-10, CHECK->DO correction, n=2/3, defect 13-D7): the
reproduction-grade series (pre-generated fixtures, one separate process per
duration, VmHWM = /usr/bin/time max RSS) is authoritative:
| input length | VmHWM (KiB) | MiB |
|---|---|---|
| 10 s  | 17476 | 17.07 |
| 30 s  | 17420 | 17.01 |
| 60 s  | 18964 | 18.52 |
| 120 s | 18820 | 18.38 |
Ratio 10->120 s = 18820/17476 = 1.077 <= 1.15. M1 still holds. The earlier
series is retained as prior run history with the harness-generation caveat; only
the reproduction series is authoritative. See rss/run_*.log and rss/measurements.tsv.
