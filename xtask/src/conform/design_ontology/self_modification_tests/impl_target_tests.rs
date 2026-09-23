//! 自己変更の禁止の試験のうち、実装の見出しの読み方(`unsafe impl`・パスで書いた対象の型・可変参照を対象にするトレイトの実装)を確かめるもの。
//! 先頭の識別子で対象の型を読むと `impl crate::a::規則` を `crate` の実装と読み、`unsafe impl` を実装と読まないため、どちらも黙って通っていた。
//! 別のモジュールを指すパスで書いた同名の型の実装も、名前が当たるため違反になる。それを台帳で除くことは `name_match_exclusion_tests.rs` が固定する。

use super::super::name_match_exclusion_ledger::{台帳の行, 名前が当たった別の型の実装の台帳};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧, 台帳を与えた自己変更の違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";

fn 違反が1件だけあり自己変更の違反である(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
}

#[test]
fn unsafeの付いたトレイトの実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\nunsafe trait 変更可能 {\n    fn 変える(&mut self);\n}\nunsafe impl 変更可能 for 規則 {\n    fn 変える(&mut self) {}\n}\n",
    )]);
}

#[test]
fn クレートの起点からのパスで書いた固有の実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![
        ソース("crates/a/src/a.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n"),
        ソース("crates/a/src/b.rs", "impl crate::a::規則 {\n    fn 変える(&mut self) {}\n}\n"),
    ]);
}

#[test]
fn 自分のモジュールからのパスで書いたトレイトの実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/a.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\ntrait 変更 {\n    fn 変える(&mut self);\n}\nimpl 変更 for self::規則 {\n    fn 変える(&mut self) {}\n}\n",
    )]);
}

#[test]
fn 可変参照を対象にするトレイトの実装は値で受けるselfでも違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\ntrait 変更 {\n    fn 変える(self);\n}\nimpl<'a> 変更 for &'a mut 規則 {\n    fn 変える(self) {}\n}\n",
    )]);
}

#[test]
fn 別のモジュールを指すパスで書いた同名の型の実装も名前で検査され台帳の行で除ける() {
    let ソース一覧 = || {
        vec![
            ソース(
                "crates/a/src/a.rs",
                "pub struct 規則;
impl M不変データ for 規則 {}
",
            ),
            ソース(
                "crates/a/src/c.rs",
                "pub struct 規則;
",
            ),
            ソース(
                "crates/a/src/b.rs",
                "impl crate::c::規則 {
    fn 変える(&mut self) {}
}
",
            ),
        ]
    };
    let 空の台帳 = 名前が当たった別の型の実装の台帳::行一覧から組む(Vec::new());
    let 説明一覧 = 台帳を与えた自己変更の違反の説明一覧(ソース一覧(), &空の台帳);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
    let 台帳 = 名前が当たった別の型の実装の台帳::行一覧から組む(vec![台帳の行 {
        マーカーの名前: "規則",
        パス: "crates/a/src/b.rs",
        見出し: "impl crate::c::規則",
        区分: "同じ名前の別の型",
        除外する理由: "この実装の対象は crate::c の別の型であり、マーカーを名乗る crate::a の型ではない",
    }]);
    assert!(台帳を与えた自己変更の違反の説明一覧(ソース一覧(), &台帳).is_empty());
}
