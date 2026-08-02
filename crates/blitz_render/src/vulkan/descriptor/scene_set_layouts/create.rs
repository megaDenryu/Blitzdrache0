//! 5つのセットレイアウトと固定サンプラーを確保する局面。呼ばれるのはレンダラー生成時の1度だけである。
//! 途中で失敗したら、そこまでに確保したレイアウトとサンプラーをその場で破棄するため、
//! 部分的に生成された一式は呼び出し元から見えない。

use ash::vk;

use super::シーンセットレイアウト一式;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{empty_set, geometry_set, lighting_set, material_set, view_pass_set};
use crate::vulkan::material_table::テクスチャ表レイアウト容量;
use crate::vulkan::texture::table_sampler;

/// 生成する5つのレイアウトの数。4つの役割に、役割を読まない位置を埋める空のレイアウトを足したものである。
const レイアウト数: usize = 5;

pub(super) fn 生成する(
    device: &ash::Device,
    表容量: テクスチャ表レイアウト容量,
) -> Result<シーンセットレイアウト一式, レンダラーエラー> {
    let 材質サンプラー = table_sampler::生成する(device)?;
    let 一覧 = match 順に生成する(device, 表容量, 材質サンプラー) {
        Ok(値) => 値,
        Err(誤り) => {
            table_sampler::破棄する(device, 材質サンプラー);
            return Err(誤り);
        }
    };
    Ok(シーンセットレイアウト一式 {
        材質サンプラー,
        材質テクスチャ表容量: 表容量,
        ビューとパス: 一覧[0],
        ジオメトリ: 一覧[1],
        材質: 一覧[2],
        照明問い合わせ: 一覧[3],
        空: 一覧[4],
    })
}

fn 順に生成する(
    device: &ash::Device,
    表容量: テクスチャ表レイアウト容量,
    材質サンプラー: vk::Sampler,
) -> Result<[vk::DescriptorSetLayout; レイアウト数], レンダラーエラー> {
    let mut 一覧 = [vk::DescriptorSetLayout::null(); レイアウト数];
    for 添字 in 0..レイアウト数 {
        match 番号で生成する(device, 添字, 表容量, 材質サンプラー) {
            Ok(値) => 一覧[添字] = 値,
            Err(誤り) => {
                for 生成済み in 一覧.iter().take(添字) {
                    // 安全性: 生成途中のレイアウトはこのスコープの唯一の所有者で、以降使用しない。
                    unsafe { device.destroy_descriptor_set_layout(*生成済み, None) };
                }
                return Err(誤り);
            }
        }
    }
    Ok(一覧)
}

fn 番号で生成する(
    device: &ash::Device,
    添字: usize,
    表容量: テクスチャ表レイアウト容量,
    材質サンプラー: vk::Sampler,
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    match 添字 {
        0 => view_pass_set::レイアウトを生成する(device),
        1 => geometry_set::レイアウトを生成する(device),
        2 => material_set::レイアウトを生成する(device, 表容量, 材質サンプラー),
        3 => lighting_set::レイアウトを生成する(device),
        _ => empty_set::レイアウトを生成する(device),
    }
}
