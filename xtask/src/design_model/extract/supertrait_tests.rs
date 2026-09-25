//! 抽出の規則1(上位トレイトの宣言)の回帰試験。「このRustの構文からこの上位トレイトの宣言の事実が出て、設計関係は出ない」を固定する。

use super::test_support::{分類の事実の表記一覧, 原文から設計関係グラフと抽出の欠けを組む, 抽出できなかった理由の説明一覧, 関係の表記一覧};

const 正本のパス: &str = "crates/blitz_design/src/marker.rs";

#[test]
fn 正本のトレイトの宣言の上位トレイトから上位トレイトの宣言の事実が出る() {
    let 原文 = "pub trait M不変データ: Clone {}\npub trait M状態: M不変データ {}\npub trait M値オブジェクト: M不変データ + PartialEq {}\n";
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[(正本のパス, 原文)]);
    let 表記一覧 = 分類の事実の表記一覧(&結果);
    for 期待 in [
        "blitz_design::marker::M不変データ は Clone を上位トレイトに持つ",
        "blitz_design::marker::M状態 は blitz_design::marker::M不変データ を上位トレイトに持つ",
        "blitz_design::marker::M値オブジェクト は blitz_design::marker::M不変データ を上位トレイトに持つ",
        "blitz_design::marker::M値オブジェクト は PartialEq を上位トレイトに持つ",
    ] {
        assert!(表記一覧.contains(&期待.to_string()), "{期待} が無い: {表記一覧:?}");
    }
    assert!(関係の表記一覧(&結果).is_empty(), "上位トレイトの宣言から設計関係が出ている: {:?}", 関係の表記一覧(&結果));
}

#[test]
fn 上位トレイトを持たないマーカーの宣言からは事実が出ない() {
    let 表記一覧 = 分類の事実の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[(正本のパス, "pub trait M結果 {}\n")]));
    assert!(表記一覧.is_empty(), "{表記一覧:?}");
}

#[test]
fn 正本の外でマーカーを継ぐトレイトの宣言からも事実が出る() {
    let 原文 = "pub trait M旅程: M状態 {}\n";
    let 表記一覧 = 分類の事実の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/ontology.rs", 原文)]));
    assert_eq!(表記一覧, vec!["blitz_esca::ontology::M旅程 は blitz_design::marker::M状態 を上位トレイトに持つ".to_string()]);
}

#[test]
fn 正本の外でマーカーを継がないトレイトの宣言からは事実が出ない() {
    let 表記一覧 = 分類の事実の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/ontology.rs", "pub trait 表示器: Clone {}\n")]));
    assert!(表記一覧.is_empty(), "{表記一覧:?}");
}

#[test]
fn where節を持つトレイトの宣言は欠落として数える() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[(正本のパス, "pub trait M旅程: M状態 where Self: Sized {}\n")]);
    assert!(分類の事実の表記一覧(&結果).is_empty(), "{:?}", 分類の事実の表記一覧(&結果));
    let 欠落 = 結果.関係を落とした抽出の欠落へ写す();
    assert_eq!(欠落.合計の件数(), 1, "{}", 欠落.説明());
    assert!(抽出できなかった理由の説明一覧(&結果).iter().any(|説明| 説明.contains("`where` 節")), "{:?}", 抽出できなかった理由の説明一覧(&結果));
}

#[test]
fn 宣言が1行に収まらないトレイトは欠落として数える() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[(正本のパス, "pub trait M旅程:\n    M状態\n{\n}\n")]);
    assert!(分類の事実の表記一覧(&結果).is_empty(), "{:?}", 分類の事実の表記一覧(&結果));
    let 欠落 = 結果.関係を落とした抽出の欠落へ写す();
    assert_eq!(欠落.合計の件数(), 1, "{}", 欠落.説明());
    assert!(抽出できなかった理由の説明一覧(&結果).iter().any(|説明| 説明.contains("1行に収まっておらず")), "{:?}", 抽出できなかった理由の説明一覧(&結果));
}
