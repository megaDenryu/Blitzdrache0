//! ビルド時に埋め込まれた初期シェーダー(SPIR-V)。ホットリロードで置き換わるまでの
//! 起動直後の1回だけに使う。ファイル名は`build_support::spirv_compile`
//! `build_support::particle_spirv_compile`の出力名と一致させる。

use blitz_render::{シェーダー一式, 粒子シェーダー一式};

use crate::error::起動エラー;

const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv"));
const フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv"));

const 粒子コンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_compute.spv"));
const 粒子頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_vertex.spv"));
const 粒子フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_fragment.spv"));

const UI頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_vertex.spv"));
const UIフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_fragment.spv"));

const シャドウ頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_vertex.spv"));
const シャドウフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_fragment.spv"));

pub(crate) fn 埋め込みシェーダーを生成する() -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(頂点SPIRV.to_vec(), フラグメントSPIRV.to_vec())?)
}

pub(crate) fn 埋め込み粒子シェーダーを生成する() -> Result<粒子シェーダー一式, 起動エラー> {
    Ok(粒子シェーダー一式::生成する(
        粒子コンピュートSPIRV.to_vec(),
        粒子頂点SPIRV.to_vec(),
        粒子フラグメントSPIRV.to_vec(),
    )?)
}

/// 開発用UI(egui)パイプライン用のシェーダー一式(判断33)。UIリソース一式は
/// 常に生成する(判断34)ため、粒子シェーダーと異なり無条件で埋め込む。
pub(crate) fn 埋め込みuiシェーダーを生成する() -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(UI頂点SPIRV.to_vec(), UIフラグメントSPIRV.to_vec())?)
}

/// シャドウパス(判断35)用のシェーダー一式。シャドウパスは常にグラフへ積むため、
/// 開発用UIと同様に無条件で埋め込む。
pub(crate) fn 埋め込みシャドウシェーダーを生成する() -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(シャドウ頂点SPIRV.to_vec(), シャドウフラグメントSPIRV.to_vec())?)
}
