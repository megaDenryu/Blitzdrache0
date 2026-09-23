//! 走査範囲のトレイトの宣言を、宣言のクレートと名前の組で引く索引。全ソースから1度だけ組み、組ごとに、宣言の本体の直下の関数の署名の一覧を持つ。
//! 参照するのは、実装側が取り込み元から決めた宣言のクレートの候補(`implemented_trait/declaring_crate.rs`)の宣言であり、候補のどのクレートにも宣言が無ければ同名の全宣言である。
//! 参照した宣言が複数あるときは、1つでも自己変更を与える関数があれば与えると答える(一意に決まらない宣言を違反の側へ倒す)。
//! 読む見出しは `trait`・`pub trait`・`pub(crate) trait`・`unsafe trait`・`pub unsafe trait` と、型引数・`where`・上位トレイトを持つものであり、前に同じ行で属性(`#[..] trait`)を書いてもよい。既定の関数の `where Self: Sized` も除外しない。
//! 上位トレイトは辿らない。`impl 下位 for 型` が成り立つには `型: 上位` が要り、それは明示の `impl 上位 for 型`(型に属する実装として別に読む)か全称の実装(対象の型を決められない実装として別に読む)でしか満たせないためである。
//! 走査範囲の外のトレイト(std と依存クレート)は索引に無い。索引に無いトレイトを検査しなくてよいかは、呼び出し側が `implemented_trait.rs` で判定する。

use std::collections::HashMap;
use std::path::PathBuf;

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::function_signature::{本体の直下の関数の署名一覧, 自己変更を問う対象, 関数の署名};
use super::impl_header::宣言の見出しを読む;
use super::line_matching::先頭の識別子;
use super::module_path::モジュールパス;

/// トレイトの宣言がありうるクレートの候補の一覧。実装が書いたトレイトの取り込み元から決め(`implemented_trait/declaring_crate.rs`)、この索引を引く鍵にする。
pub struct トレイトの宣言のクレートの候補(pub Vec<モジュールパス>);

pub struct トレイトの宣言の索引 {
    クレートと名前ごとの関数一覧: HashMap<(モジュールパス, String), Vec<関数の署名>>, // 同じクレートに同名の宣言が複数あれば、その関数を1つの列へ連ねる(関数を1つも持たない宣言も空の列として持つ)
}

impl トレイトの宣言の索引 {
    pub fn 全ソースから組む(ソース一覧: &[(PathBuf, Vec<String>)]) -> Self {
        let mut クレートと名前ごとの関数一覧: HashMap<(モジュールパス, String), Vec<関数の署名>> = HashMap::new();
        for (パス, 行一覧) in ソース一覧 {
            let クレート = モジュールパス::ファイルのパスから求める(パス).クレート();
            for (開始, 行) in 行一覧.iter().enumerate() {
                let Some(名前) = トレイトの宣言の名前(行) else {
                    continue;
                };
                let Some(見出し) = 宣言の見出しを読む(行一覧, 開始) else {
                    continue;
                };
                クレートと名前ごとの関数一覧.entry((クレート.clone(), 名前)).or_default().extend(本体の直下の関数の署名一覧(&見出し.本体の文字列(行一覧)));
            }
        }
        Self { クレートと名前ごとの関数一覧 }
    }

    /// 候補のクレートのその名前のトレイトの宣言を参照し(候補のどれにも無ければ同名の全宣言を参照し)、対象へ自己変更を与える関数を宣言しているかを答える。
    /// 走査範囲に同名の宣言が1つも無ければ、そう答える(呼び出し側が走査範囲の外かを判定する)。
    pub fn 宣言を参照する(&self, 候補: &トレイトの宣言のクレートの候補, トレイトの名前: &str, 対象: &自己変更を問う対象) -> トレイトの宣言を参照した結果 {
        let 候補の宣言一覧: Vec<&Vec<関数の署名>> = 候補.0.iter().filter_map(|クレート| self.クレートと名前ごとの関数一覧.get(&(クレート.clone(), トレイトの名前.to_string()))).collect();
        let 宣言一覧 = if 候補の宣言一覧.is_empty() {
            self.クレートと名前ごとの関数一覧.iter().filter(|((_, 名前), _)| 名前 == トレイトの名前).map(|(_, 関数一覧)| 関数一覧).collect()
        } else {
            候補の宣言一覧
        };
        if 宣言一覧.is_empty() {
            return トレイトの宣言を参照した結果::宣言が無い;
        }
        match 宣言一覧.into_iter().flatten().find(|関数| 対象.自己変更を与えるか(関数)) {
            Some(関数) => トレイトの宣言を参照した結果::自己変更を与える { 関数名: 関数.名前.clone() },
            None => トレイトの宣言を参照した結果::自己変更を与えない,
        }
    }
}

/// トレイトの名前で走査範囲の宣言を参照した結果。
pub enum トレイトの宣言を参照した結果 {
    宣言が無い,
    自己変更を与えない,
    自己変更を与える { 関数名: String },
}

// トレイトの宣言の見出しの行なら、そのトレイトの名前。同じ行の属性と可視性(`pub`・`pub(...)`)と `unsafe` を読み飛ばす。
fn トレイトの宣言の名前(行: &str) -> Option<String> {
    let mut 残り = 属性と可視性を読み飛ばす(行);
    if let Some(後ろ) = 残り.strip_prefix("unsafe").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)) {
        残り = 後ろ.trim_start();
    }
    let 後ろ = 残り.strip_prefix("trait")?;
    if !後ろ.starts_with(char::is_whitespace) {
        return None;
    }
    let 名前 = 先頭の識別子(後ろ.trim_start());
    (!名前.is_empty()).then_some(名前)
}
