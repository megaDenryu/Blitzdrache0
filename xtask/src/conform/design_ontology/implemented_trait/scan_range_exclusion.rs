//! 実装が書いたトレイトのうち、トレイトの宣言の索引に宣言が無いものを検査しなくてよいかの判定。受け取るのはモジュールの索引、返すのは検査しなくてよいかである。
//! 索引に宣言が無いトレイトは、次の4つのどれかなら検査しない。(1) `std`・`core`・`alloc` か、実装のクレートの外部の依存クレート(依存の白リストのうち、ワークスペースのクレートでないもの)で始まるパス。
//! (2) それらのクレートから明示の `use` で取り込んだ名前(`use std::fmt::Display;`)と、それらのクレートのモジュールを取り込んで書いたパス(`use std::fmt;` の後の `fmt::Display`)。
//! (3) std の prelude と derive で使う名前の固定の一覧(明示の `use` で取り込んでいない名前に限る)。(4) 設計解釈マーカーの名前。(1) から (3) は宣言を読めないが、実在を言語と Cargo が保証する。
//! 明示の `use` で取り込んでいない修飾の無い名前で、実装の位置のモジュールの glob の取り込み元(`use std::fmt::*;`)がすべて (1) のクレートであるものも、(2) と同じく検査しない。
//! (4) はマーカーの宣言が自己変更を与える関数を持たないことを正本 `blitz_design` が定め、その宣言が走査範囲にあるときは索引が先に参照する。
//! 明示の `use` で取り込んでいない名前(パスの起点を含む)は、実装のクレートかこのファイルの `use` の取り込み元のクレートのどこかで `use … as 名前` の別名として付けられていれば、4つのどれとしても扱わない。
//! `pub use crate::t::可変化 as Default;` を glob か2段の再公開で取り込んだ `impl Default for 型` を、prelude の名前として通さないためである。
//! これ以外の索引に無いトレイト(`crate::`・`self::`・`super::` で始まるパスや、ワークスペースのクレートのパス)は宣言を読めないため、黙って通さず根拠にする。

use std::path::Path;

use super::super::super::dependency_whitelist::外部の依存クレートのパスの名前一覧;
use super::super::line_matching::{クレート名, パスの最後の名前};
use super::super::module_index::モジュールの索引;
use super::super::module_path::モジュールパス;
use super::super::syntax_patterns::オントロジートレイト;
use super::super::use_resolution::取り込みの項目;
use super::実装したトレイト;

/// 注意: 取り込まずに書ける std のトレイトの名前を空白で区切って並べた一覧である。Rust 2021 の std の prelude にあるトレイトと、derive で実装を生やす std のトレイト(`Debug`・`Hash`)を置く。
/// prelude に無い名前をここへ足すと、同名の別のトレイトの実装を宣言を読まずに通すことになる。
const 取り込まずに書けるトレイトの名前の並び: &str =
    "Clone Copy Debug Default PartialEq Eq PartialOrd Ord Hash Drop From Into TryFrom TryInto AsRef AsMut Iterator IntoIterator DoubleEndedIterator ExactSizeIterator Extend FromIterator ToString ToOwned Send Sync Sized Unpin Fn FnMut FnOnce";

/// 宣言が走査範囲の外にある標準のクレートの名前。
const 標準のクレート名一覧: [&str; 3] = ["std", "core", "alloc"];

impl 実装したトレイト<'_> {
    // 索引に宣言が無くても検査しなくてよいトレイトか(冒頭の4つ)。明示の取り込みが無く、別名として付けられた名前は、どれとしても扱わない。
    // 起点を名乗る明示の取り込みが複数(cfg で切り替わる取り込み)あれば、全部の取り込み元が走査範囲の外のクレートであるときに限る。
    pub(super) fn 走査範囲の外か(&self, モジュールの索引: &モジュールの索引) -> bool {
        let 名前 = パスの最後の名前(self.表記);
        let パス = self.パス();
        let 起点 = self.起点();
        let 自分 = self.実装.位置のモジュール(モジュールの索引);
        let 項目一覧 = モジュールの索引.取り込みの項目一覧(&自分);
        let 明示の取り込み一覧: Vec<&取り込みの項目> = 項目一覧.iter().filter(|項目| 項目.名乗る名前() == 起点).collect();
        if 明示の取り込み一覧.is_empty() && self.別名として付けられているか(モジュールの索引, &自分, 起点) {
            return false;
        }
        if オントロジートレイト::全部の一覧().iter().any(|マーカー| マーカー.名前() == 名前) {
            return true;
        }
        if 起点 == パス && 明示の取り込み一覧.is_empty() && (取り込まずに書けるトレイトの名前か(名前) || globの取り込み元がすべて走査範囲の外のクレートか(モジュールの索引, &自分, self.実装.パス))
        {
            return true;
        }
        if 明示の取り込み一覧.is_empty() {
            return 走査範囲の外のクレートか(self.実装.パス, 起点);
        }
        明示の取り込み一覧
            .iter()
            .all(|項目| 走査範囲の外のクレートか(self.実装.パス, 項目.パス.trim_start_matches("::").split("::").next().unwrap_or_default().trim()))
    }

    // 名前が、実装のクレートか、このファイルの `use` の取り込み元のクレートのどこかで、`use … as 名前` の別名として付けられているか。
    fn 別名として付けられているか(&self, モジュールの索引: &モジュールの索引, 自分: &モジュールパス, 名前: &str) -> bool {
        let 取り込み元のクレート一覧 = モジュールの索引.取り込みの項目一覧(自分).iter().map(|項目| モジュールの索引.書かれたパスのクレート(自分, &項目.パス));
        std::iter::once(自分.クレート())
            .chain(取り込み元のクレート一覧)
            .any(|クレート| モジュールの索引.クレートで別名として付けられているか(&クレート, 名前))
    }
}

// 取り込まずに書ける std のトレイトの名前か。
pub(super) fn 取り込まずに書けるトレイトの名前か(名前: &str) -> bool {
    取り込まずに書けるトレイトの名前の並び.split_whitespace().any(|書ける名前| 書ける名前 == 名前)
}

// そのクレートの名前が、宣言を走査範囲の外に持つクレート(std・core・alloc か、実装のクレートの外部の依存クレート)か。
fn 走査範囲の外のクレートか(実装のパス: &Path, 名前: &str) -> bool {
    標準のクレート名一覧.contains(&名前) || 外部の依存クレートのパスの名前一覧(&クレート名(実装のパス).to_string_lossy()).iter().any(|依存| 依存 == 名前)
}

// 実装の位置のモジュールに glob の取り込みが1つ以上あり、その取り込み元がすべて走査範囲の外のクレートか。修飾の無い名前がそこからしか来られないなら、宣言を走査範囲で探さない。
pub(super) fn globの取り込み元がすべて走査範囲の外のクレートか(索引: &モジュールの索引, 自分: &モジュールパス, 実装のパス: &Path) -> bool {
    let mut globの取り込み元一覧 = 索引.取り込みの項目一覧(自分).iter().filter(|項目| 項目.全部を取り込むか()).map(|項目| 索引.書かれたパスのクレート(自分, &項目.パス)).peekable();
    globの取り込み元一覧.peek().is_some() && globの取り込み元一覧.all(|クレート| 走査範囲の外のクレートか(実装のパス, クレート.表記()))
}
