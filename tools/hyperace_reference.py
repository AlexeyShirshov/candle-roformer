#!/usr/bin/env python3
"""Generate real-weight HyperACE v2 parity fixtures from the author's source.

The --source must be v2_inst/bs_roformer.py from the pinned model repository.
No source or weights are downloaded. Only the reviewed source SHA is executed.
The attention shim is exact inference-mode PyTorch SDPA (no Sage/flash kernels).
"""
import argparse
import hashlib
from pathlib import Path
import sys
import types


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--weights", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    source = args.source.read_bytes()
    # Git blob id published for the pinned source file.
    git_hash = hashlib.sha1(f"blob {len(source)}\0".encode() + source).hexdigest()
    if git_hash != "012630e2deba7f0d4bad5de69c1833d54469354b":
        raise ValueError(f"unexpected HyperACE v2 reference source: {git_hash}")
    import torch
    from safetensors.torch import load_file, save_file

    class Attend(torch.nn.Module):
        def __init__(self, **kwargs):
            super().__init__()

        def forward(self, q, k, v):
            return torch.nn.functional.scaled_dot_product_attention(q, k, v)

    attend = types.ModuleType("models.bs_roformer.attend")
    attend.Attend = Attend
    sys.modules["models.bs_roformer.attend"] = attend
    # torchaudio is imported but never used by this reference file.
    sys.modules["torchaudio"] = types.ModuleType("torchaudio")
    namespace = {"__name__": "hyperace_reference"}
    exec(compile(source, str(args.source), "exec"), namespace)
    torch.set_num_threads(4)
    state = load_file(str(args.weights))
    model = namespace["BSRoformer"](
        dim=256, depth=12, stereo=True, num_stems=1,
        time_transformer_depth=1, freq_transformer_depth=1,
        stft_hop_length=512, flash_attn=False,
    ).eval()
    model.load_state_dict(state, strict=True)
    head = model.mask_estimators[0].segm
    fixture = {}
    with torch.no_grad():
        for label, time in [("even", 32), ("odd", 33)]:
            x = torch.sin(torch.arange(time * 62 * 256, dtype=torch.float32) * 0.013)
            x = x.reshape(1, time, 62, 256)
            y = head(x.permute(0, 3, 1, 2)).permute(0, 2, 3, 1)
            fixture[f"input_{label}"] = x.contiguous()
            fixture[f"output_{label}"] = y.reshape(1, time, 4100).contiguous()
        audio = torch.sin(torch.arange(8192, dtype=torch.float32) * 0.04) * 0.1
        audio = torch.stack([audio, audio.roll(71)]).unsqueeze(0)
        fixture["audio"] = audio
        fixture["separated"] = model(audio).contiguous()
    save_file(fixture, str(args.out))
    print(f"Saved pinned-source reference to {args.out}")


if __name__ == "__main__":
    main()
