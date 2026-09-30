//! 読めない実装の見出しの検査の試験。型引数が閉じず本体を開く `{` に届かない `impl` の見出しが違反になることと、
//! rustfmt が次の行へ折った戻り値の型の `impl Iterator<…>,` の行が違反にならないことを固定する。

use std::path::PathBuf;

use super::scan_entry::原文一覧を検査する;
use super::syntax_checker::クレート構文検査;
use super::tests::ソース;

// conform の本番の工程(`scan_entry.rs` の `原文一覧を検査する`)に原文を通し、報告に載った違反の説明の一覧を返す。試験用に写した説明関数の並びを通さないのは、本番の並びから説明関数を外したことを見逃さないためである。
fn 本番の工程の違反の説明一覧(原文一覧: Vec<(&str, &str)>) -> Vec<String> {
    let 報告 = 原文一覧を検査する(原文一覧.into_iter().map(|(パス, 原文)| (PathBuf::from(パス), 原文.to_string())).collect());
    報告.違反一覧().iter().map(|違反| 違反.説明.clone()).collect()
}

// 反証: 読めない見出しを4か所がそれぞれ外したままにすると、この実装はどの検査にも現れず、違反が0件のまま通る。
#[test]
fn 構文解析_型引数が閉じず本体を開く波括弧に届かない実装の見出しは違反になる() {
    let 原文 = "pub struct 型;\nimpl<T: 甲<U> 型 {\n    fn 使う(&self) -> u8 {\n        0\n    }\n}\n";
    let 違反一覧 = クレート構文検査::生成する(vec![ソース("crates/a/src/x.rs", 原文)]).すべての実装の見出しを読めること().違反一覧();
    assert_eq!(違反一覧.len(), 1);
    assert_eq!(違反一覧[0].行番号, Some(2));
    assert!(違反一覧[0].説明.contains("型引数が閉じず本体を開く `{` に届かない"), "{}", 違反一覧[0].説明);
}

// 反証: 本番の説明関数の並びへ繋がないと、conform の実行で違反が1件も出ない。本番の工程の全体を通しても1件だけであることで、報告が1か所にまとまっていることも見る。
#[test]
fn 構文解析_読めない実装の見出しは本番の工程の全体で1件だけ違反になる() {
    let 説明一覧 = 本番の工程の違反の説明一覧(vec![("crates/a/src/x.rs", "pub struct 型;\nimpl<T: 甲<U> 型 {\n    fn 使う(&self) {}\n}\n")]);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("実装の見出しを読めない"), "{説明一覧:?}");
}

// 反証: 行の頭が `impl` の行をすべて実装の見出しとして読むと、関数の見出しの途中の型の表記を読めない見出しと数えて違反にする。
#[test]
fn 構文解析_折った戻り値の型の行は違反にならない() {
    let 関数 = "pub struct 地点;\n\npub fn 並べる<'a>(値: &'a 地点) -> Result<\n    impl Iterator<Item = 地点> + 'a,\n    地点,\n> {\n    Err(地点)\n}\n";
    let トレイト = "pub struct 地点;\n\npub trait 並べ手 {\n    fn 並べる<'a>(&'a self) -> Result<\n        impl Iterator<Item = 地点> + 'a,\n        地点,\n    >;\n}\n";
    let 説明一覧 = 本番の工程の違反の説明一覧(vec![("crates/a/src/x.rs", 関数), ("crates/a/src/y.rs", トレイト)]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
