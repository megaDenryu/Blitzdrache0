//! `大域世界/構造`のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn 大域世界構造の初期状態はnullを返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_structure_get_null");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/大域世界/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}

#[tokio::test]
async fn 大域世界構造を保存して取得できる() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_structure_put_get");
    crate::common::区画割りを保存する(&一時).await;

    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/大域世界/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 取得データ: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(取得データ, crate::common::区画割りJson());
}

#[tokio::test]
async fn 一辺のメートルが0の区画割りは422を返し正本を変えない() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_structure_reject");
    let 不正データ = serde_json::json!({
        "区画割り": { "一辺のメートル": 0.0, "軸あたりチャンク数": 4, "チャンクあたり格子解像度": 128 },
        "広域道路一覧": [{ "制御点列": [], "全幅メートル": 12.0, "細分割数": 120 }]
    });
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/大域世界/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&不正データ).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/大域世界/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}
