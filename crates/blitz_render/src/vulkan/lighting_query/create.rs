//! 照明問い合わせ資源束を確保して内容を結ぶ局面。呼ばれるのはレンダラー生成時の1度だけであり、
//! 以降のフレームは書き込みと参照しかしない。途中で失敗したら、そこまでに確保したバッファを逆順に破棄し、
//! プールをその場で破棄する。部分的に生成された束は呼び出し元から見えない。

use ash::vk;

use super::slot_resources::スロット資源;
use super::照明問い合わせ資源束;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{lighting_set, シーンセットレイアウト一式};
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    レイアウト: &シーンセットレイアウト一式,
    シャドウマップ: &シャドウマップ,
) -> Result<照明問い合わせ資源束, レンダラーエラー> {
    let 束縛レイアウト = レイアウト.照明束縛();
    let pool = lighting_set::プールを生成する(device, 進行中フレーム数, 束縛レイアウト)?;
    match 束を組み上げる(device, メモリプロパティ, pool, レイアウト, シャドウマップ) {
        Ok(束) => Ok(束),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}

/// スロットとサンプラーを順に作る。サンプラーの生成に失敗したら、作ったスロットをその場で逆順に破棄する。
fn 束を組み上げる(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    pool: vk::DescriptorPool,
    レイアウト: &シーンセットレイアウト一式,
    シャドウマップ: &シャドウマップ,
) -> Result<照明問い合わせ資源束, レンダラーエラー> {
    let 束縛レイアウト = レイアウト.照明束縛();
    let スロット一覧 = スロット一覧を作る(device, メモリプロパティ, pool, レイアウト, シャドウマップ)?;
    let 遠方環境サンプラー = match 遠方環境サンプラーを作る(device, 束縛レイアウト) {
        Ok(値) => 値,
        Err(誤り) => {
            for スロット in スロット一覧.iter().rev() {
                スロット.破棄する(device);
            }
            return Err(誤り);
        }
    };
    Ok(照明問い合わせ資源束 {
        pool,
        スロット一覧,
        束縛レイアウト,
        遠方環境サンプラー,
    })
}

fn 遠方環境サンプラーを作る(
    device: &GPUデバイス,
    束縛レイアウト: 照明束縛レイアウト,
) -> Result<Option<vk::Sampler>, レンダラーエラー> {
    if !束縛レイアウト.遠方環境の画像を結ぶか() {
        return Ok(None);
    }
    Ok(Some(lighting_set::distant_environment::サンプラーを生成する(device)?))
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
        match スロット資源::生成する(device, メモリプロパティ, セット) {
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
