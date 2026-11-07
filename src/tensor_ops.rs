use candle_core::{Device, Tensor};

use crate::error::DspError;

pub fn gather_1d(input: &Tensor, indices: &[usize]) -> Result<Tensor, candle_core::Error> {
    let idx = Tensor::from_vec(
        indices.iter().map(|&x| x as i64).collect::<Vec<_>>(),
        indices.len(),
        input.device(),
    )?;
    input.gather(&idx, 0)
}

pub fn scatter_add_1d(
    out_len: usize,
    source: &Tensor,
    indices: &[usize],
    device: &Device,
) -> Result<Tensor, candle_core::Error> {
    let base = Tensor::zeros(out_len, source.dtype(), device)?;
    let idx = Tensor::from_vec(
        indices.iter().map(|&x| x as i64).collect::<Vec<_>>(),
        indices.len(),
        device,
    )?;
    base.scatter_add(&idx, source, 0)
}

pub fn average_by_counts(values: &Tensor, counts: &Tensor) -> Result<Tensor, candle_core::Error> {
    values.div(counts)
}

// Keep a typed alias for callers that prefer crate-level errors.
pub fn gather_1d_checked(input: &Tensor, indices: &[usize]) -> Result<Tensor, DspError> {
    gather_1d(input, indices).map_err(|e| DspError::ShapeMismatch(e.to_string()))
}
