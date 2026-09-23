//! 名前が当たった別の型の実装の台帳のうち、陳腐化の試験。台帳に載っているのに自己変更を与える実装として見つからなくなった行を違反にすることを固定する。
//! 台帳の行が別の型の実装を除くこと自体は、その形ごとの試験(`definition_position_tests.rs`・`impl_target_tests.rs`・`../type_identity_tests.rs`・`../use_resolution_tests.rs`)が固定する。

use super::super::name_match_exclusion_ledger::{台帳の行, 名前が当たった別の型の実装の台帳};
use super::super::tests::{ソース, 台帳を与えた自己変更の違反の説明一覧};

fn 別の型の実装を除く台帳() -> 名前が当たった別の型の実装の台帳 {
    名前が当たった別の型の実装の台帳::行一覧から組む(vec![台帳の行 {
        マーカーの名前: "規則",
        パス: "crates/a/src/b.rs",
        見出し: "impl crate::c::規則",
        除外する理由: "この実装の対象は crate::c の別の型であり、マーカーを名乗る crate::a の型ではない",
    }])
}

#[test]
fn 該当しなくなった台帳の行は陳腐化として違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/a.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n")];
    let 説明一覧 = 台帳を与えた自己変更の違反の説明一覧(ソース一覧, &別の型の実装を除く台帳());
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("名前が当たった別の型の実装の台帳に載っている `impl crate::c::規則`"), "{説明一覧:?}");
}
