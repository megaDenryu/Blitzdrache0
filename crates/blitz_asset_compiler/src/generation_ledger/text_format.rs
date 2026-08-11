//! 生成台帳のテキスト形式そのものの決め事。1行目に形式宣言、続けて見出しの3行、その後にチャンク1つにつき1行を置く。
//! 欄の綴りをこの1箇所へ集め、組み立てる側の`build`と解析する側の`parse`が同じ綴りを見る。
//!
//! テキストにするのは、増分が効かなかったときに人が台帳を開いて原因を読めるようにするためである。

mod build;
mod parse;

use std::collections::BTreeMap;

use super::content_hash::内容ハッシュ;
use super::heading::生成台帳の見出し;

pub(super) use build::台帳のテキストを組み立てる;
pub(super) use parse::台帳のテキストを解析する;

pub(super) const 形式名: &str = "blitz_generation_ledger";
pub(super) const 対応版: &str = "1";
pub(super) const 種の欄: &str = "seed";
pub(super) const 種を持たないときの綴り: &str = "none";
pub(super) const 生成器の版の欄: &str = "generator_version";
pub(super) const 生成器の実行ファイルの欄: &str = "generator_image";
pub(super) const チャンクの欄: &str = "chunk";

/// テキスト1枚が表す台帳の中身。解析が返し、生成台帳がそのまま自分の中身として受け取る。
pub(super) struct 台帳のテキストが表す中身 {
    pub(super) 見出し: 生成台帳の見出し,
    pub(super) チャンクごとの内容ハッシュ: BTreeMap<(i32, i32), 内容ハッシュ>,
}
