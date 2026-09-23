//! マクロの本体の中の宣言の読み口の試験。`macro_rules!` の本体はトークン木であるため、行の途中の `impl`・`type`・`use` が例外なしに違反になることと、
//! 行の頭の `use` と `type` の辺を作る名前(`use` の別名とパスの最後の名前、`type` の右辺)がマクロのメタ変数なら読み切れない違反になり、`$crate` の決まったパスなら読めることを固定する。
//! 実装の本体の中の関連型も名前の閉包の片方向の辺を作るため、右辺のメタ変数を同じく違反にする。

use super::tests::{ソース, 正規形の説明関数を連ねた違反の説明一覧};

#[test]
fn macro_rulesの本体に書いたimplとtypeとuseを行の頭でない予約語として違反にする() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "macro_rules! 生やす {\n    ($($名:ident),*) => { $( impl M不変データ for $名 {} )* };\n    ($名:ident) => { impl M不変データ for $名 {} };\n    () => { pub type 法則 = crate::x::規則; };\n    (別名) => { use crate::x::規則 as 法則; };\n}\n",
    )];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 4, "{説明一覧:?}");
    for (説明, 予約語) in 説明一覧.iter().zip(["impl", "impl", "type", "use"]) {
        assert!(説明.contains(&format!("`{予約語}` を行の頭でない位置に書いている")), "{説明}");
    }
}

#[test]
fn マクロの本体の中で辺を作る名前がメタ変数のuseとtypeを違反にし決まったパスは読む() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "macro_rules! 取り込む {\n    ($パス:path, $別名:ident) => {\n        use $パス as $別名;\n    };\n}\nmacro_rules! 名付ける {\n    ($元:ty) => {\n        pub type 別の法則 = $元;\n    };\n}\nmacro_rules! 辿れる {\n    () => {\n        use $crate::x::規則 as 法則;\n        type 別名 = $crate::x::規則;\n    };\n}\n",
    )];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧[0].contains("別名かパスの最後の名前にマクロのメタ変数がある"), "{}", 説明一覧[0]);
    assert!(説明一覧[1].contains("`=` の右辺にマクロのメタ変数がある"), "{}", 説明一覧[1]);
}

#[test]
fn 実装の本体の中の関連型も右辺のメタ変数を違反にする() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "macro_rules! 足し算を生やす {\n    ($型:ident) => {\n        impl std::ops::Add for $型 {\n            type Output = $型;\n            fn add(self, 右辺: Self) -> Self {\n                右辺\n            }\n        }\n    };\n}\n",
    )];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("`=` の右辺にマクロのメタ変数がある"), "{}", 説明一覧[0]);
}
