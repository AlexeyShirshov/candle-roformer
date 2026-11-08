//! Timestamp-token math and transcript assembly.
//!
//! Whisper encodes timestamps as vocabulary tokens `<|0.00|>`, `<|0.02|>`, ...
//! at 50 tokens per second. A decoded window is a sequence
//! `[ts_start, text..., ts_end, ...]`; a transcript segment is the text between
//! a start and an end timestamp.

use serde::Serialize;

/// One transcribed span, in seconds on the input track's timeline.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TranscriptSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub text: String,
}

/// Full transcription result.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Transcript {
    /// Detected or requested language code (`ru`, `en`, ...), if known.
    pub language: Option<String>,
    pub segments: Vec<TranscriptSegment>,
}

impl Transcript {
    /// The segment texts joined in order.
    pub fn text(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.text.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// End of the last segment, or zero for an empty transcript.
    pub fn end_seconds(&self) -> f64 {
        self.segments.last().map(|s| s.end_seconds).unwrap_or(0.0)
    }
}

/// Seconds encoded by a timestamp token. `timestamp_begin` is the id of
/// `<|0.00|>`, so `<|0.02|>` maps to `0.02` and so on.
pub fn timestamp_seconds(token: u32, timestamp_begin: u32) -> f64 {
    token.saturating_sub(timestamp_begin) as f64 / 50.0
}

#[allow(clippy::too_many_arguments)]
fn push_segment<F>(
    out: &mut Vec<TranscriptSegment>,
    text_tokens: &mut Vec<u32>,
    start_rel: f64,
    end_rel: f64,
    offset_seconds: f64,
    total_duration_seconds: f64,
    decode: &mut F,
) -> Result<(), String>
where
    F: FnMut(&[u32]) -> Result<String, String>,
{
    if text_tokens.is_empty() {
        return Ok(());
    }
    let text = decode(text_tokens)?.trim().to_string();
    text_tokens.clear();
    if text.is_empty() {
        return Ok(());
    }
    let start = (offset_seconds + start_rel).clamp(0.0, total_duration_seconds);
    let end = (offset_seconds + end_rel).clamp(start, total_duration_seconds.max(start));
    out.push(TranscriptSegment {
        start_seconds: start,
        end_seconds: end,
        text,
    });
    Ok(())
}

/// Build segments from one decoded window's content tokens.
///
/// `offset_seconds` is the window's absolute start on the input timeline;
/// timestamps inside the window are relative to it. Segments are clamped to
/// `[0, total_duration_seconds]`, made non-negative in length, and empty texts
/// are dropped. `special` lists prompt/task tokens that must not leak into the
/// text (BOS/EOT/transcribe/notimestamps). `decode` turns token ids into text;
/// its error is propagated as a string.
#[allow(clippy::too_many_arguments)]
pub fn segments_from_tokens<F>(
    tokens: &[u32],
    timestamp_begin: u32,
    offset_seconds: f64,
    window_end_seconds: f64,
    total_duration_seconds: f64,
    special: &[u32],
    mut decode: F,
) -> Result<Vec<TranscriptSegment>, String>
where
    F: FnMut(&[u32]) -> Result<String, String>,
{
    let mut out = Vec::new();
    let mut start_rel = 0.0f64;
    let mut text_tokens: Vec<u32> = Vec::new();

    for &token in tokens {
        if special.contains(&token) {
            continue;
        }
        if token >= timestamp_begin {
            let ts = timestamp_seconds(token, timestamp_begin);
            push_segment(
                &mut out,
                &mut text_tokens,
                start_rel,
                ts,
                offset_seconds,
                total_duration_seconds,
                &mut decode,
            )?;
            start_rel = ts;
        } else {
            text_tokens.push(token);
        }
    }
    // Text after the last timestamp belongs to the rest of the window.
    push_segment(
        &mut out,
        &mut text_tokens,
        start_rel,
        window_end_seconds - offset_seconds,
        offset_seconds,
        total_duration_seconds,
        &mut decode,
    )?;
    Ok(out)
}

/// True when the next window repeats the previous window's last segment exactly
/// at the shared boundary (a deterministic duplicate-suppression rule for
/// long-form decoding). Genuine repeated lyrics usually differ in timing or are
/// separated by other segments.
pub fn is_boundary_duplicate(previous: &TranscriptSegment, next: &TranscriptSegment) -> bool {
    (previous.end_seconds - next.start_seconds).abs() < 1e-6
        && !previous.text.is_empty()
        && previous.text == next.text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_decode(ids: &[u32]) -> Result<String, String> {
        Ok(ids
            .iter()
            .map(|id| format!("w{}", id))
            .collect::<Vec<_>>()
            .join(" "))
    }

    #[test]
    fn timestamp_tokens_are_20ms_steps() {
        // <|0.00|> is the first timestamp token.
        assert_eq!(timestamp_seconds(1000, 1000), 0.0);
        assert_eq!(timestamp_seconds(1001, 1000), 0.02);
        assert_eq!(timestamp_seconds(1050, 1000), 1.0);
        assert_eq!(timestamp_seconds(1050, 1000), 1.0);
        // Defensive: a non-timestamp id never produces a negative time.
        assert_eq!(timestamp_seconds(999, 1000), 0.0);
    }

    #[test]
    fn assembles_pairs_of_timestamps() {
        // ts(0.0) w1 ts(1.5) w2 ts(3.0)
        let tokens = [1000, 7, 1000 + 75, 8, 1000 + 150];
        let segs = segments_from_tokens(&tokens, 1000, 0.0, 30.0, 30.0, &[], fake_decode).unwrap();
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].start_seconds, 0.0);
        assert_eq!(segs[0].end_seconds, 1.5);
        assert_eq!(segs[0].text, "w7");
        assert_eq!(segs[1].start_seconds, 1.5);
        assert_eq!(segs[1].end_seconds, 3.0);
        assert_eq!(segs[1].text, "w8");
    }

    #[test]
    fn applies_window_offset_and_clamps() {
        let tokens = [1000, 7, 1000 + 50];
        let segs = segments_from_tokens(&tokens, 1000, 28.0, 30.0, 29.0, &[], fake_decode).unwrap();
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].start_seconds, 28.0);
        // 29.0 s clamps to the track duration, never beyond it.
        assert_eq!(segs[0].end_seconds, 29.0);
    }

    #[test]
    fn skips_special_tokens_and_empty_text() {
        let tokens = [4, 1000, 1000 + 50];
        let segs = segments_from_tokens(&tokens, 1000, 0.0, 30.0, 30.0, &[4], fake_decode).unwrap();
        assert!(segs.is_empty());
    }

    #[test]
    fn trailing_text_ends_at_window_end() {
        let tokens = [1000, 7, 8];
        let segs = segments_from_tokens(&tokens, 1000, 0.0, 12.5, 30.0, &[], fake_decode).unwrap();
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].start_seconds, 0.0);
        assert_eq!(segs[0].end_seconds, 12.5);
    }

    #[test]
    fn propagates_decode_errors() {
        let tokens = [1000, 7, 1000 + 50];
        let err = segments_from_tokens(&tokens, 1000, 0.0, 30.0, 30.0, &[], |_| {
            Err("boom".to_string())
        })
        .unwrap_err();
        assert_eq!(err, "boom");
    }
}
