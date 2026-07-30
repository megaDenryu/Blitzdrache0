//! 空段階資源の生成局面。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは束縛先を引くだけである。
//! 大気LUT腕は標本ディスクリプタを先に作る。パイプラインレイアウトがそのディスクリプタセットレイアウトを要るためである。
//! 合成の一式は最後に作る。合成の失敗で空パスまで失われないよう、途中で失敗したらそれまでに作った資源をその場で片付ける。

use super::{空中遠近合成資源, 空段階の生成要求, 空段階資源};
use crate::error::レンダラーエラー;
use crate::shader_bundle::空シェーダー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::atmosphere_lut::大気LUT標本ディスクリプタ;
use crate::vulkan::pipeline::空パイプライン;

pub(super) fn 生成する(device: &ash::Device, 要求: 空段階の生成要求<'_>) -> Result<空段階資源, レンダラーエラー> {
    match 要求.シェーダー {
        空シェーダー::Hosek解析近似(シェーダー) => {
            let layout一覧 = [要求.シーンlayout];
            let パイプライン = パイプラインを作る(device, &要求, &layout一覧, シェーダー)?;
            Ok(空段階資源::Hosek解析近似 { パイプライン })
        }
        空シェーダー::大気LUT {
            放射輝度, 空中遠近合成
        } => 大気lut腕を作る(device, &要求, 放射輝度, 空中遠近合成.as_ref()),
    }
}

fn 大気lut腕を作る(
    device: &ash::Device,
    要求: &空段階の生成要求<'_>,
    放射輝度: &シェーダー一式,
    合成シェーダー: Option<&シェーダー一式>,
) -> Result<空段階資源, レンダラーエラー> {
    let 標本 = 大気LUT標本ディスクリプタ::生成する(device, &要求.大気lut.標本の束縛先())?;
    let layout一覧 = [要求.シーンlayout, 標本.layout];
    let パイプライン = match パイプラインを作る(device, 要求, &layout一覧, 放射輝度) {
        Ok(パイプライン) => パイプライン,
        Err(誤り) => {
            標本.破棄する(device);
            return Err(誤り);
        }
    };
    match 合成を作る(device, 要求, 合成シェーダー) {
        Ok(合成) => Ok(空段階資源::大気LUT {
            パイプライン, 標本, 合成
        }),
        Err(誤り) => {
            パイプライン.破棄する(device);
            標本.破棄する(device);
            Err(誤り)
        }
    }
}

fn 合成を作る(
    device: &ash::Device,
    要求: &空段階の生成要求<'_>,
    合成シェーダー: Option<&シェーダー一式>,
) -> Result<Option<空中遠近合成資源>, レンダラーエラー> {
    let Some(シェーダー) = 合成シェーダー else {
        return Ok(None);
    };
    Ok(Some(空中遠近合成資源::生成する(
        device,
        要求.カラー形式,
        要求.シーンlayout,
        &要求.大気lut.合成の束縛先(),
        シェーダー,
    )?))
}

fn パイプラインを作る(
    device: &ash::Device,
    要求: &空段階の生成要求<'_>,
    layout一覧: &[ash::vk::DescriptorSetLayout],
    シェーダー: &シェーダー一式,
) -> Result<空パイプライン, レンダラーエラー> {
    空パイプライン::生成する(device, 要求.カラー形式, 要求.深度形式, layout一覧, シェーダー)
}
