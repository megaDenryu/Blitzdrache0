//! 設計解釈マーカーの実装の形を正規形へ固定する検査の試験のうち、`blitz_design` の別名の取り込みと、ファイルの中の `mod` の中のマーカーの実装と、同じファイルの同名の定義の重複を確かめるもの。正規形を別名なしで迂回する書き方の試験は `marker_canonical_form_tests.rs` にある。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 別名の違反: &str = "設計解釈マーカーは `impl M不変データ for 型` または `impl blitz_design::M不変データ for 型` の形だけで実装する。別名で取り込むと conform が実装を認識できない";
pub(super) const 正規形でない実装の違反: &str = "設計解釈マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形だけで書く。再公開・絶対パス・パスの中の空白のどれも、構文検査が実装として認識できない";

#[test]
fn 構文解析_クレートの別名の取り込みは違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "use blitz_design as design;\npub struct 位置;\nimpl design::M不変データ for 位置 {}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains(別名の違反));
    assert!(説明一覧[1].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_波括弧の中のトレイトの別名の取り込みは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use blitz_design::{\n    M不変データ as M純粋,\n    M状態,\n};\npub struct 位置;\nimpl M純粋 for 位置 {}\nimpl M状態 for 位置 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(別名の違反));
}

#[test]
fn 構文解析_別名でない取り込みは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use blitz_design::{M不変データ, M状態};\nuse std::fmt::Debug as 表示;\npub struct 位置;\nimpl M不変データ for 位置 {}\nimpl M状態 for 位置 {}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_modの中のマーカーの実装は違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "pub struct 位置;\nmod 内側 {\n    impl super::M不変データ for super::位置 {}\n    impl M不変データ for 位置 {}\n}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 3, "{説明一覧:?}");
    assert!(説明一覧[0].contains("M不変データ `位置` の同名の定義が複数あり一意に決まらない"));
    assert!(説明一覧[1].contains(正規形でない実装の違反));
    assert!(説明一覧[2].contains("設計解釈マーカーの実装をファイルの中の `mod` の中へ置かない"));
}

#[test]
fn 構文解析_マーカーの実装を持たないmod_testsは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 位置;\nimpl M不変データ for 位置 {}\n#[cfg(test)]\nmod tests {\n    use super::*;\n    #[test]\n    fn 複製できる() {\n        let _ = 位置.clone();\n    }\n}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_同じファイルに同名の定義が複数あれば黙って通さず違反にする() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "mod a {\n    struct 値;\n}\nmod b {\n    struct 値<'a> {\n        参照: &'a i32,\n    }\n}\nimpl M不変データ for 値<'_> {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `値` の同名の定義が複数あり一意に決まらない"));
}
