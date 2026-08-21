//! 建物外形カタログのHTTP境界が、状態へ渡した版付きJSONをそのまま公開することを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn 建物外形カタログは形式版と定義一覧を返す() {
    let 一時 = common::一時プロジェクト::生成する("building_outline_catalog_get");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/建物外形カタログ").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), axum::http::StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(本文["形式版"], editor_server::建物外形カタログの現在の形式版);
    assert_eq!(本文["建物定義一覧"], serde_json::json!([]));
}
