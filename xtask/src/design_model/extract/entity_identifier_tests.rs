//! 抽出の規則4(エンティティの識別子の関連型)の回帰試験。向きが識別子から型へであることを固定する。

use super::test_support::{原文から結末を組む, 抽出できなかった理由の説明一覧, 関係の表記一覧};

const 旅行者のパス: &str = "crates/blitz_esca/src/traveler.rs";

#[test]
fn 識別子の関連型から識別子を主語にした識別する関係が出る() {
    let 原文 = "pub struct 旅行者ID {\n    値: u64,\n}\n\npub struct 旅行者 {\n    識別子: 旅行者ID,\n}\n\nimpl Mエンティティ for 旅行者 {\n    type 識別子 = 旅行者ID;\n\n    fn 識別子(&self) -> &旅行者ID {\n        &self.識別子\n    }\n}\n";
    let 表記一覧 = 関係の表記一覧(&原文から結末を組む(&[(旅行者のパス, 原文)]));
    assert!(表記一覧.contains(&"blitz_esca::traveler::旅行者ID 識別する blitz_esca::traveler::旅行者".to_string()), "{表記一覧:?}");
    assert!(!表記一覧.contains(&"blitz_esca::traveler::旅行者 識別する blitz_esca::traveler::旅行者ID".to_string()), "{表記一覧:?}");
}

#[test]
fn 識別子の関連型が無い実装は抽出できなかった行として数える() {
    let 原文 = "pub struct 旅行者 {\n    識別子: u64,\n}\n\nimpl Mエンティティ for 旅行者 {\n}\n";
    let 結末 = 原文から結末を組む(&[(旅行者のパス, 原文)]);
    assert!(!関係の表記一覧(&結末).iter().any(|表記| 表記.contains("識別する")), "{:?}", 関係の表記一覧(&結末));
    assert!(抽出できなかった理由の説明一覧(&結末).iter().any(|説明| 説明.contains("`type 識別子 =` の行が無い")), "{:?}", 抽出できなかった理由の説明一覧(&結末));
}
