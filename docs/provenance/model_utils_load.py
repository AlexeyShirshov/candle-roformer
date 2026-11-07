                    param.requires_grad = True
        else:
            from peft import LoraConfig, get_peft_model
            lora_config = LoraConfig(
                **config['lora']
            )
            model = get_peft_model(model, lora_config)
    return model


def load_start_checkpoint(args: argparse.Namespace,
                          model: torch.nn.Module,
                          old_model,
                          type_: str = 'train') -> None:
    """
    Load an initial checkpoint into `model`.

    For `type_ == "train"`, performs a tolerant load using `old_model` (a state dict or a
    checkpoint dict) via `load_not_compatible_weights`, allowing partial shape mismatches.
    For other modes, loads a strict state dict from `args.start_check_point`, with special
    handling for HTDemucs/Apollo checkpoints (keys under "state"/"state_dict"). If
    `args.lora_checkpoint` is set, LoRA weights are applied after the base load.

    Args:
        args: Namespace with at least `start_check_point`, `model_type`, and optionally `lora_checkpoint`.
        model: Target PyTorch module to receive weights.
        old_model: Source weights for tolerant loading in train mode (state dict or checkpoint dict).
        type_: Loading strategy; "train" uses tolerant loading, otherwise strict loading from path.

    Returns:
        None
    """
    should_print = not dist.is_initialized() or dist.get_rank() == 0

    if should_print:
        print(f'Start from checkpoint: {args.start_check_point}')
    if type_ in ['train']:
        if not args.load_only_compatible_weights:
            load_not_compatible_weights(model, old_model, verbose=False)
        else:
            model.load_state_dict(torch.load(args.start_check_point))
    else:
        device = 'cpu'
        if args.model_type in ['htdemucs', 'apollo']:
            old_model = torch.load(args.start_check_point, map_location=device, weights_only=False)
            # Fix for htdemucs pretrained models
            if 'state' in old_model:
                old_model = old_model['state']
            # Fix for apollo pretrained models
            if 'state_dict' in old_model:
                old_model = old_model['state_dict']
        else:
            if 'state' in old_model:
                # Fix for htdemucs weights loading
                old_model = old_model['state']
            if 'state_dict' in old_model:
                # Fix for apollo weights loading
                old_model = old_model['state_dict']
            if 'model_state_dict' in old_model:
                # Fix for full_check_point
                old_model = old_model['model_state_dict']
        model.load_state_dict(old_model)

    if args.lora_checkpoint_loralib:
        if should_print:
            print(f"Loading LoRA weights from: {args.lora_checkpoint_loralib}")
        load_lora_weights(model, args.lora_checkpoint_loralib)


def bind_lora_to_model(config: Dict[str, Any], model: nn.Module) -> nn.Module:
    """
