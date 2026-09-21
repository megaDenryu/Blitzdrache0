//! Rustのソースから `mod` の宣言を抽出する工程。受け取るのは宣言元のファイルのパスと原文、返すのはモジュール宣言の一覧と、どの宣言にも結び付かない `#[path]` の属性の行である。
//! 照合はコメントと文字列を落としたコードだけの行に対して行う。コードだけの行は文字列も落とすため、`#[path]` に書かれた表記は字句の走査から別に受け取る。
//! `#[path]` が無い宣言も抽出する。この概念がRustのモジュールの宣言であって、`#[path]` が付いた宣言ではないためである。既定の探索では物理と論理が一致することがRustの規則から従うため、それらは一致の検査を必ず通る。
//! 属性と `mod` の間に挟まれる可視性(`pub`・`pub(crate)`・`pub(super)`)と他の属性(`#[cfg(test)]`)と空行は飛ばす。波括弧付きの `mod 名前 { ... }` は本体の形として観測し、黙って対象から外さない。

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::super::source_lexing::{コードだけの行一覧, 文字列リテラル一覧};
use super::line_matching::先頭の識別子;
use super::module_declaration::{モジュール宣言, 本体の形, 置き場の指定};

pub struct モジュール宣言の抽出 {
    宣言元のファイル: PathBuf,
    行一覧: Vec<String>,
    行ごとの文字列一覧: Vec<(usize, String)>, // 属性に書かれた表記を引くための、行番号と文字列リテラルの中身の対
}

impl モジュール宣言の抽出 {
    pub fn 生成する(宣言元のファイル: PathBuf, 原文: &str) -> Self {
        let 行ごとの文字列一覧 = 文字列リテラル一覧(原文).into_iter().map(|断片| (断片.開始行, 断片.中身)).collect();
        Self {
            宣言元のファイル,
            行一覧: コードだけの行一覧(原文),
            行ごとの文字列一覧,
        }
    }

    pub fn 宣言一覧(&self) -> Vec<モジュール宣言> {
        let 属性の対応表 = self.path属性の対応表();
        self.行一覧
            .iter()
            .enumerate()
            .filter_map(|(添字, 行)| 行の中のmodの宣言(行).map(|(モジュール名, 本体の形)| self.宣言にする(添字, モジュール名, 本体の形, 属性の対応表.get(&添字).copied())))
            .collect()
    }

    /// どの宣言にも結び付かない `#[path]` の属性の行番号。属性の後ろに `mod` の宣言が無い形を、黙って読み飛ばさずに報告するためである。
    pub fn 宣言に結び付かないpath属性の行一覧(&self) -> Vec<usize> {
        self.path属性の行一覧().filter(|添字| self.属性に続く宣言の添字(*添字).is_none()).map(|添字| 添字 + 1).collect()
    }

    fn 宣言にする(&self, 添字: usize, モジュール名: String, 本体の形: 本体の形, 属性の行番号: Option<usize>) -> モジュール宣言 {
        モジュール宣言 {
            宣言元のファイル: self.宣言元のファイル.clone(),
            行番号: 添字 + 1,
            モジュール名,
            置き場の指定: self.置き場の指定を求める(属性の行番号),
            本体の形,
        }
    }

    fn 置き場の指定を求める(&self, 属性の行番号: Option<usize>) -> 置き場の指定 {
        let Some(行番号) = 属性の行番号 else {
            return 置き場の指定::既定の探索に任せる;
        };
        match self.行ごとの文字列一覧.iter().find(|(行, _)| *行 == 行番号) {
            Some((_, 中身)) => 置き場の指定::属性で明示する(中身.clone()),
            None => 置き場の指定::属性の表記を読めない,
        }
    }

    /// 宣言の行の添字から、その宣言に付く `#[path]` の属性の行番号を引く表。
    fn path属性の対応表(&self) -> BTreeMap<usize, usize> {
        self.path属性の行一覧().filter_map(|添字| self.属性に続く宣言の添字(添字).map(|宣言の添字| (宣言の添字, 添字 + 1))).collect()
    }

    fn path属性の行一覧(&self) -> impl Iterator<Item = usize> + '_ {
        self.行一覧.iter().enumerate().filter(|(_, 行)| 行.trim_start().starts_with("#[path")).map(|(添字, _)| 添字)
    }

    /// 属性の行から、その属性が付く `mod` の宣言の行の添字。間の空行と他の属性は飛ばす。同じ行に宣言が続く形では属性の行そのものである。
    fn 属性に続く宣言の添字(&self, 属性の添字: usize) -> Option<usize> {
        if self.行一覧.get(属性の添字).and_then(|行| 行の中のmodの宣言(行)).is_some() {
            return Some(属性の添字);
        }
        let 飛ばす行か = |行: &&String| 行.trim().is_empty() || 行.trim_start().starts_with("#[");
        let (添字, 行) = self.行一覧.iter().enumerate().skip(属性の添字 + 1).find(|(_, 行)| !飛ばす行か(行))?;
        行の中のmodの宣言(行).map(|_| 添字)
    }
}

/// 行の中の `mod` の宣言のモジュール名と本体の形。同じ行の前に在る属性と可視性は飛ばす。`mod` で始まらない行は宣言でない。
fn 行の中のmodの宣言(行: &str) -> Option<(String, 本体の形)> {
    let 残り = 可視性を飛ばす(属性を飛ばす(行.trim_start())).strip_prefix("mod")?;
    if !残り.starts_with(char::is_whitespace) {
        return None;
    }
    let 残り = 残り.trim_start();
    let モジュール名 = 先頭の識別子(残り);
    if モジュール名.is_empty() {
        return None;
    }
    match 残り.get(モジュール名.len()..)?.trim_start().chars().next()? {
        ';' => Some((モジュール名, 本体の形::別のファイル)),
        '{' => Some((モジュール名, 本体の形::波括弧の中)),
        _ => None,
    }
}

fn 属性を飛ばす(行: &str) -> &str {
    if 行.starts_with("#[") { 行.rsplit_once(']').map_or("", |(_, 残り)| 残り.trim_start()) } else { 行 }
}

fn 可視性を飛ばす(行: &str) -> &str {
    let Some(残り) = 行.strip_prefix("pub") else {
        return 行;
    };
    let 残り = if 残り.starts_with('(') { 残り.split_once(')').map_or("", |(_, 後ろ)| 後ろ) } else { 残り };
    残り.trim_start()
}
