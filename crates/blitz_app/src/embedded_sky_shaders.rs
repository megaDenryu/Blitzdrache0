//! ビルド時に埋め込まれた空パスと空中遠近合成のSPIR-Vと、方式ごとの腕の組み立て。担当するのは「起動指定の方式と
//! 合成の指定から空シェーダーの腕を1つ選ぶ」ことである。布シェーダーを`embedded_cloth_shaders`が持つのと同じ分け方である。
//!
//! 頂点は方式でも段でも変わらないため、空の両腕と合成が同じSPIR-Vを使う。ファイル名は
//! `build_support/sky_spirv_compile.rs`の出力名と一致させる。

use blitz_render::{シェーダー一式, 空シェーダー};

use crate::cli::{空の描画指定, 空の方式指定, 空中遠近合成指定};
use crate::error::起動エラー;

const 空頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_vertex.spv"));
const 空フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_fragment.spv"));
const 空大気LUTフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_atmosphere_fragment.spv"));
const 空中遠近合成フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/aerial_composite_fragment.spv"));

pub(crate) fn 埋め込み空シェーダーを生成する(指定: 空の描画指定) -> Result<空シェーダー, 起動エラー> {
    match 指定.方式 {
        空の方式指定::Hosek解析近似 => Ok(空シェーダー::Hosek解析近似(一式を作る(空フラグメントSPIRV)?)),
        空の方式指定::大気LUT => Ok(空シェーダー::大気LUT {
            放射輝度: 一式を作る(空大気LUTフラグメントSPIRV)?,
            空中遠近合成: 合成の一式を作る(指定.空中遠近合成)?,
        }),
    }
}

/// 合成しない指定では`None`を返す。合成のパイプラインもディスクリプタも作られず、ボリュームも焼かれない。
fn 合成の一式を作る(合成: 空中遠近合成指定) -> Result<Option<シェーダー一式>, 起動エラー> {
    match 合成 {
        空中遠近合成指定::合成しない => Ok(None),
        空中遠近合成指定::合成する => Ok(Some(一式を作る(空中遠近合成フラグメントSPIRV)?)),
    }
}

fn 一式を作る(フラグメント: &[u8]) -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(空頂点SPIRV.to_vec(), フラグメント.to_vec())?)
}
