//! 正解表の読みの試験。担当するのは、実物の綴りをそのまま読めることと、座標系が違う表を拒むことを固定することである。
//!
//! 座標系の検査を固定するのは、Blender側が座標系を変えた表を黙って読むと、突き合わせの不一致が
//! 「接合の計算が誤っている」という別の結論に化けるためである。

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::error::正解表の読み込みエラー;
use super::reference_file::組み立ての正解表のファイル;

fn 一時ファイルへ書く(名前: &str, 中身: &str) -> PathBuf {
    let ディレクトリ = std::env::temp_dir().join("blitzdrache0_assembly_reference");
    std::fs::create_dir_all(&ディレクトリ).unwrap();
    let パス = ディレクトリ.join(名前);
    std::fs::write(&パス, 中身).unwrap();
    パス
}

const 実物の綴り: &str = r#"{
  "建物ID": "Building_TavernInn",
  "座標系": "glTF",
  "部品の姿勢": [
    { "部品ID": "Mod_Tavern_F1_Base", "平行移動": [0.0, 0.0, 0.0], "回転": [0.0, 0.0, 0.0, 1.0], "倍率": [1.0, 1.0, 1.0] },
    { "部品ID": "Mod_Tavern_F2_Jetty", "平行移動": [0.0, 3.0, 0.0], "回転": [0.0, 0.0, 0.0, 1.0], "倍率": [1.0, 1.0, 1.0] }
  ]
}"#;

#[test]
fn 実物の綴りをそのまま読む() {
    let パス = 一時ファイルへ書く("正解表.json", 実物の綴り);
    let 表 = 組み立ての正解表のファイル::生成する(&パス).正解表を読み取る().unwrap();
    assert_eq!(表.組み立ての識別子(), "Building_TavernInn");
    assert_eq!(表.姿勢一覧().len(), 2);
    let 二階 = 表.部品で全件引く("Mod_Tavern_F2_Jetty")[0];
    assert_eq!(二階.平行移動, [0.0, 3.0, 0.0]);
    assert_eq!(二階.回転, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(二階.倍率, [1.0, 1.0, 1.0]);
}

#[test]
fn 受け取らない座標系の正解表を拒む() {
    let パス = 一時ファイルへ書く("別の座標系.json", &実物の綴り.replace("\"glTF\"", "\"Blender\""));
    let 誤り = 組み立ての正解表のファイル::生成する(&パス).正解表を読み取る().unwrap_err();
    assert!(matches!(誤り, 正解表の読み込みエラー::座標系がglTFでない { .. }), "得た誤り: {誤り}");
}

#[test]
fn 成分の数が足りない姿勢を拒む() {
    let パス = 一時ファイルへ書く("成分不足.json", &実物の綴り.replace("[0.0, 3.0, 0.0]", "[0.0, 3.0]"));
    let 誤り = 組み立ての正解表のファイル::生成する(&パス).正解表を読み取る().unwrap_err();
    assert!(matches!(誤り, 正解表の読み込みエラー::値の形が違う { .. }), "得た誤り: {誤り}");
}
