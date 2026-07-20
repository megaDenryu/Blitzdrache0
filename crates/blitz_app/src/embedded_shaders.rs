//! ビルド時に埋め込まれた初期シェーダー(SPIR-V)。シーンシェーダーはホットリロードで
//! 置き換わるまでの起動直後の1回だけに使う。ファイル名は`build_support`配下の
//! 各コンパイルモジュールの出力名と一致させる。
//! トーンマップシェーダーのホットリロードは粒子と同様に非対応(ビルド時コンパイルのみ)。

use blitz_render::{シェーダー一式, シェーダー束, 粒子シェーダー一式};

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

const トーンマップ頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_vertex.spv"));
const トーンマップフラグメントSPIRV: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_fragment.spv"));

/// レンダラー生成に渡す全シェーダーを埋め込みSPIR-Vから組み立てる(判断38)。
/// 粒子シェーダーは`--particles`指定時のみ含める(判断29)。
pub(crate) fn 埋め込みシェーダー束を生成する(粒子有効: bool) -> Result<シェーダー束, 起動エラー> {
    let 粒子 = if 粒子有効 {
        Some(粒子シェーダー一式::生成する(
            粒子コンピュートSPIRV.to_vec(),
            粒子頂点SPIRV.to_vec(),
            粒子フラグメントSPIRV.to_vec(),
        )?)
    } else {
        None
    };
    Ok(シェーダー束 {
        シーン: シェーダー一式::生成する(頂点SPIRV.to_vec(), フラグメントSPIRV.to_vec())?,
        シャドウ: シェーダー一式::生成する(シャドウ頂点SPIRV.to_vec(), シャドウフラグメントSPIRV.to_vec())?,
        トーンマップ: シェーダー一式::生成する(トーンマップ頂点SPIRV.to_vec(), トーンマップフラグメントSPIRV.to_vec())?,
        ui: シェーダー一式::生成する(UI頂点SPIRV.to_vec(), UIフラグメントSPIRV.to_vec())?,
        粒子,
    })
}
