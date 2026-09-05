//! 最新の形へ移行できない旧版の正本が残っているチャンクへの上書きを、保管庫が拒むことを確かめる。
//! 読めないまま上書きすると、旧版だけが持っていた建物(塔・宝箱)の位置が復元不能に消える。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

fn 塔を持つ旧版のチャンク構造Json() -> serde_json::Value {
    serde_json::json!({
        "道路一覧": [],
        "建物一覧": [{
            "識別子": "old-tower-1",
            "種別": "塔",
            "位置": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "向きラジアン": 0.5,
            "基礎半径メートル": 4.0,
            "なじみ半径メートル": 8.0
        }],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 42 }
    })
}

fn 建物を持たない最新のチャンク構造Json() -> serde_json::Value {
    serde_json::json!({
        "道路一覧": [],
        "建物一覧": [],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 7 },
        "散布の個体一覧": [],
        "見下ろし図の下書き": { "等高線一覧": [], "粗マスの一辺の升目数": 8, "粗マスの塗り一覧": [] }
    })
}

#[tokio::test]
async fn 移行できない旧版が残るチャンクへの保存は拒まれ正本は残る() {
    let 一時 = crate::common::一時プロジェクト::生成する("legacy_overwrite_guard");
    let 構造パス = 一時.ルート().join("editor_data/チャンク/0_0/構造.json");
    std::fs::create_dir_all(構造パス.parent().unwrap()).unwrap();
    let 旧版のバイト列 = serde_json::to_vec_pretty(&塔を持つ旧版のチャンク構造Json()).unwrap();
    std::fs::write(&構造パス, &旧版のバイト列).unwrap();

    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&建物を持たない最新のチャンク構造Json()).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::CONFLICT);
    assert_eq!(
        std::fs::read(&構造パス).unwrap(),
        旧版のバイト列,
        "拒んだ保存が旧版の正本を書き換えている"
    );
}
