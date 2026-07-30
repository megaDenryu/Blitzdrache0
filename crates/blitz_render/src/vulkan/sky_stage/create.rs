//! 空段階資源の生成局面。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは束縛先を引くだけである。
//! 大気LUT腕は標本ディスクリプタを先に作る。パイプラインレイアウトがそのディスクリプタセットレイアウトを要るためである。
//! 途中で失敗したら、それまでに作った資源をその場で片付ける。

use super::{空段階の生成要求, 空段階資源};
use crate::error::レンダラーエラー;
use crate::shader_bundle::空シェーダー;
use crate::vulkan::atmosphere_lut::大気LUT標本ディスクリプタ;
use crate::vulkan::pipeline::空パイプライン;

pub(super) fn 生成する(device: &ash::Device, 要求: 空段階の生成要求<'_>) -> Result<空段階資源, レンダラーエラー> {
    match 要求.シェーダー {
        空シェーダー::Hosek解析近似(シェーダー) => {
            let layout一覧 = [要求.シーンlayout];
            let パイプライン = パイプラインを作る(device, &要求, &layout一覧, シェーダー)?;
            Ok(空段階資源::Hosek解析近似 { パイプライン })
        }
        空シェーダー::大気LUT(シェーダー) => {
            let 標本 = 大気LUT標本ディスクリプタ::生成する(device, &要求.大気lut.標本の束縛先())?;
            let layout一覧 = [要求.シーンlayout, 標本.layout];
            match パイプラインを作る(device, &要求, &layout一覧, シェーダー) {
                Ok(パイプライン) => Ok(空段階資源::大気LUT { パイプライン, 標本 }),
                Err(誤り) => {
                    標本.破棄する(device);
                    Err(誤り)
                }
            }
        }
    }
}

fn パイプラインを作る(
    device: &ash::Device,
    要求: &空段階の生成要求<'_>,
    layout一覧: &[ash::vk::DescriptorSetLayout],
    シェーダー: &crate::shader_set::シェーダー一式,
) -> Result<空パイプライン, レンダラーエラー> {
    空パイプライン::生成する(device, 要求.カラー形式, 要求.深度形式, layout一覧, シェーダー)
}
