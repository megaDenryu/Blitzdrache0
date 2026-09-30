//! 関数境界の役割の検査の試験。`MParameter` を実装する型の定義の `Option<` のフィールドは違反になり、`MOptions` を実装する型の同じフィールドは違反にならないことを固定する。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_任意の値を持つmparameterは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "#[derive(Clone)]\npub struct 遷移パラメータ {\n    pub 経過時間: 経過時間,\n    pub 乱数の種: Option<u64>,\n}\nimpl M不変データ for 遷移パラメータ {}\nimpl MParameter for 遷移パラメータ {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("MParameter `遷移パラメータ` の定義は `Option<` のフィールドを持てません"));
    assert!(説明一覧[0].contains("任意の指定を束ねるなら `MOptions` を使う"));
}

#[test]
fn 構文解析_任意の値を持つmoptionsは違反にならず任意の値を持たないmparameterも違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "#[derive(Clone)]\npub struct 描画の設定 {\n    pub 幅: u32,\n    pub 題名: Option<String>,\n}\nimpl M不変データ for 描画の設定 {}\nimpl blitz_design::MOptions for 描画の設定 {}\n#[derive(Clone)]\npub struct 遷移パラメータ {\n    pub 経過時間: 経過時間,\n}\nimpl M不変データ for 遷移パラメータ {}\nimpl MParameter for 遷移パラメータ {}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
