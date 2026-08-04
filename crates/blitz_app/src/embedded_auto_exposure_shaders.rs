//! ビルド時に埋め込まれた自動露出のコンピュートシェーダー(SPIR-V)。
//! 明るさの圧縮と同じくホットリロードは非対応(ビルド時コンパイルのみ)であり、
//! ファイル名は`build_support/auto_exposure_spirv_compile.rs`の出力名と一致させる。

use blitz_render::auto_exposure::自動露出のシェーダー一式;
use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 集計SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/auto_exposure_histogram.spv"));
const 導出と適応SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/auto_exposure_resolve.spv"));

pub(crate) fn 埋め込み自動露出シェーダーを生成する() -> Result<自動露出のシェーダー一式, 起動エラー> {
    Ok(自動露出のシェーダー一式 {
        集計: コンピュートシェーダー::生成する(集計SPIRV.to_vec())?,
        導出と適応: コンピュートシェーダー::生成する(導出と適応SPIRV.to_vec())?,
    })
}
