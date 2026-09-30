//! 短い版の本文を行ごとに組み立てる。冒頭の2行(題と、生成したものであることの断り)を先に置き、見出しと語の行を読んだ順に足す。

use std::path::Path;

use super::生成のコマンド;
use super::裁定待ちの語::裁定待ちの語;

const 題の行: &str = "# 裁定待ちの語の短い定義";
const 語の行の頭: &str = "- **";

pub(super) struct 短い版の組み立て {
    行一覧: Vec<String>,
}

impl 短い版の組み立て {
    pub(super) fn 冒頭から始める(元の一覧のパス: &Path) -> Self {
        let 断り = format!(
            "このファイルは、`{生成のコマンド}` が `{}` から生成したものであり、手で編集しない。語ごとの使っている場所と既存語で表せないと考えた理由は、元のファイルにある。",
            元の一覧のパス.display()
        );
        Self {
            行一覧: vec![題の行.to_string(), String::new(), 断り, String::new()],
        }
    }

    pub(super) fn 見出しを書く(&mut self, 井桁の数: usize, 名前: &str) {
        if self.行一覧.last().is_some_and(|最後の行| !最後の行.is_empty()) {
            self.行一覧.push(String::new());
        }
        self.行一覧.push(format!("{} {名前}", "#".repeat(井桁の数)));
        self.行一覧.push(String::new());
    }

    pub(super) fn 語を書く(&mut self, 語: &裁定待ちの語<'_>) {
        self.行一覧.push(語.短い版の行());
    }

    pub(super) fn 仕上げる(self) -> 短い版の本文 {
        短い版の本文(self.行一覧.join("\n") + "\n")
    }
}

/// 短い版のファイルの本文。生成したものと、書かれているファイルから読んだものの両方をこの型で持ち、突き合わせる。
#[derive(PartialEq, Eq)]
pub(crate) struct 短い版の本文(String);

impl 短い版の本文 {
    pub(crate) fn 読んだ文字列から(文字列: String) -> Self {
        Self(文字列)
    }

    /// 語の行(`- **` で始まる行)の数。組み立てた語の数と独立に数え、1語が1行になったことを確かめるために使う。
    pub(crate) fn 語の数を数える(&self) -> usize {
        self.0.lines().filter(|行| 行.starts_with(語の行の頭)).count()
    }

    /// 相手と初めて食い違う行の番号(1から数える)。一致すれば `None` を返す。
    pub(crate) fn 初めて食い違う行番号(&self, 相手: &Self) -> Option<usize> {
        if self == 相手 {
            return None;
        }
        let 行番号 = self.0.lines().zip(相手.0.lines()).take_while(|(自分の行, 相手の行)| 自分の行 == 相手の行).count() + 1;
        Some(行番号)
    }

    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }
}
