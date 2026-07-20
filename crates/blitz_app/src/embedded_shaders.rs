//! ビルド時に埋め込まれた初期シェーダー(SPIR-V)。ホットリロードで置き換わるまでの
//! 起動直後の1回だけに使う。ファイル名は`build_support::spirv_compile`の出力名と一致させる。

use blitz_render::シェーダー一式;

use crate::error::起動エラー;

const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv"));
const フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv"));

pub(crate) fn 埋め込みシェーダーを生成する() -> Result<シェーダー一式, 起動エラー> {
    Ok(シェーダー一式::生成する(頂点SPIRV.to_vec(), フラグメントSPIRV.to_vec())?)
}
