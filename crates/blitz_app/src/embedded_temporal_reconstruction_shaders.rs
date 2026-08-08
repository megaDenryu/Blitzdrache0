//! ビルド時に埋め込まれた時間再構成の頂点段と画素段(SPIR-V)。
//! 明るさの圧縮と同じくホットリロードは非対応(ビルド時コンパイルのみ)であり、
//! ファイル名は`build_support/temporal_reconstruction_spirv_compile.rs`の出力名と一致させる。

use blitz_render::temporal_reconstruction::時間再構成のシェーダー一式;
use blitz_render::シェーダー一式;

use crate::error::起動エラー;

const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/temporal_reconstruction_vertex.spv"));
const 画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/temporal_reconstruction_fragment.spv"));

pub(crate) fn 埋め込み時間再構成シェーダーを生成する() -> Result<時間再構成のシェーダー一式, 起動エラー> {
    Ok(時間再構成のシェーダー一式 {
        再構成: シェーダー一式::生成する(頂点SPIRV.to_vec(), 画素段SPIRV.to_vec())?,
    })
}
