//! 光のにじみ一式のパイプライン部(サンプラー・レイアウト2種・パイプライン3本)の生成。
//! ディスクリプタのプールとセットは空(null)のまま返し、`ディスクリプタを作り直す`が埋める。

use ash::vk;

use super::{descriptor, 光のにじみ一式};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::hdr_target::HDR形式;
use crate::vulkan::{fullscreen_pipeline, linear_sampler};

pub(super) fn パイプライン部を生成する(
    device: &ash::Device,
    前処理シェーダー: &シェーダー一式,
    縮小シェーダー: &シェーダー一式,
    拡大シェーダー: &シェーダー一式,
) -> Result<光のにじみ一式, レンダラーエラー> {
    let sampler = linear_sampler::作る(device)?;
    let (単一読みlayout, 二読みlayout) = match descriptor::レイアウト2種を作る(device) {
        Ok(組) => 組,
        Err(誤り) => {
            // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_sampler(sampler, None) };
            return Err(誤り);
        }
    };

    // 描画先はすべてHDR形式のピラミッド画像(判断41)。プッシュ定数は使わない。
    let 仕様一覧: [(&シェーダー一式, &std::ffi::CStr, vk::DescriptorSetLayout); 3] = [
        (前処理シェーダー, c"prefilterMain", 単一読みlayout),
        (縮小シェーダー, c"downsampleMain", 単一読みlayout),
        (拡大シェーダー, c"upsampleMain", 二読みlayout),
    ];
    let mut 組一覧: Vec<(vk::Pipeline, vk::PipelineLayout)> = Vec::new();
    for (シェーダー, エントリ名, layout) in 仕様一覧 {
        match fullscreen_pipeline::組み立てる(device, HDR形式, layout, シェーダー, エントリ名, 0) {
            Ok(組) => 組一覧.push(組),
            Err(誤り) => {
                // 安全性: 生成済みのパイプラインとlayoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    for &(pipeline, pipeline_layout) in &組一覧 {
                        device.destroy_pipeline(pipeline, None);
                        device.destroy_pipeline_layout(pipeline_layout, None);
                    }
                    device.destroy_descriptor_set_layout(単一読みlayout, None);
                    device.destroy_descriptor_set_layout(二読みlayout, None);
                    device.destroy_sampler(sampler, None);
                }
                return Err(誤り);
            }
        }
    }

    Ok(光のにじみ一式 {
        前処理pipeline: 組一覧[0].0,
        前処理layout: 組一覧[0].1,
        縮小pipeline: 組一覧[1].0,
        縮小layout: 組一覧[1].1,
        拡大pipeline: 組一覧[2].0,
        拡大layout: 組一覧[2].1,
        sampler,
        単一読みlayout,
        二読みlayout,
        descriptor_pool: vk::DescriptorPool::null(),
        前処理set: vk::DescriptorSet::null(),
        縮小set一覧: Vec::new(),
        拡大set一覧: Vec::new(),
    })
}
