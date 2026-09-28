//! 項目の見出しの読み取り。受け取るのはコードだけの行の一覧と見出しの行と位置のモジュール、返すのは、その行が `impl`・`trait`・`mod 名 {` の見出しなら、
//! 次に開く波括弧で開く本体(実装かトレイトの本体なら関数の持ち主の材料)である。依存を持たない純粋な関数だけを置く。
//! 走査の状態を持つ `関数を持つ位置の走査` から分けるのは、見出し1つから本体の区分を読む規則が、開いている波括弧の並びに触れないためである。

use super::{次に開く本体, 開いた波括弧};
use crate::conform::design_ontology::impl_header::implの見出しを読む;
use crate::conform::design_ontology::line_matching::{先頭の型引数を分ける, 先頭の識別子, 型の引数の名前一覧};
use crate::conform::design_ontology::module_path::モジュールパス;
use crate::conform::design_ontology::属性と可視性の前置き::属性と可視性を読み飛ばす;
use crate::design_model::{Rustの項目の種類, 設計概念の識別子, 設計概念への参照};
use blitz_design_verification::{型の表記, 関数の所有者};

/// 行が `impl`・`trait`・`mod 名 {` の見出しなら、その見出しが次に開く本体。位置のモジュールは、トレイトの節点の識別子に使う。
pub(super) fn 見出しから次に開く本体を読む(行一覧: &[String], 添字: usize, 位置のモジュール: &モジュールパス) -> Option<次に開く本体> {
    if let Some(見出し) = implの見出しを読む(行一覧, 添字) {
        let 構文 = 見出し.構文を読む()?;
        let 型引数名一覧 = 構文.型引数の名前一覧().to_vec();
        // トレイトの実装の関数は、同じ型の固有の実装の同じ名前の関数と別の処理であり、同じトレイトを型引数だけ変えて実装した関数どうしも別の処理である。
        // そのため所有者の並びへ、対象の型の名前に続けてトレイトを書いた位置の表記(型引数を含む)を足す。
        let 所有者の名前一覧 = [構文.対象.名前().to_string()].into_iter().chain(構文.トレイトの表記().map(str::to_string)).collect();
        let 本体 = 開いた波括弧::実装かトレイトの本体 {
            所有者の名前一覧,
            所有者: 関数の所有者::型(型の表記::生成する(構文.対象.表記(), 型引数名一覧.clone())),
            型引数名一覧,
        };
        return Some(次に開く本体 {
            読み飛ばす波括弧の数: 見出し.表記.matches('{').count().saturating_sub(1),
            本体,
        });
    }
    let 頭 = 属性と可視性を読み飛ばす(行一覧[添字].trim());
    let 本体 = match (頭.strip_prefix("mod "), トレイトの名前より後ろ(頭)) {
        (Some(後ろ), _) => モジュールの本体を読む(後ろ.trim_start())?,
        (None, Some(後ろ)) => トレイトの本体を読む(後ろ, 位置のモジュール)?,
        (None, None) => return None,
    };
    Some(次に開く本体 { 読み飛ばす波括弧の数: 0, 本体 })
}

// `trait` の名前より後ろから、トレイトの本体を組む。トレイトの `Self` は実装する型の総称であり名指す型が無いため、型引数の名前に数える。
fn トレイトの本体を読む(名前から: &str, 位置のモジュール: &モジュールパス) -> Option<開いた波括弧> {
    let 名前 = 先頭の識別子(名前から);
    let (型引数, _) = 先頭の型引数を分ける(名前から[名前.len()..].trim_start());
    let mut 型引数名一覧 = 型の引数の名前一覧(型引数);
    型引数名一覧.push("Self".to_string());
    let 識別子 = 設計概念の識別子::Rustの項目として生成する(位置のモジュール.表記(), &名前);
    (!名前.is_empty()).then(|| 開いた波括弧::実装かトレイトの本体 {
        所有者の名前一覧: vec![名前],
        所有者: 関数の所有者::トレイト(設計概念への参照::Rustの項目として生成する(識別子, Rustの項目の種類::トレイト)),
        型引数名一覧,
    })
}

// `trait` の見出しの行(前に `unsafe` と `auto` があってもよい)なら、`trait` より後ろ。
fn トレイトの名前より後ろ(頭: &str) -> Option<&str> {
    let 頭 = 頭.strip_prefix("unsafe ").map_or(頭, str::trim_start);
    let 頭 = 頭.strip_prefix("auto ").map_or(頭, str::trim_start);
    Some(頭.strip_prefix("trait")?.strip_prefix(char::is_whitespace)?.trim_start())
}

// `mod 名 {` の `mod` より後ろから、そのモジュールの本体。本体を別のファイルに持つ `mod 名;` は無しである。
fn モジュールの本体を読む(後ろ: &str) -> Option<開いた波括弧> {
    let 名前 = 先頭の識別子(後ろ);
    (!名前.is_empty() && 後ろ[名前.len()..].trim_start().starts_with('{')).then_some(開いた波括弧::モジュールの本体(名前))
}
