//! 道路が1本だけだった頃に保存されたJSONを、保管庫が最新の道路一覧の形へ変換して読めることを確かめる。
//! 保存は最新の形だけで行うため、往復ではなく「旧版を書いて最新で読む」向きだけを固定する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use editor_server::{チャンク座標, プロジェクト保管庫};

fn 旧版のチャンク構造Json() -> serde_json::Value {
    serde_json::json!({
        "道路": { "制御点列": [{ "x": 1.0, "y": 2.0, "z": 3.0 }], "全幅メートル": 8.0, "散布除外バッファメートル": 14.0, "細分割数": 80 },
        "建物一覧": [],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 42 }
    })
}

fn 旧版の大域世界構造Json() -> serde_json::Value {
    serde_json::json!({
        "区画割り": { "一辺のメートル": 1024.0, "軸あたりチャンク数": 4, "チャンクあたり格子解像度": 128 },
        "広域道路": { "制御点列": [{ "x": 4.0, "y": 5.0, "z": 6.0 }], "全幅メートル": 12.0, "細分割数": 120 }
    })
}

#[test]
fn 道路が1本だけの旧版のチャンク構造は道路一覧1本として読める() {
    let (一時, 保管庫) = common::保管庫を作る("legacy_chunk_structure");
    let 座標 = チャンク座標::生成する(0, 0);
    let 構造パス = 一時.ルート().join("editor_data/チャンク/0_0/構造.json");
    std::fs::create_dir_all(構造パス.parent().unwrap()).unwrap();
    std::fs::write(&構造パス, serde_json::to_vec_pretty(&旧版のチャンク構造Json()).unwrap()).unwrap();

    let 読み込み結果 = 保管庫.チャンクの構造を読む(座標).unwrap().unwrap();
    assert_eq!(読み込み結果.道路一覧.len(), 1);
    assert_eq!(読み込み結果.道路一覧[0].制御点列.len(), 1);
    assert_eq!(読み込み結果.道路一覧[0].全幅メートル, 8.0);
    assert_eq!(読み込み結果.散布.乱数の種, 42);
}

#[test]
fn 広域道路が1本だけの旧版の大域世界構造は広域道路一覧1本として読める() {
    let (一時, 保管庫) = common::保管庫を作る("legacy_world_structure");
    let 構造パス = 一時.ルート().join("editor_data/大域世界/構造.json");
    std::fs::create_dir_all(構造パス.parent().unwrap()).unwrap();
    std::fs::write(&構造パス, serde_json::to_vec_pretty(&旧版の大域世界構造Json()).unwrap()).unwrap();

    let 読み込み結果 = 保管庫.大域世界の構造を読む().unwrap().unwrap();
    assert_eq!(読み込み結果.広域道路一覧.len(), 1);
    assert_eq!(読み込み結果.広域道路一覧[0].制御点列.len(), 1);
    assert_eq!(読み込み結果.広域道路一覧[0].全幅メートル, 12.0);
    assert_eq!(読み込み結果.区画割り.軸あたりチャンク数, 4);
}
