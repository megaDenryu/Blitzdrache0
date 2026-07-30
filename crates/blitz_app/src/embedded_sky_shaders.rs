//! ビルド時に埋め込まれた空パスのSPIR-Vと、方式ごとの腕の組み立て。担当するのは「起動指定の方式から
//! 空シェーダーの腕を1つ選ぶ」ことである。布シェーダーを`embedded_cloth_shaders`が持つのと同じ分け方である。
//!
//! 頂点は方式で変わらないため、両腕が同じSPIR-Vを使う。ファイル名は`build_support/sky_spirv_compile.rs`の
//! 出力名と一致させる。

use blitz_render::{シェーダー一式, 空シェーダー};

use crate::cli::空の方式指定;
use crate::error::起動エラー;

const 空頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_vertex.spv"));
const 空フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_fragment.spv"));
const 空大気LUTフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sky_atmosphere_fragment.spv"));

pub(crate) fn 埋め込み空シェーダーを生成する(方式: 空の方式指定) -> Result<空シェーダー, 起動エラー> {
    let フラグメント = match 方式 {
        空の方式指定::Hosek解析近似 => 空フラグメントSPIRV,
        空の方式指定::大気LUT => 空大気LUTフラグメントSPIRV,
    };
    let 一式 = シェーダー一式::生成する(空頂点SPIRV.to_vec(), フラグメント.to_vec())?;
    Ok(match 方式 {
        空の方式指定::Hosek解析近似 => 空シェーダー::Hosek解析近似(一式),
        空の方式指定::大気LUT => 空シェーダー::大気LUT(一式),
    })
}
