//! `チャンク/{x}/{z}/構造`のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn チャンク構造の初期状態はnullを返す() {
    let 一時 = common::一時プロジェクト::生成する("chunk_structure_get_null");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/0/0/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}

#[tokio::test]
async fn 負のチャンク座標でも構造を保存して取得できる() {
    let 一時 = common::一時プロジェクト::生成する("chunk_structure_negative_coord");
    let 送信データ = serde_json::json!({
        "道路一覧": [{ "制御点列": [], "全幅メートル": 8.0, "散布除外バッファメートル": 14.0, "細分割数": 80 }],
        "建物一覧": [],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 7 },
        "散布の個体一覧": [{ "安定識別子": "散布-x-2z3-0", "チャンク中心からの東メートル": 3.5, "チャンク中心からの南メートル": -7.25 }]
    });
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/-2/3/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&送信データ).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/-2/3/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 取得データ: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(取得データ, 送信データ);

    let 他チャンク応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/2/3/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(他チャンク応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}

#[tokio::test]
async fn 配列でないJSONの構造保存は422を返す() {
    let 一時 = common::一時プロジェクト::生成する("chunk_structure_bad_shape");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&serde_json::json!({ "invalid": true })).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::BAD_REQUEST);
}
