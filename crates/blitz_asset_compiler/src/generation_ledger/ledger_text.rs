//! 生成台帳の本文: 台帳1枚のテキストそのものを表す型。組み立てと解析をこの型のメソッドで閉じ、
//! 形式を知らない綴りが台帳として出入りすることを防ぐ。
//!
//! 1行目に形式宣言、続けて見出しの4行、その後にチャンク1つにつき1行を置く。欄の綴りをこのモジュールへ集め、
//! 組み立てる側の`build`と解析する側の`parse`が同じ綴りを見る。
//!
//! テキストにするのは、増分が効かなかったときに人が台帳を開いて原因を読めるようにするためである。

mod build;
mod parse;

use std::collections::BTreeMap;

use blitz_engine::チャンク座標;

use super::content_hash::内容ハッシュ;
use super::error::生成台帳エラー;
use super::heading::生成台帳の見出し;

pub(super) const 形式名: &str = "blitz_generation_ledger";
pub(super) const 対応版: &str = "1";
pub(super) const 種の欄: &str = "seed";
pub(super) const 種を持たないときの綴り: &str = "none";
pub(super) const 生成器の版の欄: &str = "generator_version";
pub(super) const 生成器の実行ファイルの欄: &str = "generator_image";
pub(super) const 焼き方の指定の欄: &str = "bake_options";
pub(super) const チャンクの欄: &str = "chunk";

/// テキスト1枚が表す台帳の中身。解析が返し、生成台帳がそのまま自分の中身として受け取る。
pub(super) struct 台帳のテキストが表す中身 {
    pub(super) 見出し: 生成台帳の見出し,
    pub(super) チャンクごとの内容ハッシュ: BTreeMap<チャンク座標, 内容ハッシュ>,
}

#[repr(transparent)]
pub(super) struct 生成台帳の本文(String);

impl 生成台帳の本文 {
    pub(super) fn 組み立てる(
        見出し: 生成台帳の見出し, チャンクごとの内容ハッシュ: &BTreeMap<チャンク座標, 内容ハッシュ>
    ) -> Self {
        Self(build::台帳のテキストを組み立てる(見出し, チャンクごとの内容ハッシュ))
    }

    /// ファイルから読んだ綴りを本文として受け取る。形式の検査は`解析する`が行うため、ここでは形を問わない。
    pub(super) fn 読み取った綴りから作る(綴り: String) -> Self {
        Self(綴り)
    }

    pub(super) fn 解析する(&self) -> Result<台帳のテキストが表す中身, 生成台帳エラー> {
        parse::台帳のテキストを解析する(&self.0)
    }

    /// 書き出す側だけが綴りへ戻す。ファイルへ書く1箇所のための口である。
    pub(super) fn 綴り(&self) -> &str {
        &self.0
    }
}
