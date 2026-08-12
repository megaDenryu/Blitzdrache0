//! 共有ディスクリプタセットを確保して内容を結ぶ局面。呼ばれるのはレンダラー生成時の1度だけであり、
//! 以降のフレームは参照しかしない。途中で失敗したらプールをその場で破棄する。

use ash::vk;

use super::共有ディスクリプタセット;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::view_pass_set::ビューとパスのセットの書き込み先;
use crate::vulkan::descriptor::{alloc, シーンセットレイアウト一式};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(super) fn 生成する(
    device: &ash::Device,
    レイアウト: &シーンセットレイアウト一式,
    シェーダー定数: &フレームシェーダー定数一式,
) -> Result<共有ディスクリプタセット, レンダラーエラー> {
    let pool = プールを生成する(device)?;
    match 割り当てて結ぶ(device, pool, レイアウト, シェーダー定数) {
        Ok(ビューとパス一覧) => Ok(共有ディスクリプタセット {
            pool, ビューとパス一覧
        }),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}

fn 割り当てて結ぶ(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    レイアウト: &シーンセットレイアウト一式,
    シェーダー定数: &フレームシェーダー定数一式,
) -> Result<[vk::DescriptorSet; 進行中フレーム数], レンダラーエラー> {
    let ビューとパス一覧 = alloc::割り当てる(device, pool, レイアウト.ビューとパス(), 進行中フレーム数)?;
    let ビューとパス一覧 = match <[vk::DescriptorSet; 進行中フレーム数]>::try_from(ビューとパス一覧) {
        Ok(値) => 値,
        Err(_) => panic!("ビューとパスのセット数が進行中フレーム数と一致しない"),
    };
    for フレーム添字 in フレームスロット添字::全スロット() {
        ビューとパスのセットの書き込み先::生成する(device, ビューとパス一覧[フレーム添字.配列添字()])
            .フレームの定数3本を結ぶ(シェーダー定数, フレーム添字);
    }
    Ok(ビューとパス一覧)
}

/// 定数3本を進行中フレーム数ぶんだけ確保する。
fn プールを生成する(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let フレーム数 = u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(3 * フレーム数)];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(フレーム数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
