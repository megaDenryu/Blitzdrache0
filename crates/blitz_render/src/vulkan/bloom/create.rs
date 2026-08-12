//! 光のにじみ一式のパイプライン部(サンプラー・レイアウト2種・パイプライン3本)の生成。
//! ディスクリプタのプールとセットは空(null)のまま返し、`ディスクリプタを作り直す`が埋める。

use ash::vk;

use super::{descriptor, 光のにじみ一式};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;
use crate::vulkan::hdr_target::HDR形式;

pub(super) fn パイプライン部を生成する(
    確保係: &GPU資源の確保係<'_>,
    前処理シェーダー: &シェーダー一式,
    縮小シェーダー: &シェーダー一式,
    拡大シェーダー: &シェーダー一式,
) -> Result<光のにじみ一式, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let sampler = 確保係.線形サンプラーを作る()?;
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
    let mut 一覧: Vec<全画面パスのパイプライン> = Vec::new();
    for (シェーダー, エントリ名, layout) in 仕様一覧 {
        match 全画面パスのパイプライン::組み立てる(確保係, HDR形式, layout, シェーダー, エントリ名, 0) {
            Ok(パイプライン) => 一覧.push(パイプライン),
            Err(誤り) => {
                for 生成済み in &一覧 {
                    生成済み.破棄する(device);
                }
                // 安全性: 2つのレイアウトとサンプラーはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    device.destroy_descriptor_set_layout(単一読みlayout, None);
                    device.destroy_descriptor_set_layout(二読みlayout, None);
                    device.destroy_sampler(sampler, None);
                }
                return Err(誤り);
            }
        }
    }
    let [前処理, 縮小, 拡大] = 一覧
        .try_into()
        .unwrap_or_else(|_| panic!("全画面パスのパイプラインが仕様の本数ぶん揃わなかった"));

    Ok(光のにじみ一式 {
        前処理,
        縮小,
        拡大,
        sampler,
        単一読みlayout,
        二読みlayout,
        descriptor_pool: vk::DescriptorPool::null(),
        前処理set: vk::DescriptorSet::null(),
        縮小set一覧: Vec::new(),
        拡大set一覧: Vec::new(),
    })
}
