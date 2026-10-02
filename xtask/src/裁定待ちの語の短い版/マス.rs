//! 表の1行を縦棒で区切った1つ1つの値(マス)と、表の行をマスの並びへ分ける読み方。
//! マスは前後の空白を除いた値を持つ。見出しの行・区切りの行・語の行のどのマスもこの型で扱い、列の番号で取り出した値を裸の文字列として運ばない。

use super::原文の塊::本文の行;

#[derive(Clone, Copy)]
pub(super) struct マス<'a>(&'a str);

impl<'a> マス<'a> {
    /// 行の前後の空白を除き、両端の縦棒の内側を縦棒で区切って、マスごとに前後の空白を除く。両端が縦棒でなければ `None` を返す。
    pub(super) fn 表の行から分ける(行: &本文の行<'a>) -> Option<Vec<Self>> {
        let 内側 = 行.文字列().trim().strip_prefix('|')?.strip_suffix('|')?;
        Some(内側.split('|').map(|値| Self(値.trim())).collect())
    }

    pub(super) fn 空か(&self) -> bool {
        self.0.is_empty()
    }

    /// 表の区切りの行のマスか。空でなく、区切りの記号(`-` と `:`)だけで書かれているときに真を返す。
    pub(super) fn 区切りの記号だけで書かれているか(&self) -> bool {
        !self.空か() && self.0.chars().all(|文字| 文字 == '-' || 文字 == ':')
    }

    /// 見出しの行のマスが、指定した列の名前と等しいか。
    pub(super) fn 名前と等しいか(&self, 名前: &str) -> bool {
        self.0 == 名前
    }

    /// マスの値。既存の型(語の表記・区切られたドメイン・定義文の最初の文)へ包む境界でだけ使う。
    pub(super) fn 値(&self) -> &'a str {
        self.0
    }
}
