//! フレーム構成しだいで作る段階の資源(空パイプライン・大気LUT・布専用シャドウ経路)の生成局面。
//! 受け取るのは器の生成要求、返すのは3つの`Option`を束ねた組である。段階を持たない構成では`None`が
//! 「その段階を積まない」ことを型で表す。途中で失敗したら、それまでに作った資源をその場で逆順に破棄する。

use super::生成要求;
use crate::atmosphere::大気LUT解像度;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム段階;
use crate::vulkan;

pub(super) struct 任意段階の資源 {
    pub(super) 空: Option<vulkan::pipeline::空パイプライン>,
    pub(super) 大気lut: Option<vulkan::atmosphere_lut::大気LUT一式>,
    pub(super) 布シャドウ: Option<vulkan::cloth_shadow::布シャドウ資源>,
}

pub(super) fn 組み立てる(要求: &生成要求<'_>) -> Result<任意段階の資源, レンダラーエラー> {
    let 空 = 空を生成する(要求)?;
    let 大気lut = match 大気lutを生成する(要求) {
        Ok(値) => 値,
        Err(誤り) => {
            片付ける(要求, 空.as_ref(), None);
            return Err(誤り);
        }
    };
    match 布シャドウを生成する(要求) {
        Ok(布シャドウ) => Ok(任意段階の資源 {
            空, 大気lut, 布シャドウ
        }),
        Err(誤り) => {
            片付ける(要求, 空.as_ref(), 大気lut.as_ref());
            Err(誤り)
        }
    }
}

/// 空パイプラインはフレーム構成に空段階があるときだけ作る。無い構成では`None`が「空を描かない」ことを型で表す。
fn 空を生成する(要求: &生成要求<'_>) -> Result<Option<vulkan::pipeline::空パイプライン>, レンダラーエラー> {
    if !要求.構成.含む(フレーム段階::空) {
        return Ok(None);
    }
    Ok(Some(vulkan::pipeline::空パイプライン::生成する(
        要求.device,
        要求.シーンカラー形式,
        vulkan::depth::深度形式,
        要求.ディスクリプタlayout,
        &要求.シェーダー.空,
    )?))
}

/// 大気LUTはフレーム構成に空段階があるときだけ作る。空を描かない世界は大気の物理量を1度も焼かない。
fn 大気lutを生成する(要求: &生成要求<'_>) -> Result<Option<vulkan::atmosphere_lut::大気LUT一式>, レンダラーエラー> {
    if !要求.構成.含む(フレーム段階::空) {
        return Ok(None);
    }
    Ok(Some(vulkan::atmosphere_lut::大気LUT一式::生成する(
        要求.device,
        要求.メモリプロパティ,
        大気LUT解像度::既定値(),
        &要求.シェーダー.大気lut,
    )?))
}

/// 布専用シャドウ経路はフレーム構成に布シミュレーション段階があるときだけ作る。無い構成では`None`が「布を描かない」ことを型で表す。
fn 布シャドウを生成する(
    要求: &生成要求<'_>
) -> Result<Option<vulkan::cloth_shadow::布シャドウ資源>, レンダラーエラー> {
    if !要求.構成.含む(フレーム段階::布シミュレーション) {
        return Ok(None);
    }
    Ok(Some(vulkan::cloth_shadow::布シャドウ資源::生成する(
        要求.device,
        要求.ユニフォーム,
        &要求.シェーダー.布.シャドウ,
    )?))
}

fn 片付ける(
    要求: &生成要求<'_>, 空: Option<&vulkan::pipeline::空パイプライン>, 大気lut: Option<&vulkan::atmosphere_lut::大気LUT一式>
) {
    if let Some(大気lut) = 大気lut {
        大気lut.破棄する(要求.device);
    }
    if let Some(空) = 空 {
        空.破棄する(要求.device);
    }
}
