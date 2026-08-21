//! 散布の個体一覧を持たなかった頃に保存されたチャンク構造を、保管庫が個体0件の最新の形へ変換して読めることを確かめる。
//! 保存は最新の形だけで行うため、往復ではなく「旧版を書いて最新で読む」向きだけを固定する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use editor_server::{チャンク座標, プロジェクト保管庫};

fn 散布の個体一覧を持たない旧版のチャンク構造Json() -> serde_json::Value {
    serde_json::json!({
        "道路一覧": [{ "制御点列": [], "全幅メートル": 8.0, "散布除外バッファメートル": 14.0, "細分割数": 80 }],
        "建物一覧": [{
            "識別子": "house-1",
            "建物定義ID": "frame_house_one_bay",
            "位置": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "向きラジアン": 0.5,
            "基礎半径メートル": 4.0,
            "なじみ半径メートル": 8.0
        }],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 42 }
    })
}

#[test]
fn 散布の個体一覧を持たない旧版は個体0件として読める() {
    let (一時, 保管庫) = common::保管庫を作る("legacy_structure_without_scatter");
    let 座標 = チャンク座標::生成する(0, 0);
    let 構造パス = 一時.ルート().join("editor_data/チャンク/0_0/構造.json");
    std::fs::create_dir_all(構造パス.parent().unwrap()).unwrap();
    std::fs::write(
        &構造パス,
        serde_json::to_vec_pretty(&散布の個体一覧を持たない旧版のチャンク構造Json()).unwrap(),
    )
    .unwrap();

    let 読み込み結果 = 保管庫.チャンクの構造を読む(座標).unwrap().unwrap();
    assert!(読み込み結果.散布の個体一覧.is_empty());
    assert_eq!(読み込み結果.散布.乱数の種, 42);
    assert_eq!(読み込み結果.道路一覧.len(), 1);
    assert_eq!(読み込み結果.建物一覧[0].建物定義ID.綴り(), common::一間四方の家の識別子);
}
