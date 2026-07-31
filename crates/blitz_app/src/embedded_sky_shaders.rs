//! ビルド時に埋め込まれた空パスと空中遠近合成のSPIR-Vと、その組み立て。担当するのは「合成の指定から
//! 空シェーダーを1つ作る」ことである。布シェーダーを`embedded_cloth_shaders`が持つのと同じ分け方である。
//!
//! 頂点は段で変わらないため、空パスと合成が同じSPIR-Vを使う。ファイル名は
//! `build_support/sky_spirv_compile.rs`の出力名と一致させる。

use blitz_render::{シェーダー一式, 空シェーダー};

use crate::cli::空中遠近合成指定;
use crate::error::起動エラー;

const 空頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_vertex.spv"));
const 空画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_atmosphere_fragment.spv"));
const 空中遠近合成画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/aerial_composite_fragment.spv"));

pub(crate) fn 埋め込み空シェーダーを生成する(合成: 空中遠近合成指定) -> Result<空シェーダー, 起動エラー> {
    Ok(空シェーダー {
        放射輝度: 一式を作る(空画素段SPIRV)?,
        空中遠近合成: 合成の一式を作る(合成)?,
    })
}

/// 合成しない指定では`None`を返す。合成のパイプラインもディスクリプタも作られず、ボリュームも焼かれない。
fn 合成の一式を作る(合成: 空中遠近合成指定) -> Result<Option<シェーダー一式>, 起動エラー> {
    match 合成 {
        空中遠近合成指定::合成しない => Ok(None),
        空中遠近合成指定::合成する => Ok(Some(一式を作る(空中遠近合成画素段SPIRV)?)),
    }
}

fn 一式を作る(画素段: &[u8]) -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(空頂点SPIRV.to_vec(), 画素段.to_vec())?)
}
