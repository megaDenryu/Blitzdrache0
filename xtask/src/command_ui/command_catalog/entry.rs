//! コマンド一覧の1件を表す値オブジェクト。所有するのは、既存の説明文からASCIIコマンド名と
//! メニュー表示用の要約と引数の構文を取り出す責務である。日本語のコマンド名と、そのコマンドが
//! 受け取る引数の定義の並びを、この型が追加で持つ。

use super::argument::引数定義;

/// コマンド一覧の1件。`説明文`は"  ascii名 引数の構文  日本語の説明"の形式で統一されており、
/// ASCIIコマンド名は先頭の空白区切りの語から取り出せる。全文を1つのフィールドへ持つのは、
/// `cargo xtask`の一覧表示とメニューの使い方表示の両方が、同じ全文をそのまま必要とするためである。
/// 引数を取らないコマンドの`引数定義一覧`は空であり、メニューはそのとき何も尋ねずに実行する。
/// 3フィールドとも私有にして構築の口を下の2つへ絞り、引数の定義を書き忘れた項目を書き表せないようにする
/// (説明文の形式の不変条件はcommand_catalogの結合テストが全件について検査して守る)。
#[derive(Clone, Copy)]
pub(crate) struct コマンド項目 {
    日本語名: &'static str,
    説明文: &'static str,
    引数定義一覧: &'static [引数定義],
}

impl コマンド項目 {
    /// 引数を1つも解釈しないコマンドとして載せる。メニューは選ばれた時点で何も尋ねずに実行する。
    pub(super) const fn 引数なしで生成する(日本語名: &'static str, 説明文: &'static str) -> Self {
        Self {
            日本語名,
            説明文,
            引数定義一覧: &[],
        }
    }

    /// 引数を解釈するコマンドとして載せる。並びの順は、コマンド行へ語を並べる順そのものである。
    pub(super) const fn 引数定義を添えて生成する(
        日本語名: &'static str,
        説明文: &'static str,
        引数定義一覧: &'static [引数定義],
    ) -> Self {
        Self {
            日本語名,
            説明文,
            引数定義一覧,
        }
    }

    /// 日本語のコマンド名を返す。
    pub(crate) fn 日本語名(&self) -> &'static str {
        self.日本語名
    }

    /// このコマンドが受け取る引数の定義。空なら引数を1つも解釈しない。
    pub(crate) fn 引数定義一覧(&self) -> &'static [引数定義] {
        self.引数定義一覧
    }

    /// 説明文の先頭語をASCIIコマンド名として取り出す。全項目が"  ascii名 ..."の形式で
    /// 統一されているという前提に立つ(この前提はcommand_catalogの結合テストが全件検査で守る)。
    pub(crate) fn ascii名(&self) -> &'static str {
        self.説明文.split_whitespace().next().unwrap_or("")
    }

    /// `cargo xtask`の引数なし一覧表示に載せる説明文そのもの(先頭の空白による字下げを含む)。
    pub(crate) fn 全文(&self) -> &'static str {
        self.説明文
    }

    /// メニューが「使い方: cargo xtask 」に続けて表示する行。先頭の字下げだけを取り除いた説明文である。
    pub(crate) fn 使い方の行(&self) -> &'static str {
        self.説明文.trim_start()
    }

    /// 使い方の行のうち、ASCIIコマンド名に続く引数の構文の部分。構文と日本語の説明は半角空白2つ以上で
    /// 区切られており、引数を取らないコマンドではここが空になる。引数定義と表示の食い違いをテストが
    /// この部分だけで突き合わせる(日本語の説明の中の綴りは引数の宣言ではないため見ない)。
    pub(crate) fn 引数の構文(&self) -> &'static str {
        let 続き = self.使い方の行().strip_prefix(self.ascii名()).unwrap_or("");
        match 続き.split_once("  ") {
            Some((構文, _)) => 構文.trim(),
            None => 続き.trim(),
        }
    }

    /// メニューの一覧行に載せる要約。先頭のASCIIコマンド名を取り除いた残り(引数の構文と日本語の説明)を
    /// 文字数で切り詰める。ASCIIコマンド名は一覧行の別の場所へ既に表示するため、ここでは重複させない。
    pub(crate) fn 要約(&self, 上限文字数: usize) -> String {
        let 残り = self.使い方の行().strip_prefix(self.ascii名()).unwrap_or(self.使い方の行()).trim_start();
        let 全文字数 = 残り.chars().count();
        if 全文字数 <= 上限文字数 {
            return 残り.to_string();
        }
        let 先頭: String = 残り.chars().take(上限文字数).collect();
        format!("{先頭}…")
    }
}

#[cfg(test)]
mod tests;
