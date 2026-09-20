//! 設計マーカーの実装の形を正規形へ固定する検査の試験。`blitz_design` の別名の取り込みと、ファイルの中の `mod` の中のマーカーの実装と、同じファイルの同名の定義の重複を確かめる。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 別名の違反: &str = "設計マーカーは `impl MDTO for 型` または `impl blitz_design::MDTO for 型` の形だけで実装する。別名で取り込むと conform が実装を認識できない";

#[test]
fn 構文解析_クレートの別名の取り込みは違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "use blitz_design as design;\npub struct 位置;\nimpl design::MDTO for 位置 {}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(別名の違反));
}

#[test]
fn 構文解析_波括弧の中のトレイトの別名の取り込みは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use blitz_design::{\n    MDTO as M純粋,\n    M状態,\n};\npub struct 位置;\nimpl M純粋 for 位置 {}\nimpl M状態 for 位置 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(別名の違反));
}

#[test]
fn 構文解析_別名でない取り込みは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use blitz_design::{MDTO, M状態};\nuse std::fmt::Debug as 表示;\npub struct 位置;\nimpl MDTO for 位置 {}\nimpl M状態 for 位置 {}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_modの中のマーカーの実装は違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "pub struct 位置;\nmod 内側 {\n    impl super::MDTO for super::位置 {}\n    impl MDTO for 位置 {}\n}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("設計マーカーの実装をファイルの中の `mod` の中へ置かない"));
}

#[test]
fn 構文解析_マーカーの実装を持たないmod_testsは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 位置;\nimpl MDTO for 位置 {}\n#[cfg(test)]\nmod tests {\n    use super::*;\n    #[test]\n    fn 複製できる() {\n        let _ = 位置.clone();\n    }\n}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_同じファイルに同名の定義が複数あれば黙って通さず違反にする() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "mod a {\n    struct 値;\n}\nmod b {\n    struct 値<'a> {\n        参照: &'a i32,\n    }\n}\nimpl MDTO for 値<'_> {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("MDTO `値` の同名の定義が複数あり一意に決まらない"));
}
