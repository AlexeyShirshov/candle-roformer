# Unwa BS-RoFormer HyperACE v2 Inst

## Reference and assets

- Author: Unwa (`pcunwa`).
- Model repository: <https://huggingface.co/pcunwa/BS-Roformer-HyperACE>.
- Pinned revision: `5b1f8283125d5e4a3614d0e3635a636e09c84059`.
- Source: `v2_inst/bs_roformer.py`, Git blob
  `012630e2deba7f0d4bad5de69c1833d54469354b`.
- Profile: `v2_inst/config.yaml`.
- Checkpoint: `v2_inst/bs_roformer_inst_hyperacev2.ckpt`,
  288,725,646 bytes, SHA-256
  `4d61178ef966d2b4e9ad456ffbbc6fd5b2828df07a7af32931142c1a5ff1fe6f`.
  Size/hash are published by the pinned Hugging Face LFS metadata.
- No checkpoint or author's Python source is redistributed in this repository.
  The model repository does not declare a licence in its model metadata; this
  project makes no claim about permission to redistribute its weights.

## Implementation

The existing BS-RoFormer trunk and per-band MLP are unchanged. Each mask estimator
adds `segm(x)` to its MLP output. The segmentation head is implemented in
`src/model/hyperace.rs`, retaining the original parameter paths.

- Depthwise-separable backbone with asymmetric time-only strides followed by
  two-dimensional strides.
- Input-statistic affine InstanceNorm, biased spatial variance, epsilon `1e-8`.
- Hyperedge prototypes adjusted by global average/max context; eight heads,
  32 hyperedges, softmax over vertices, residual hypergraph convolution.
- Parallel high/low/skip branches, gated multi-scale decoding.
- Frequency-only shuffle, four upsampling blocks, two TFC/TDF blocks per stage.
- Bilinear resizing with half-pixel coordinates (`align_corners=false`).
- Final channel order is frequency, stereo channel, real/imaginary component.

The original YAML lacks a head selector. The adapted repository profile adds
`family: bs_roformer_hyperace_v2` without changing the author's inference geometry.
`instrument` in its training target denotes instrumental, not vocals.

## Reproduce verification

First export the Rust schema and convert the local checkpoint as in the README.
For the optional numerical comparison install `torch`, `safetensors`, `einops`,
`beartype` and `rotary-embedding-torch` in an isolated Python environment.
Obtain the pinned `v2_inst/bs_roformer.py` locally, then:

```sh
python tools/hyperace_reference.py --source bs_roformer.py \
  --weights hyperace_v2_inst.safetensors --out hyperace-reference.safetensors
HYPERACE_SAFETENSORS=hyperace_v2_inst.safetensors \
HYPERACE_REFERENCE=hyperace-reference.safetensors \
  cargo test --test hyperace -- --ignored --nocapture
```

The generator validates the author's source Git blob before execution. Its only
attention substitution is inference-mode PyTorch scaled-dot-product attention.
The test checks the complete real checkpoint schema, compares the head on both
32- and 33-frame deterministic inputs, then compares an 8,192-sample stereo
forward pass through the full network. Acceptance: maximum absolute error and
relative L2 error below `1e-3`. This is numerical implementation parity on short
fixtures, not an audio-quality benchmark, full-song performance measurement or
GPU validation.

The pinned real checkpoint has passed conversion against the Rust schema:
all 1,170 tensor names and shapes match (699 standard BS tensors plus 471
segmentation-head tensors). Shared rotary storage is cloned without dropping
any alias keys. Converted F32 output SHA-256:
`775175135ce67bbbd68ec8812e9035aa7ac9c3332efdbf743172d4df2a3ae062`.

## Verified CPU result (2026-11-16)

Windows CPU, Candle 0.11.1, F32, PyTorch 2.15.0+cpu,
einops 0.8.2, rotary-embedding-torch 0.9.2. The opt-in
`real_checkpoint_and_head_match_pytorch` test passed in 38.47 s:

| Fixture | Maximum absolute error | RMSE | Relative L2 |
|---|---:|---:|---:|
| Head, 32 frames | 9.9361e-5 | 1.8265e-6 | 2.2582e-5 |
| Head, 33 frames | 9.7275e-5 | 1.8992e-6 | 2.3308e-5 |
| Full stereo forward, 8,192 samples | 7.5303e-5 | 2.7025e-5 | 3.8028e-4 |

The same test successfully loaded the checkpoint through the public
`Separator::load` using the repository profile. Unit tests also cover biased
per-sample InstanceNorm and frequency-shuffle ordering. Converter tests cover
wrong hashes/sizes, missing/extra/reshaped tensors and shared rotary storage.

This is short-fixture CPU verification only. No full-song performance, separation
quality benchmark, CUDA or Metal execution claim is made for HyperACE.
