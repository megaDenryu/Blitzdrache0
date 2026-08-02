//! 束のディスクリプタプールを確保し、割り当てたジオメトリのセットへその対象が読む2本のストレージバッファを書き込む局面。
//! 呼ばれるのは束の読込時の1度だけであり、毎フレームの参照とは呼び出し頻度が異なる。
//! 走査順は割り当ての並び(描画対象の順、その中でフレームスロットの順)と同じである。

use ash::vk;

use super::{ジオメトリセット参照, 位置を求める, 描画対象ディスクリプタプール};
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{alloc, geometry_set, シーンセットレイアウト一式};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

impl 描画対象ディスクリプタプール {
    /// `ジオメトリ参照一覧`の並びが束の中での描画対象添字である。
    pub(crate) fn 生成する(
        device: &ash::Device,
        レイアウト: &シーンセットレイアウト一式,
        ジオメトリ参照一覧: &[ジオメトリセット参照],
    ) -> Result<Self, レンダラーエラー> {
        let 描画対象数 = ジオメトリ参照一覧.len();
        let pool = プールを生成する(device, セット数を数える(描画対象数))?;
        match 割り当てて書き込む(device, pool, レイアウト, ジオメトリ参照一覧) {
            Ok(ジオメトリset一覧) => Ok(Self::束ねる(pool, ジオメトリset一覧, 描画対象数)),
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                Err(誤り)
            }
        }
    }
}

fn セット数を数える(描画対象数: usize) -> u32 {
    let 件数 = 描画対象数
        .checked_mul(進行中フレーム数)
        .unwrap_or_else(|| panic!("ジオメトリのセット数がusizeを超えた"));
    u32::try_from(件数).unwrap_or_else(|_| panic!("ジオメトリのセット数がu32に収まらない: {件数}"))
}

/// 束の描画対象がフレームスロットごとに1つずつ使うジオメトリのセットを確保する。1セットが2本のストレージバッファを結ぶ。
fn プールを生成する(device: &ash::Device, セット数: u32) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(2 * セット数)];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 割り当てて書き込む(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    レイアウト: &シーンセットレイアウト一式,
    ジオメトリ参照一覧: &[ジオメトリセット参照],
) -> Result<Vec<vk::DescriptorSet>, レンダラーエラー> {
    let セット数 = ジオメトリ参照一覧.len() * 進行中フレーム数;
    let set一覧 = alloc::割り当てる(device, pool, レイアウト.ジオメトリ(), セット数)?;
    for (描画対象添字, 参照) in ジオメトリ参照一覧.iter().enumerate() {
        for フレーム添字 in フレームスロット添字::全スロット() {
            let Some(set) = set一覧.get(位置を求める(描画対象添字, フレーム添字)).copied() else {
                panic!("ジオメトリのセット一覧が走査中の位置を持たない");
            };
            let 可視id列 = (参照.可視id列.buffer(フレーム添字), 参照.可視id列.範囲());
            geometry_set::資源を結ぶ(device, set, 参照.個体レコード, 可視id列);
        }
    }
    Ok(set一覧)
}
