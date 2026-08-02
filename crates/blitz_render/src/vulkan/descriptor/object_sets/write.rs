//! 束のディスクリプタプールを確保し、割り当てたセットへそのセットが束ねる資源を書き込む工程。束の読込時に1度だけ呼ばれる。
//! 走査順は割り当ての並び(ジオメトリは描画対象の順その中でフレームスロットの順、材質は描画対象の順その中で材質スロットの順)と同じであり、
//! 位置の導出は`placement`が持つ配置に一任する。

use ash::vk;

use super::placement::セット配置;
use super::{ジオメトリセット参照, 材質セット参照};
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{alloc, geometry_set, material_set, シーンセットレイアウト一式};
use crate::vulkan::sync::フレームスロット添字;

type セット二種 = (Vec<vk::DescriptorSet>, Vec<vk::DescriptorSet>);

pub(super) fn 割り当てて書き込む(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    レイアウト: &シーンセットレイアウト一式,
    配置: &セット配置,
    ジオメトリ参照一覧: &[ジオメトリセット参照],
    対象別材質参照一覧: &[Vec<材質セット参照<'_>>],
) -> Result<セット二種, レンダラーエラー> {
    let ジオメトリset一覧 = alloc::割り当てる(device, pool, レイアウト.ジオメトリ(), 配置.ジオメトリセット数())?;
    let 材質set一覧 = alloc::割り当てる(device, pool, レイアウト.材質(), 配置.材質セット数())?;
    for (描画対象添字, 参照) in ジオメトリ参照一覧.iter().enumerate() {
        for フレーム添字 in フレームスロット添字::全スロット() {
            let set = 位置のセット(&ジオメトリset一覧, 配置.ジオメトリ位置(描画対象添字, フレーム添字));
            let 可視id列 = (参照.可視id列.buffer(フレーム添字), 参照.可視id列.範囲());
            geometry_set::資源を結ぶ(device, set, 参照.個体レコード, 可視id列);
        }
    }
    for (描画対象添字, スロット別参照) in 対象別材質参照一覧.iter().enumerate() {
        for (スロット添字, 参照) in スロット別参照.iter().enumerate() {
            let set = 位置のセット(&材質set一覧, 配置.材質位置(描画対象添字, スロット添字));
            material_set::資源を結ぶ(device, set, 参照.材質レコード, 参照.テクスチャ);
        }
    }
    Ok((ジオメトリset一覧, 材質set一覧))
}

/// 束の描画対象が要るジオメトリのセットと材質のセットを、種類ごとの件数だけ確保する。
pub(super) fn プールを生成する(device: &ash::Device, 配置: &セット配置) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let ジオメトリ数 = u32へ変換する(配置.ジオメトリセット数());
    let 材質数 = u32へ変換する(配置.材質セット数());
    let プールサイズ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(2 * ジオメトリ数 + 材質数),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(3 * 材質数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(ジオメトリ数 + 材質数)
        .pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 位置のセット(set一覧: &[vk::DescriptorSet], 位置: Option<usize>) -> vk::DescriptorSet {
    let Some(位置) = 位置 else {
        panic!("書き込む対象の添字がディスクリプタセットの配置の外だった");
    };
    match set一覧.get(位置) {
        Some(set) => *set,
        None => panic!("ディスクリプタセット一覧が配置の示す位置を持たない"),
    }
}

fn u32へ変換する(件数: usize) -> u32 {
    u32::try_from(件数).unwrap_or_else(|_| panic!("ディスクリプタセット数がu32に収まらない: {件数}"))
}
