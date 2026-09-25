//! 抽出の規則2(設計解釈マーカーの実装)の回帰試験。「このRustの構文からこの分類の事実が出て、設計関係は出ない」を固定する。

use super::test_support::{分類の事実の表記一覧, 原文から設計関係グラフと抽出の欠けを組む, 関係の表記一覧};

#[test]
fn マーカーの実装から実装の事実が出て設計関係は出ない() {
    let 原文 = "pub struct 旅行者の現在地 {\n    東: f32,\n}\n\nimpl M状態 for 旅行者の現在地 {}\n";
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/traveler.rs", 原文)]);
    let 表記一覧 = 分類の事実の表記一覧(&結果);
    assert!(表記一覧.contains(&"blitz_esca::traveler::旅行者の現在地 は blitz_design::marker::M状態 を実装する".to_string()), "{表記一覧:?}");
    assert!(!関係の表記一覧(&結果).iter().any(|表記| 表記.contains("M状態")), "マーカーの実装から設計関係が出ている: {:?}", 関係の表記一覧(&結果));
}

#[test]
fn 修飾を書いたマーカーの実装からも同じ節点への事実が出る() {
    let 原文 = "pub struct 移動の変位 {\n    東: f32,\n}\n\nimpl blitz_design::M不変データ for 移動の変位 {}\n";
    let 表記一覧 = 分類の事実の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/traveler.rs", 原文)]));
    assert!(表記一覧.contains(&"blitz_esca::traveler::移動の変位 は blitz_design::marker::M不変データ を実装する".to_string()), "{表記一覧:?}");
}

#[test]
fn 標準resultへの包括の実装は標準の型の節点として事実が出る() {
    let 原文 = "impl<T, E> M結果 for std::result::Result<T, E> {}\n";
    let 表記一覧 = 分類の事実の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_design/src/marker.rs", 原文)]));
    assert_eq!(表記一覧, vec!["std::result::Result は blitz_design::marker::M結果 を実装する".to_string()]);
}
