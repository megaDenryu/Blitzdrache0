//! 分類の排他と網羅の検査の試験。排他の組の両方を名乗る型は違反になり、片方だけなら違反にならないことと、
//! 下位の分類で分ける上位の分類を名乗る型が下位の分類のどれも名乗らなければ違反になることを固定する。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 識別子付きの旅行者: &str = "#[derive(Clone)]\npub struct 旅行者 {\n    識別子: 旅行者の識別子,\n}\nimpl Mエンティティ for 旅行者 {\n    type 識別子 = 旅行者の識別子;\n    fn 識別子(&self) -> &旅行者の識別子 {\n        &self.識別子\n    }\n}\n";

#[test]
fn 構文解析_不変エンティティと可変エンティティを同時に名乗る型は違反になる() {
    let 内容 = format!("{識別子付きの旅行者}impl M不変データ for 旅行者 {{}}\nimpl M不変エンティティ for 旅行者 {{}}\nimpl blitz_design::M可変エンティティ for 旅行者 {{}}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M可変エンティティ `旅行者` は `M不変エンティティ` も名乗っている"));
    assert!(説明一覧[0].contains("どちらか一方だけを名乗る"));
}

#[test]
fn 構文解析_mparameterとmoptionsを同時に名乗る型は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "#[derive(Clone)]\npub struct 描画の設定 {\n    pub 幅: u32,\n}\nimpl M不変データ for 描画の設定 {}\nimpl MParameter for 描画の設定 {}\nimpl MOptions for 描画の設定 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("MOptions `描画の設定` は `MParameter` も名乗っている"));
}

#[test]
fn 構文解析_排他の分類の片方だけを名乗る型は違反にならない() {
    let 内容 =
        format!("{識別子付きの旅行者}impl M可変エンティティ for 旅行者 {{}}\n#[derive(Clone)]\npub struct 遷移パラメータ {{\n    pub 経過時間: 経過時間,\n}}\nimpl M不変データ for 遷移パラメータ {{}}\nimpl MParameter for 遷移パラメータ {{}}\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]).is_empty());
}

#[test]
fn 構文解析_別のモジュールの同名の型が排他の分類の片方ずつを名乗るのは違反にならない() {
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", "#[derive(Clone)]\npub struct 設定;\nimpl M不変データ for 設定 {}\nimpl MParameter for 設定 {}\n"),
        ソース("crates/a/src/y.rs", "#[derive(Clone)]\npub struct 設定;\nimpl M不変データ for 設定 {}\nimpl MOptions for 設定 {}\n"),
    ];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

const 解釈前の入力の定義: &str = "#[derive(Clone)]\npub struct キーの押下 {\n    pub 上: bool,\n}\nimpl M不変データ for キーの押下 {}\nimpl M解釈前の入力 for キーの押下 {}\n";

#[test]
fn 構文解析_生入力と中間入力を同時に名乗る型は違反になる() {
    let 内容 = format!("{解釈前の入力の定義}impl M生入力 for キーの押下 {{}}\nimpl M中間入力 for キーの押下 {{}}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M中間入力 `キーの押下` は `M生入力` も名乗っている"));
}

#[test]
fn 構文解析_生入力と中間入力のどちらも名乗らない解釈前の入力は違反になる() {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 解釈前の入力の定義)]);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M解釈前の入力 `キーの押下` は `M生入力` と `M中間入力` のどちらも名乗っていない"));
}

#[test]
fn 構文解析_生入力か中間入力の片方だけを名乗る解釈前の入力は違反にならない() {
    for 下位 in ["M生入力", "blitz_design::M中間入力"] {
        let 内容 = format!("{解釈前の入力の定義}impl {下位} for キーの押下 {{}}\n");
        assert!(全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]).is_empty(), "{下位}");
    }
}

#[test]
fn 構文解析_別のモジュールの同名の型が生入力を名乗っても解釈前の入力の網羅は満たされない() {
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", 解釈前の入力の定義),
        ソース(
            "crates/a/src/y.rs",
            "#[derive(Clone)]\npub struct キーの押下;\nimpl M不変データ for キーの押下 {}\nimpl M解釈前の入力 for キーの押下 {}\nimpl M生入力 for キーの押下 {}\n",
        ),
    ];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M解釈前の入力 `キーの押下` は `M生入力` と `M中間入力` のどちらも名乗っていない"));
}
