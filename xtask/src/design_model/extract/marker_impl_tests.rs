//! 抽出の規則2(設計解釈マーカーの実装)の回帰試験。「このRustの構文からこの関係が出る」を固定する。

use super::test_support::{原文から結果を組む, 関係の表記一覧};

#[test]
fn マーカーの実装から実装した型の下位型である関係が出る() {
    let 原文 = "pub struct 旅行者の現在地 {\n    東: f32,\n}\n\nimpl M状態 for 旅行者の現在地 {}\n";
    let 表記一覧 = 関係の表記一覧(&原文から結果を組む(&[("crates/blitz_esca/src/traveler.rs", 原文)]));
    assert!(表記一覧.contains(&"blitz_esca::traveler::旅行者の現在地 下位型である blitz_design::marker::M状態".to_string()), "{表記一覧:?}");
}

#[test]
fn 修飾を書いたマーカーの実装からも同じ節点へ関係が出る() {
    let 原文 = "pub struct 移動の変位 {\n    東: f32,\n}\n\nimpl blitz_design::M不変データ for 移動の変位 {}\n";
    let 表記一覧 = 関係の表記一覧(&原文から結果を組む(&[("crates/blitz_esca/src/traveler.rs", 原文)]));
    assert!(表記一覧.contains(&"blitz_esca::traveler::移動の変位 下位型である blitz_design::marker::M不変データ".to_string()), "{表記一覧:?}");
}

#[test]
fn 標準resultへの包括の実装は標準の型の節点として関係が出る() {
    let 原文 = "impl<T, E> M結果 for std::result::Result<T, E> {}\n";
    let 表記一覧 = 関係の表記一覧(&原文から結果を組む(&[("crates/blitz_design/src/marker.rs", 原文)]));
    assert_eq!(表記一覧, vec!["std::result::Result 下位型である blitz_design::marker::M結果".to_string()]);
}
