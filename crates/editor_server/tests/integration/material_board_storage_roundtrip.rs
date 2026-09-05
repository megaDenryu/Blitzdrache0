//! `ファイル保管庫`のマテリアル台帳の保存往復テスト。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use editor_server::{プロジェクト保管庫, マテリアル台帳, マテリアル定義, 層割当};

fn 台帳例() -> マテリアル台帳 {
    マテリアル台帳 {
        マテリアル一覧: vec![
            マテリアル定義 {
                エンジン材質名: "grass_a".to_string(),
                識別色: "#2d5a27".to_string(),
            },
            マテリアル定義 {
                エンジン材質名: "dirt_a".to_string(),
                識別色: "#5c4033".to_string(),
            },
            マテリアル定義 {
                エンジン材質名: "rock_a".to_string(),
                識別色: "#64748b".to_string(),
            },
            マテリアル定義 {
                エンジン材質名: "sand_a".to_string(),
                識別色: "#d4b483".to_string(),
            },
        ],
        層割当: 層割当 {
            草: "grass_a".to_string(),
            泥: "dirt_a".to_string(),
            岩: "rock_a".to_string(),
            砂: "sand_a".to_string(),
        },
    }
}

#[test]
fn マテリアル台帳は保存前は無しを返す() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("material_board_none");
    assert!(保管庫.マテリアル台帳を読む().unwrap().is_none());
}

#[test]
fn マテリアル台帳を保存して読み戻すと一致する() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("material_board_roundtrip");
    保管庫.マテリアル台帳を検証して保存する(台帳例()).unwrap();
    assert_eq!(保管庫.マテリアル台帳を読む().unwrap(), Some(台帳例()));
}

#[test]
fn 層割当が一覧に無い材質名を参照していると保存を拒む() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("material_board_invalid_reference");
    let mut 不正な台帳 = 台帳例();
    不正な台帳.層割当.草 = "存在しない材質".to_string();
    assert!(保管庫.マテリアル台帳を検証して保存する(不正な台帳).is_err());
    assert!(保管庫.マテリアル台帳を読む().unwrap().is_none());
}

#[test]
fn 材質名が重複していると保存を拒む() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("material_board_duplicate_name");
    let mut 不正な台帳 = 台帳例();
    不正な台帳.マテリアル一覧.push(マテリアル定義 {
        エンジン材質名: "grass_a".to_string(),
        識別色: "#000000".to_string(),
    });
    assert!(保管庫.マテリアル台帳を検証して保存する(不正な台帳).is_err());
    assert!(保管庫.マテリアル台帳を読む().unwrap().is_none());
}
