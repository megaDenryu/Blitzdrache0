//! 標準ライブラリの名前の見分け。標準ライブラリのクレートの名前と、Rust の標準のプレリュードが修飾も取り込みも無しに見せるトレイトの名前を持ち、
//! 修飾の無い名前が、書いた位置の `use` で標準ライブラリから取り込んだ名前か、取り込みが無いプレリュードのトレイトの名前かを答える。
//! 修飾で書いたとき(`std::fmt::Display`)と取り込んで書いたとき(`use std::fmt::Display;` の後の `Display`)で、同じ標準ライブラリの項目の扱いを揃えるためである。
//! この問いは、書いた位置から見える定義が無いと分かった後に問う(同じ名前の定義がこのクレートに在れば、プレリュードの名前はそれに隠される)。

use std::path::PathBuf;

use super::super::module_path::モジュールパス;
use super::super::module_path::在るモジュールの一覧::在るモジュールの一覧;
use super::import_origin::{取り込み元の問い, 取り込み元を求めた結果};
use super::{型の在り処の問い, 探す定義の宣言};

/// 標準ライブラリのクレートの名前。
pub(super) const 標準ライブラリのクレート一覧: [&str; 3] = ["std", "core", "alloc"];

/// Rust 2024 の標準のプレリュードが、修飾も取り込みも無しに名前を見せるトレイト。
const プレリュードのトレイト一覧: [&str; 31] = [
    "Copy",
    "Send",
    "Sized",
    "Sync",
    "Unpin",
    "Drop",
    "Fn",
    "FnMut",
    "FnOnce",
    "AsMut",
    "AsRef",
    "From",
    "Into",
    "TryFrom",
    "TryInto",
    "Clone",
    "Default",
    "Eq",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "ToOwned",
    "ToString",
    "Iterator",
    "IntoIterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "Extend",
    "FromIterator",
    "Future",
    "IntoFuture",
];

impl 型の在り処の問い<'_> {
    /// 修飾の無い問いの名前が、書いた位置の `use` で標準ライブラリから取り込んだ名前か、取り込みが無いプレリュードのトレイトの名前か。
    /// 書いた位置から見える定義が無いと分かった後に問う。取り込み元が複数あるか別名のため決められないときは偽である。
    pub(super) fn 修飾の無い名前が標準ライブラリの項目か(&self, 宣言: 探す定義の宣言, ソース一覧: &[(PathBuf, Vec<String>)], 在るモジュール: &在るモジュールの一覧) -> bool {
        let Some(行一覧) = ソース一覧.iter().find(|(パス, _)| パス == self.参照元のパス).map(|(_, 行一覧)| 行一覧) else {
            return false;
        };
        let 字句位置一覧 = モジュールパス::ファイルのパスから求める(self.参照元のパス).行ごとの字句位置一覧(行一覧);
        let Some(位置) = 字句位置一覧.get(self.参照元の行番号.saturating_sub(1)) else {
            return false;
        };
        let 問い = 取り込み元の問い {
            型名: self.型名,
            実装の位置: 位置,
            在るモジュール,
        };
        match 問い.行一覧から取り込み元を求める(行一覧, &字句位置一覧) {
            取り込み元を求めた結果::取り込んでいる { モジュールパス: 取り込み元 } => 標準ライブラリのクレート一覧.contains(&取り込み元.クレート().表記()),
            取り込み元を求めた結果::取り込んでいない => 宣言 == 探す定義の宣言::トレイト && プレリュードのトレイト一覧.contains(&self.型名),
            取り込み元を求めた結果::取り込み元が複数ある | 取り込み元を求めた結果::別名のため取り込み元を求められない => false,
        }
    }
}
