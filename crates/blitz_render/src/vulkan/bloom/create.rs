//! ブルーム一式の生成手順: サンプラー → ディスクリプタ(layout+pool+3セット) →
//! 抽出パイプライン(プッシュ定数なし) → ぼかしパイプライン(方向float2)。
//! 途中失敗時は生成済みの資源を逆順で片付ける。

use super::{descriptor, ぼかしプッシュ定数バイト数, ブルーム一式};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::hdr_target::HDR形式;
use crate::vulkan::{fullscreen_pipeline, linear_sampler};

pub(super) fn 生成する(
    device: &ash::Device,
    抽出シェーダー: &シェーダー一式,
    ぼかしシェーダー: &シェーダー一式,
) -> Result<ブルーム一式, レンダラーエラー> {
    let sampler = linear_sampler::作る(device)?;
    let ディスクリプタ = match descriptor::生成する(device) {
        Ok(ディスクリプタ) => ディスクリプタ,
        Err(誤り) => {
            // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_sampler(sampler, None) };
            return Err(誤り);
        }
    };
    let 片付ける = |device: &ash::Device| {
        ディスクリプタ.破棄する(device);
        // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
        unsafe { device.destroy_sampler(sampler, None) };
    };

    // 抽出・ぼかしの描画先はどちらも1/2解像度のHDR形式画像(判断39)。
    let 抽出組 = match fullscreen_pipeline::組み立てる(device, HDR形式, ディスクリプタ.layout, 抽出シェーダー, c"extractMain", 0) {
        Ok(組) => 組,
        Err(誤り) => {
            片付ける(device);
            return Err(誤り);
        }
    };
    let ぼかし組 = match fullscreen_pipeline::組み立てる(
        device,
        HDR形式,
        ディスクリプタ.layout,
        ぼかしシェーダー,
        c"blurMain",
        ぼかしプッシュ定数バイト数,
    ) {
        Ok(組) => 組,
        Err(誤り) => {
            // 安全性: 抽出パイプラインとlayoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_pipeline(抽出組.0, None);
                device.destroy_pipeline_layout(抽出組.1, None);
            }
            片付ける(device);
            return Err(誤り);
        }
    };

    Ok(ブルーム一式 {
        抽出pipeline: 抽出組.0,
        抽出layout: 抽出組.1,
        ぼかしpipeline: ぼかし組.0,
        ぼかしlayout: ぼかし組.1,
        sampler,
        descriptor_layout: ディスクリプタ.layout,
        descriptor_pool: ディスクリプタ.pool,
        抽出set: ディスクリプタ.set一覧[0],
        横set: ディスクリプタ.set一覧[1],
        縦set: ディスクリプタ.set一覧[2],
    })
}
