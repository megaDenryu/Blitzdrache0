//! ビルド時に埋め込まれた局所可視性補正のコンピュートシェーダー(SPIR-V)。
//! 自動露出と同じくホットリロードは非対応(ビルド時コンパイルのみ)であり、
//! ファイル名は`build_support/local_visibility_spirv_compile.rs`の出力名と一致させる。

use blitz_render::local_visibility::局所可視性のシェーダー一式;
use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 遮蔽の標本化SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/local_visibility_occlusion.spv"));
const 両側ぼかしSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/local_visibility_blur.spv"));

pub(crate) fn 埋め込み局所可視性シェーダーを生成する() -> Result<局所可視性のシェーダー一式, 起動エラー> {
    Ok(局所可視性のシェーダー一式 {
        遮蔽の標本化: コンピュートシェーダー::生成する(遮蔽の標本化SPIRV.to_vec())?,
        両側ぼかし: コンピュートシェーダー::生成する(両側ぼかしSPIRV.to_vec())?,
    })
}
