//! 頂点段のSPIR-Vへ、位置の組み込み出力を機材の最適化で揺らさない装飾(Invariant)を付ける工程。
//! 受け取るのは頂点段のSPIR-Vのバイト列、返すのは装飾を1つ足したバイト列である。
//!
//! この工程が要るのは、深度プリパスと色パスが同じ頂点段から同じ位置を出すことが、色パスの深度比較を等値にする条件そのものだからである
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。パイプラインが違えばドライバーが別の最適化を掛けうるため、
//! 同じソースであることは同じ値が出ることを保証しない。
//!
//! 注意: 生成器の側では付けられないことを実測で確かめてある。slangc 2025.11は`[[vk::invariant]]`を未知の属性として警告し、
//! `precise`は装飾を1つも残さない。SPIR-VのInvariantは変数にしか付けられないため、Slangのspirv_asmから値へ付けるとspirv-valが
//! 「変数でなければならない」で落ちる。生成物の語の並びへ直に足すのが、この版のslangcで装飾を実在させる唯一の手立てである。
//! 装飾が実在することは`crates/blitz_app/src/embedded_shaders/spirv_checks/position_invariant.rs`が機械的に見る。

mod decoration_scan;
mod error;
#[cfg(test)]
mod position_invariant_tests;
mod word_stream;

pub use error::位置の不変装飾エラー;

/// SPIR-VのOpDecorateの命令コード。
const 命令_OP_DECORATE: u16 = 71;
/// SPIR-Vの装飾BuiltInの値。
const 装飾_BUILT_IN: u32 = 11;
/// SPIR-Vの装飾Invariantの値。
const 装飾_INVARIANT: u32 = 18;
/// SPIR-Vの組み込みPositionの値。
const 組み込み_POSITION: u32 = 0;
/// OpDecorateへ装飾1つを足すときの語数(命令の先頭語・対象id・装飾)。
const 装飾命令の語数: u32 = 3;

pub fn 位置の不変装飾を付ける(spirv: &[u8]) -> Result<Vec<u8>, 位置の不変装飾エラー> {
    let 語一覧 = word_stream::語へ分解する(spirv)?;
    let 命令一覧 = word_stream::命令一覧を数える(&語一覧);
    let 宣言 = decoration_scan::位置の宣言を探す(&語一覧, &命令一覧).ok_or(位置の不変装飾エラー::位置の組み込み出力が無い)?;
    if decoration_scan::不変装飾が付いているか(&語一覧, &命令一覧, 宣言.変数id) {
        return Ok(word_stream::バイト列へ戻す(&語一覧));
    }
    Ok(word_stream::バイト列へ戻す(&装飾を挿し込む(&語一覧, 宣言)))
}

/// 位置の組み込み出力の宣言の直後へ不変装飾を挿し込む。装飾の並びの中へ入れるのは、SPIR-Vが装飾を型と変数の宣言より前の
/// 1つの区画へまとめることを要求するためである。
fn 装飾を挿し込む(語一覧: &[u32], 宣言: decoration_scan::位置の宣言) -> Vec<u32> {
    let 命令の先頭語 = (装飾命令の語数 << 16) | u32::from(命令_OP_DECORATE);
    let mut 結果 = Vec::with_capacity(語一覧.len() + 3);
    結果.extend_from_slice(&語一覧[..宣言.直後の語位置]);
    結果.extend_from_slice(&[命令の先頭語, 宣言.変数id, 装飾_INVARIANT]);
    結果.extend_from_slice(&語一覧[宣言.直後の語位置..]);
    結果
}
