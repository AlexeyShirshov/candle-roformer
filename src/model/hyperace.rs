//! HyperACE v2 mask-estimator residual branch.
//!
//! Architecture reference: pcunwa/BS-Roformer-HyperACE, revision
//! 5b1f8283125d5e4a3614d0e3635a636e09c84059, v2_inst/bs_roformer.py.
//! Parameter paths are preserved verbatim. InstanceNorm uses input statistics
//! even in inference (affine=true, track_running_stats=false, eps=1e-8).
use std::collections::HashMap;

use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;

type Schema = Vec<(String, Vec<usize>)>;

fn weight(s: &mut Schema, p: &str, shape: &[usize]) {
    s.push((p.into(), shape.to_vec()));
}
fn norm_schema(s: &mut Schema, p: &str, c: usize) {
    weight(s, &format!("{p}.weight"), &[c]);
    weight(s, &format!("{p}.bias"), &[c]);
}
fn conv_schema(s: &mut Schema, p: &str, ci: usize, co: usize, k: usize) {
    weight(s, &format!("{p}.conv.weight"), &[co, ci, k, k]);
    norm_schema(s, &format!("{p}.bn"), co);
}
fn ds_schema(s: &mut Schema, p: &str, ci: usize, co: usize) {
    weight(s, &format!("{p}.dwconv.weight"), &[ci, 1, 3, 3]);
    weight(s, &format!("{p}.pwconv.weight"), &[co, ci, 1, 1]);
    norm_schema(s, &format!("{p}.bn"), co);
}
fn c3_schema(s: &mut Schema, p: &str, ci: usize, co: usize, n: usize) {
    conv_schema(s, &format!("{p}.cv1"), ci, co, 1);
    conv_schema(s, &format!("{p}.cv2"), ci, co, 1);
    conv_schema(s, &format!("{p}.cv3"), co * 2, co, 1);
    for i in 0..n {
        ds_schema(s, &format!("{p}.m.{i}.dsconv1"), co, co);
        ds_schema(s, &format!("{p}.m.{i}.dsconv2"), co, co);
    }
}
fn c32_schema(s: &mut Schema, p: &str, ci: usize, co: usize, n: usize) {
    conv_schema(s, &format!("{p}.cv1"), ci, co / 2, 1);
    c3_schema(s, &format!("{p}.m"), co / 2, co / 2, n);
    conv_schema(s, &format!("{p}.cv2"), co / 2, co, 1);
}
fn tfc_schema(s: &mut Schema, p: &str, c: usize, f: usize) {
    for i in 0..2 {
        let p = format!("{p}.blocks.{i}");
        for part in ["tfc1", "tfc2"] {
            norm_schema(s, &format!("{p}.{part}.0"), c);
            weight(s, &format!("{p}.{part}.2.weight"), &[c, c, 3, 3]);
        }
        norm_schema(s, &format!("{p}.tdf.0"), c);
        norm_schema(s, &format!("{p}.tdf.3"), c);
        weight(s, &format!("{p}.tdf.2.weight"), &[f / 4, f]);
        weight(s, &format!("{p}.tdf.5.weight"), &[f, f / 4]);
        weight(s, &format!("{p}.shortcut.weight"), &[c, c, 1, 1]);
    }
}

/// Schema for `mask_estimators.<stem>.segm`, excluding the unchanged MLP.
pub fn expected_key_shapes(dim: usize, bands: usize, prefix: &str) -> Schema {
    let mut s = Vec::new();
    let p = |name: &str| format!("{prefix}.{name}");
    ds_schema(&mut s, &p("backbone.stem"), dim, 64);
    let mut ci = 64;
    for (stage, co, n) in [(2, 256, 2), (3, 384, 4), (4, 512, 4), (5, 768, 2)] {
        ds_schema(&mut s, &p(&format!("backbone.p{stage}.0")), ci, co);
        c32_schema(&mut s, &p(&format!("backbone.p{stage}.1")), co, co, n);
        ci = co;
    }
    conv_schema(&mut s, &p("hyperace.fuse_conv"), 1920, 512, 1);
    for i in 0..2 {
        let h = p(&format!("hyperace.high_order_branch.{i}"));
        for name in ["cv1", "cv2"] {
            conv_schema(&mut s, &format!("{h}.{name}"), 256, 256, 1);
        }
        conv_schema(&mut s, &format!("{h}.cv3"), 512, 256, 1);
        let a = format!("{h}.ahc");
        weight(
            &mut s,
            &format!("{a}.adaptive_hyperedge_gen.global_proto"),
            &[32, 256],
        );
        weight(
            &mut s,
            &format!("{a}.adaptive_hyperedge_gen.context_mapper.weight"),
            &[8192, 512],
        );
        weight(
            &mut s,
            &format!("{a}.adaptive_hyperedge_gen.query_proj.weight"),
            &[256, 256],
        );
        for name in ["W_e", "W_v"] {
            weight(
                &mut s,
                &format!("{a}.hypergraph_conv.{name}.weight"),
                &[256, 256],
            );
        }
    }
    conv_schema(&mut s, &p("hyperace.high_order_fuse"), 512, 256, 1);
    c3_schema(&mut s, &p("hyperace.low_order_branch.0"), 128, 128, 1);
    conv_schema(&mut s, &p("hyperace.final_fuse"), 512, 512, 1);
    for (stage, c) in [(2, 256), (3, 384), (4, 512), (5, 768)] {
        conv_schema(&mut s, &p(&format!("decoder.h_to_d{stage}")), 512, c, 1);
        conv_schema(&mut s, &p(&format!("decoder.skip_p{stage}")), c, c, 1);
        weight(
            &mut s,
            &p(&format!("decoder.fusion_d{stage}.gamma")),
            &[1, c, 1, 1],
        );
    }
    for (stage, ci, co) in [(5, 768, 512), (4, 512, 384), (3, 384, 256)] {
        c32_schema(&mut s, &p(&format!("decoder.up_d{stage}")), ci, co, 1);
    }
    c32_schema(&mut s, &p("decoder.final_d2"), 256, 256, 1);
    for i in 1..=4 {
        let ci = 256 >> (i - 1);
        let co = ci / 2;
        let p = p(&format!("upsample_head.block{i}"));
        ds_schema(&mut s, &format!("{p}.conv"), ci, co * 2);
        tfc_schema(&mut s, &format!("{p}.out_conv"), co, bands << i);
    }
    weight(
        &mut s,
        &p("upsample_head.final_conv.weight"),
        &[4, 16, 3, 3],
    );
    s
}

#[derive(Debug, Clone)]
pub struct HyperAce {
    weights: HashMap<String, Tensor>,
    bins: usize,
}

impl HyperAce {
    pub fn new(dim: usize, bands: usize, bins: usize, vb: VarBuilder) -> Result<Self> {
        let mut weights = HashMap::new();
        for (name, shape) in expected_key_shapes(dim, bands, "segm") {
            let local = name.strip_prefix("segm.").unwrap();
            weights.insert(local.to_string(), vb.get(shape.as_slice(), local)?);
        }
        Ok(Self { weights, bins })
    }

    fn w(&self, name: &str) -> &Tensor {
        &self.weights[name]
    }

    fn linear(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        let shape = x.dims();
        let last = *shape.last().unwrap();
        let w = self.w(&format!("{p}.weight"));
        let y = x
            .contiguous()?
            .reshape((x.elem_count() / last, last))?
            .matmul(&w.t()?.contiguous()?)?;
        let mut out_shape = shape.to_vec();
        *out_shape.last_mut().unwrap() = w.dim(0)?;
        y.reshape(out_shape)
    }

    fn norm(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        let (_, c, h, w) = x.dims4()?;
        if h * w <= 1 {
            candle_core::bail!("HyperACE InstanceNorm requires more than one spatial element");
        }
        let mean = x.mean_keepdim((2, 3))?;
        let centered = x.broadcast_sub(&mean)?;
        let variance = centered.sqr()?.mean_keepdim((2, 3))?;
        centered
            .broadcast_div(&(variance + 1e-8)?.sqrt()?)?
            .broadcast_mul(&self.w(&format!("{p}.weight")).reshape((1, c, 1, 1))?)?
            .broadcast_add(&self.w(&format!("{p}.bias")).reshape((1, c, 1, 1))?)
    }

    fn raw_conv(&self, x: &Tensor, p: &str, stride: usize, groups: usize) -> Result<Tensor> {
        let w = self.w(&format!("{p}.weight"));
        x.contiguous()?.conv2d(w, w.dim(2)? / 2, stride, 1, groups)
    }

    fn conv(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        candle_nn::ops::silu(&self.norm(
            &self.raw_conv(x, &format!("{p}.conv"), 1, 1)?,
            &format!("{p}.bn"),
        )?)
    }

    fn ds(&self, x: &Tensor, p: &str, stride: (usize, usize)) -> Result<Tensor> {
        let mut y = self.raw_conv(x, &format!("{p}.dwconv"), stride.1, x.dim(1)?)?;
        if stride.0 != stride.1 {
            // PyTorch's (2,1) stride, absent from Candle's scalar-stride API.
            let indices: Vec<u32> = (0..y.dim(2)?).step_by(stride.0).map(|i| i as u32).collect();
            y = y.index_select(
                &Tensor::from_vec(indices.clone(), indices.len(), y.device())?,
                2,
            )?;
        }
        y = self.raw_conv(&y, &format!("{p}.pwconv"), 1, 1)?;
        candle_nn::ops::silu(&self.norm(&y, &format!("{p}.bn"))?)
    }

    fn c3(&self, x: &Tensor, p: &str, n: usize) -> Result<Tensor> {
        let mut a = self.conv(x, &format!("{p}.cv1"))?;
        let b = self.conv(x, &format!("{p}.cv2"))?;
        for i in 0..n {
            let y = self.ds(&a, &format!("{p}.m.{i}.dsconv1"), (1, 1))?;
            a = (&a + self.ds(&y, &format!("{p}.m.{i}.dsconv2"), (1, 1))?)?;
        }
        self.conv(&Tensor::cat(&[a, b], 1)?, &format!("{p}.cv3"))
    }

    fn c32(&self, x: &Tensor, p: &str, n: usize) -> Result<Tensor> {
        let x = self.conv(x, &format!("{p}.cv1"))?;
        let x = self.c3(&x, &format!("{p}.m"), n)?;
        self.conv(&x, &format!("{p}.cv2"))
    }

    fn resize(x: &Tensor, h: usize, w: usize) -> Result<Tensor> {
        if x.dim(2)? == h && x.dim(3)? == w {
            Ok(x.clone())
        } else {
            x.contiguous()?.upsample_bilinear2d(h, w, false)
        }
    }

    fn frequency_shuffle(x: &Tensor) -> Result<Tensor> {
        let (b, c, h, w) = x.dims4()?;
        x.reshape((b, c / 2, 2, h, w))?
            .permute((0, 1, 3, 4, 2))?
            .contiguous()?
            .reshape((b, c / 2, h, w * 2))
    }

    fn ahc(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        let (b, c, h, w) = x.dims4()?;
        let n = h * w;
        let flat = x.flatten_from(2)?.transpose(1, 2)?.contiguous()?;
        let avg = flat.mean(1)?;
        let max = flat.max(1)?;
        let ctx = Tensor::cat(&[avg, max], 1)?;
        let gen = format!("{p}.adaptive_hyperedge_gen");
        let delta = self
            .linear(&ctx, &format!("{gen}.context_mapper"))?
            .reshape((b, 32, c))?;
        let proto = delta
            .broadcast_add(&self.w(&format!("{gen}.global_proto")).unsqueeze(0)?)?
            .reshape((b, 32, 8, c / 8))?
            .permute((0, 2, 3, 1))?
            .contiguous()?;
        let z = self
            .linear(&flat, &format!("{gen}.query_proj"))?
            .reshape((b, n, 8, c / 8))?
            .permute((0, 2, 1, 3))?
            .contiguous()?;
        let sim = (z.matmul(&proto)? * ((c / 8) as f64).powf(-0.5))?.mean(1)?;
        // Normalize over vertices, NOT over hyperedges.
        let a = candle_nn::ops::softmax_last_dim(&sim.transpose(1, 2)?.contiguous()?)?;
        let f = candle_nn::ops::silu(
            &self.linear(&a.matmul(&flat)?, &format!("{p}.hypergraph_conv.W_e"))?,
        )?;
        let out = candle_nn::ops::silu(&self.linear(
            &a.transpose(1, 2)?.contiguous()?.matmul(&f)?,
            &format!("{p}.hypergraph_conv.W_v"),
        )?)?;
        (flat + out)?.transpose(1, 2)?.reshape((b, c, h, w))
    }

    fn c3ah(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        let lateral = self.conv(x, &format!("{p}.cv1"))?;
        let y = self.ahc(&self.conv(x, &format!("{p}.cv2"))?, &format!("{p}.ahc"))?;
        self.conv(&Tensor::cat(&[y, lateral], 1)?, &format!("{p}.cv3"))
    }

    fn tfc(&self, x: &Tensor, p: &str) -> Result<Tensor> {
        let mut x = x.clone();
        for i in 0..2 {
            let p = format!("{p}.blocks.{i}");
            let shortcut = self.raw_conv(&x, &format!("{p}.shortcut"), 1, 1)?;
            x = self.raw_conv(
                &candle_nn::ops::silu(&self.norm(&x, &format!("{p}.tfc1.0"))?)?,
                &format!("{p}.tfc1.2"),
                1,
                1,
            )?;
            let mut y = candle_nn::ops::silu(&self.norm(&x, &format!("{p}.tdf.0"))?)?;
            y = self.linear(&y, &format!("{p}.tdf.2"))?;
            y = candle_nn::ops::silu(&self.norm(&y, &format!("{p}.tdf.3"))?)?;
            y = self.linear(&y, &format!("{p}.tdf.5"))?;
            x = (x + y)?;
            x = self.raw_conv(
                &candle_nn::ops::silu(&self.norm(&x, &format!("{p}.tfc2.0"))?)?,
                &format!("{p}.tfc2.2"),
                1,
                1,
            )?;
            x = (x + shortcut)?;
        }
        Ok(x)
    }

    /// Input (batch,time,bands,dim), output (batch,time,bins*4), ordered
    /// frequency -> stereo channel -> complex component, matching the MLP.
    pub fn forward(&self, input: &Tensor) -> Result<Tensor> {
        let (b, time, _, _) = input.dims4()?;
        let mut x = input.permute((0, 3, 1, 2))?.contiguous()?;
        x = self.ds(&x, "backbone.stem", (2, 1))?;
        let mut enc = Vec::new();
        for (stage, n) in [(2, 2), (3, 4), (4, 4), (5, 2)] {
            let stride = if stage < 4 { (2, 1) } else { (2, 2) };
            x = self.ds(&x, &format!("backbone.p{stage}.0"), stride)?;
            x = self.c32(&x, &format!("backbone.p{stage}.1"), n)?;
            enc.push(x.clone());
        }
        let h = enc[2].dim(2)?;
        let w = enc[2].dim(3)?;
        let resized = enc
            .iter()
            .map(|x| Self::resize(x, h, w))
            .collect::<Result<Vec<_>>>()?;
        let fused = self.conv(&Tensor::cat(&resized, 1)?, "hyperace.fuse_conv")?;
        let high = fused.narrow(1, 0, 256)?.contiguous()?;
        let low = fused.narrow(1, 256, 128)?.contiguous()?;
        let skip = fused.narrow(1, 384, 128)?.contiguous()?;
        let highs = (0..2)
            .map(|i| self.c3ah(&high, &format!("hyperace.high_order_branch.{i}")))
            .collect::<Result<Vec<_>>>()?;
        let high = self.conv(&Tensor::cat(&highs, 1)?, "hyperace.high_order_fuse")?;
        let low = self.c3(&low, "hyperace.low_order_branch.0", 1)?;
        let hyper = self.conv(&Tensor::cat(&[high, low, skip], 1)?, "hyperace.final_fuse")?;
        let mut decoded: Option<Tensor> = None;
        for i in (0..4).rev() {
            let stage = i + 2;
            let mut d = self.conv(&enc[i], &format!("decoder.skip_p{stage}"))?;
            if let Some(prev) = decoded {
                let up = Self::resize(&prev, d.dim(2)?, d.dim(3)?)?;
                d = (d + self.c32(&up, &format!("decoder.up_d{}", stage + 1), 1)?)?;
            }
            let h = Self::resize(&hyper, d.dim(2)?, d.dim(3)?)?;
            let h = self.conv(&h, &format!("decoder.h_to_d{stage}"))?;
            d = (d + h.broadcast_mul(self.w(&format!("decoder.fusion_d{stage}.gamma")))?)?;
            decoded = Some(d);
        }
        x = self.c32(&decoded.unwrap(), "decoder.final_d2", 1)?;
        x = Self::resize(&x, time, x.dim(3)?)?;
        for i in 1..=4 {
            let p = format!("upsample_head.block{i}");
            x = self.ds(&x, &format!("{p}.conv"), (1, 1))?;
            // This is frequency-only shuffle, NOT standard 2D pixel_shuffle.
            x = Self::frequency_shuffle(&x)?;
            x = self.tfc(&x, &format!("{p}.out_conv"))?;
        }
        x = Self::resize(&x, time, self.bins)?;
        x = self.raw_conv(&x, "upsample_head.final_conv", 1, 1)?;
        x.permute((0, 2, 3, 1))?
            .contiguous()?
            .reshape((b, time, self.bins * 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{DType, Device};

    #[test]
    fn frequency_shuffle_interleaves_subchannels_not_time() {
        let x = Tensor::arange(0f32, 24f32, &Device::Cpu)
            .unwrap()
            .reshape((1, 4, 2, 3))
            .unwrap();
        let y = HyperAce::frequency_shuffle(&x).unwrap();
        assert_eq!(y.dims(), &[1, 2, 2, 6]);
        assert_eq!(
            y.flatten_all().unwrap().to_vec1::<f32>().unwrap(),
            vec![
                0., 6., 1., 7., 2., 8., 3., 9., 4., 10., 5., 11., 12., 18., 13., 19., 14., 20.,
                15., 21., 16., 22., 17., 23.
            ]
        );
    }

    #[test]
    fn instance_norm_uses_each_samples_biased_spatial_statistics() {
        let mut weights = HashMap::new();
        weights.insert(
            "n.weight".into(),
            Tensor::ones(1, DType::F32, &Device::Cpu).unwrap(),
        );
        weights.insert(
            "n.bias".into(),
            Tensor::zeros(1, DType::F32, &Device::Cpu).unwrap(),
        );
        let model = HyperAce { weights, bins: 1 };
        let x = Tensor::from_vec(vec![1f32, 3., 10., 14.], (2, 1, 1, 2), &Device::Cpu).unwrap();
        let y = model
            .norm(&x, "n")
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();
        for (actual, expected) in y.iter().zip([-1f32, 1., -1., 1.]) {
            assert!((actual - expected).abs() < 1e-6);
        }
        let constant = Tensor::ones((1, 1, 2, 2), DType::F32, &Device::Cpu).unwrap();
        assert!(model
            .norm(&constant, "n")
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap()
            .iter()
            .all(|v| *v == 0.));
    }
}
