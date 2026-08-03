//! ビルド時に埋め込まれた遠方環境生成のSPIR-Vと、その組み立て。
//!
//! 大気のベイク済み画像の束と別に持つのは、遠方環境が大気の表ではなく、その上に置いた供給から作る間接照明の
//! 表現だからである。束へ混ぜると、大気の4枚を要るだけの読み戻し検査が遠方環境のシェーダーまで抱えることになる。
//! ファイル名は`build_support/atmosphere_spirv_compile.rs`の出力名と一致させる。

use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 遠方環境SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/distant_environment.spv"));

pub(crate) fn 埋め込み遠方環境シェーダーを生成する() -> Result<コンピュートシェーダー, 起動エラー> {
    Ok(コンピュートシェーダー::生成する(遠方環境SPIRV.to_vec())?)
}
