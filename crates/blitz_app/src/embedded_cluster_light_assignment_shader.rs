//! ビルド時に埋め込まれたクラスタの選別のコンピュートシェーダー(SPIR-V)。
//! 局所可視性補正と同じくホットリロードは非対応(ビルド時コンパイルのみ)であり、
//! ファイル名は`build_support/cluster_light_assignment_spirv_compile.rs`の出力名と一致させる。

use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 選別SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cluster_light_assignment.spv"));

pub(crate) fn 埋め込みクラスタ選別シェーダーを生成する() -> Result<コンピュートシェーダー, 起動エラー> {
    Ok(コンピュートシェーダー::生成する(選別SPIRV.to_vec())?)
}
