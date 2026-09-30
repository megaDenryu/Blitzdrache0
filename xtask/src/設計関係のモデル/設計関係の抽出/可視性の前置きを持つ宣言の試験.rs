//! 可視性の前置き(`pub(in パス)`・`pub(self)`)と同じ行の属性を持つ宣言の回帰試験。
//! 可視性の読み方を規約の検査と1つに寄せる前、抽出器は `pub`・`pub(crate)`・`pub(super)` しか読めず、
//! それ以外の可視性を持つ型の定義とフィールドを、関係も抽出できなかった行も残さずに消していた。その消え方が戻らないことを固定する。

use super::トレイトの宣言の行::{トレイトの宣言の読み取り, トレイトの宣言を読む};
use super::抽出の試験の段取り::{原文から設計関係グラフと抽出の欠けを組む, 抽出できなかった理由の説明一覧, 概念の表記一覧, 設計解釈マーカーの宣言の表記一覧, 関係の表記一覧};

const 旅行者のパス: &str = "crates/blitz_esca/src/旅行者.rs";

#[test]
fn パスを指す可視性の構造体とフィールドからノードと保持する関係が出る() {
    let 原文 = "pub(in crate::旅行者) struct 旅行者の現在地 {\n    東: f32,\n}\n\npub(in crate::旅行者) struct 旅行者 {\n    pub(in crate::旅行者) 現在地: 旅行者の現在地,\n}\n";
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[(旅行者のパス, 原文)]);
    let 概念一覧 = 概念の表記一覧(&結果);
    assert!(概念一覧.contains(&"blitz_esca::旅行者::旅行者".to_string()), "{概念一覧:?}");
    let 表記一覧 = 関係の表記一覧(&結果);
    assert!(表記一覧.contains(&"blitz_esca::旅行者::旅行者 保持する blitz_esca::旅行者::旅行者の現在地".to_string()), "{表記一覧:?}");
    assert!(抽出できなかった理由の説明一覧(&結果).is_empty(), "{:?}", 抽出できなかった理由の説明一覧(&結果));
}

#[test]
fn 自分のモジュールに限る可視性の列挙と同じ行の属性を持つ構造体も読む() {
    let 原文 = "pub struct 歩行方向 {\n    東: f32,\n}\n\npub(self) enum 旅行者の意図 {\n    歩く { 方向: 歩行方向 },\n}\n\n#[derive(Clone)] pub(crate) struct 旅程 {\n    意図: 旅行者の意図,\n}\n";
    let 表記一覧 = 関係の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(&[(旅行者のパス, 原文)]));
    assert!(表記一覧.contains(&"blitz_esca::旅行者::旅行者の意図 保持する blitz_esca::旅行者::歩行方向".to_string()), "{表記一覧:?}");
    assert!(表記一覧.contains(&"blitz_esca::旅行者::旅程 保持する blitz_esca::旅行者::旅行者の意図".to_string()), "{表記一覧:?}");
}

#[test]
fn パスを指す可視性のトレイトの宣言から上位トレイトの宣言の事実が出る() {
    let 表記一覧 = 設計解釈マーカーの宣言の表記一覧(&原文から設計関係グラフと抽出の欠けを組む(
        &[("crates/blitz_esca/src/ontology.rs", "pub(in crate::ontology) trait M旅程: M状態 {}\n")],
    ));
    assert_eq!(表記一覧, vec!["blitz_esca::ontology::M旅程 は blitz_design::設計解釈マーカー::M状態 を上位トレイトに持つ".to_string()]);
}

#[test]
fn 可視性の後ろのunsafeを落としてトレイトの宣言を読む() {
    let トレイトの宣言の読み取り::読めた(宣言) = トレイトの宣言を読む(&["pub(in crate::x) unsafe trait 印: M不変データ {}".to_string()], 0) else {
        panic!("トレイトの宣言として読めなかった");
    };
    assert_eq!(宣言.名前, "印");
    assert_eq!(宣言.上位トレイト一覧, vec!["M不変データ".to_string()]);
}
