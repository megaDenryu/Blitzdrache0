//! `大域世界/高さ格子`のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn 区画割り未保存での高さ格子putは400を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_heightmap_no_layout");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/大域世界/高さ格子")
                .header("content-type", "application/octet-stream")
                .body(Body::from(vec![0u8; 4]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn 高さ格子は区画割りの寸法どおりなら保存して取得できる() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_heightmap_roundtrip");
    crate::common::区画割りを保存する(&一時).await;

    let 頂点数: usize = 4 * 128 + 1;
    let バイト列 = vec![9u8; 頂点数 * 頂点数 * 4];

    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/大域世界/高さ格子")
                .header("content-type", "application/octet-stream")
                .body(Body::from(バイト列.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/大域世界/高さ格子").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    assert_eq!(本体.to_vec(), バイト列);
}

#[tokio::test]
async fn 高さ格子が未保存なら204を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("world_heightmap_get_empty");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/大域世界/高さ格子").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);
}
