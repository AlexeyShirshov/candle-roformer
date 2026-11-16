## Model compatibility

- Add Unwa BS-RoFormer HyperACE v2 Inst, including its segmentation/hypergraph mask-estimator head in Rust/Candle.
- Add the HyperACE profile, strict tensor schema export, hash-pinned offline converter and reproducible PyTorch comparison tools.
- Add Unwa MelBand Inst v1e Plus profile and offline converter.
- Correct vocal/instrumental assignment for instrumental-target profiles in both in-memory and streaming separation.
- Honor published `audio.chunk_size` and tolerate Python tuple tags in inference profile parsing.

## Usage

HyperACE profile: `configs/bs_roformer_hyperace_v2_inst.yaml`.
Convert the original `.ckpt` once with `tools/convert_hyperace.py`; the runtime still loads `.safetensors`, not PyTorch pickle.
See `docs/hyperace-v2.md` in the repository for conversion and verification instructions.

## Verification and limits

- Real HyperACE checkpoint: all 1,170 tensor names/shapes validated.
- Real-weight CPU comparison against the pinned author's PyTorch code passed for the head and full short stereo forward pass; full-forward max absolute error `7.53e-5`.
- Targeted Rust and converter regression tests passed; package verification passed.
- HyperACE GPU/full-song performance and real-weight MelBand Unwa v1e Plus inference are not verified.
- The broader Rust suite previously exceeded a 10-minute limit; no full-suite pass is claimed. Existing Python documentation/evidence checks also have unresolved failures.
