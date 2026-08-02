//! 照明問い合わせ資源束を確保して内容を結ぶ局面。呼ばれるのはレンダラー生成時の1度だけであり、
//! 以降のフレームは書き込みと参照しかしない。途中で失敗したら、そこまでに確保したバッファを逆順に破棄し、
//! プールをその場で破棄する。部分的に生成された束は呼び出し元から見えない。

use ash::vk;

use super::pack::{局所光列のバイト長, 方向光列のバイト長};
use super::slot_resources::{スロット資源, 書き換えバッファ};
use super::{header_bytes, 照明問い合わせ資源束};
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{lighting_set, シーンセットレイアウト一式};
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    レイアウト: &シーンセットレイアウト一式,
    シャドウマップ: &シャドウマップ,
) -> Result<照明問い合わせ資源束, レンダラーエラー> {
    let pool = lighting_set::プールを生成する(device, 進行中フレーム数)?;
    match スロット一覧を作る(device, メモリプロパティ, pool, レイアウト, シャドウマップ) {
        Ok(スロット一覧) => Ok(照明問い合わせ資源束 { pool, スロット一覧 }),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}

fn スロット一覧を作る(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    pool: vk::DescriptorPool,
    レイアウト: &シーンセットレイアウト一式,
    シャドウマップ: &シャドウマップ,
) -> Result<Vec<スロット資源>, レンダラーエラー> {
    let セット一覧 = レイアウト.照明問い合わせのセットを割り当てる(device, pool, 進行中フレーム数)?;
    let mut スロット一覧: Vec<スロット資源> = Vec::with_capacity(進行中フレーム数);
    for 添字 in フレームスロット添字::全スロット() {
        let セット = セット一覧[添字.配列添字()];
        match スロット1つを作る(device, メモリプロパティ, セット) {
            Ok(資源) => {
                lighting_set::資源を結ぶ(device, セット, 資源.バッファ組(), シャドウマップ);
                スロット一覧.push(資源);
            }
            Err(誤り) => {
                for 生成済み in スロット一覧.iter().rev() {
                    生成済み.破棄する(device);
                }
                return Err(誤り);
            }
        }
    }
    Ok(スロット一覧)
}

fn スロット1つを作る(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    セット: vk::DescriptorSet,
) -> Result<スロット資源, レンダラーエラー> {
    let 定数用途 = vk::BufferUsageFlags::UNIFORM_BUFFER;
    let 列用途 = vk::BufferUsageFlags::STORAGE_BUFFER;
    let ヘッダ = 書き換えバッファ::生成する(device, メモリプロパティ, header_bytes::バイト長, 定数用途)?;
    let 方向光列 = match 書き換えバッファ::生成する(device, メモリプロパティ, 方向光列のバイト長, 列用途) {
        Ok(値) => 値,
        Err(誤り) => {
            ヘッダ.破棄する(device);
            return Err(誤り);
        }
    };
    let 局所光列 = match 書き換えバッファ::生成する(device, メモリプロパティ, 局所光列のバイト長, 列用途) {
        Ok(値) => 値,
        Err(誤り) => {
            方向光列.破棄する(device);
            ヘッダ.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(スロット資源 {
        ヘッダ,
        方向光列,
        局所光列,
        セット,
    })
}
