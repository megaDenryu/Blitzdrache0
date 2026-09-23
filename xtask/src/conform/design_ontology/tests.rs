//! 設計オントロジーの構文検査の試験。ソースの断片から `クレート構文検査` を作り、説明関数を連ねて違反の件数と説明を確かめる。

use std::path::{Path, PathBuf};

use super::super::source_lexing::コードだけの行一覧;
use super::syntax_checker::クレート構文検査;
use super::unbound_implementation_ledger::対象の型を決められない実装の台帳;

pub(super) fn ソース(名前: &str, 内容: &str) -> (PathBuf, Vec<String>) {
    (Path::new(名前).to_path_buf(), コードだけの行一覧(内容))
}

pub(super) fn 全部の説明関数を連ねた違反の説明一覧(ソース一覧: Vec<(PathBuf, Vec<String>)>) -> Vec<String> {
    クレート構文検査::生成する(ソース一覧)
        .すべてのコマンドが列挙型であること()
        .すべての規則が構造体であること()
        .すべての結果が列挙型であること()
        .すべての純粋データが参照と内部可変性を持たないこと()
        .すべての不変データが可変参照メソッドを持たないこと()
        .対象の型を決められない実装が自己変更を与えないこと(&対象の型を決められない実装の台帳::登録済みの台帳())
        .すべての引数オブジェクトが任意の値を持たないこと()
        .排他の分類を同時に名乗っていないこと()
        .設計解釈マーカーを別名で取り込んでいないこと()
        .設計解釈マーカーの実装が正規形であること()
        .設計解釈マーカーを再公開していないこと()
        .設計解釈マーカーの実装が波括弧付きのモジュールの中に無いこと()
        .違反一覧()
        .into_iter()
        .map(|違反| 違反.説明)
        .collect()
}

/// 正規形の説明関数だけを連ねた違反の説明の一覧。正規形の検査は、名前の重複と `use … as` の別名を意図して置く他の試験のソースの断片に当たるため、
/// 上の入口へ混ぜず、この専用の入口で確かめる。走査範囲の本番の入口(`scan_entry.rs`)は両方を連ねて走らせる。
pub(super) fn 正規形の説明関数を連ねた違反の説明一覧(ソース一覧: Vec<(PathBuf, Vec<String>)>) -> Vec<String> {
    クレート構文検査::生成する(ソース一覧)
        .型の表記が名前で追える形であること()
        .includeを呼んでいないこと()
        .取り込みの別名が宣言の名前を名乗っていないこと()
        .違反一覧()
        .into_iter()
        .map(|違反| 違反.説明)
        .collect()
}

#[test]
fn 構文解析_規則を満たす型は違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "#[derive(Debug, Clone)]\npub struct 位置 {\n    pub 東: f32,\n}\nimpl M不変データ for 位置 {}\nimpl M状態 for 位置 {}\npub(crate) enum 意図 {\n    静止,\n}\nimpl M不変データ for 意図 {}\nimpl blitz_design::Mコマンド for 意図 {}\npub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\nimpl 位置 {\n    pub fn 東へ動かした位置(&self) -> Self {\n        self.clone()\n    }\n}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_mコマンドとm規則の型定義を検証する() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 意図;\nimpl M不変データ for 意図 {}\nimpl Mコマンド for 意図 {}\npub enum 規則 {\n    甲,\n}\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains("Mコマンド `意図` は enum 定義が必要です"));
    assert!(説明一覧[1].contains("M規則 `規則` は struct 定義が必要です"));
}

#[test]
fn 構文解析_m不変データを実装する型は純粋データ規約を満たすこと() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 借り<'a> {\n    値: &'a f32,\n}\nimpl M不変データ for 借り<'_> {}\npub enum 事 {\n    起きた(std::sync::Mutex<u8>),\n}\nimpl M不変データ for 事 {}\nimpl Mイベント for 事 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains("M不変データ `借り` の定義は参照(&)を持てません"));
    assert!(説明一覧[1].contains("M不変データ `事` の定義は内部可変性 `Mutex` を持てません"));
}

#[test]
fn 構文解析_観測を実装する型にも純粋データ規約と可変参照メソッドの禁止を当てる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 移動の観測<'a> {\n    件数: &'a u32,\n}\nimpl M観測 for 移動の観測<'_> {}\npub struct 露出の観測;\nimpl M不変データ for 露出の観測 {}\nimpl M観測 for 露出の観測 {}\nimpl 露出の観測 {\n    pub fn 更新する(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains("M観測 `移動の観測` の定義は参照(&)を持てません"));
    assert!(説明一覧[1].contains("M不変データ `露出の観測` は自分の型への可変参照を受け手か引数に持つ関数を持てません"));
}

#[test]
fn 構文解析_定義が見つからない型を違反にする() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "impl M不変データ for 幻 {}\nimpl Mコマンド for 幻 {}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains("Mコマンド `幻` の定義(struct/enum)が見つかりません"));
    assert!(説明一覧[1].contains("M不変データ `幻` の定義(struct/enum)が見つかりません"));
}

#[test]
fn 構文解析_コメントと文字列の中に書かれた同じ文字列は数えない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "/// &mut self は持たない\npub struct 位置; // RefCell は使わない\nimpl M不変データ for 位置 {}\nimpl Mイベント for 位置 {}\nimpl 位置 {\n    pub fn 名前(&self) -> &'static str {\n        \"&mut self\"\n    }\n}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
