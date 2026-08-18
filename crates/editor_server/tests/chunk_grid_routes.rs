//! `チャンク/{x}/{z}/高さ格子`・`材質重み`のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn 高さ格子と材質重みは頂点数どおりなら保存して取得できる() {
    let 一時 = common::一時プロジェクト::生成する("chunk_grids_roundtrip");
    common::区画割りを保存する(&一時).await;

    let 頂点数: usize = 128 + 1;
    let 高さバイト列 = vec![5u8; 頂点数 * 頂点数 * 4];
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/高さ格子")
                .header("content-type", "application/octet-stream")
                .body(Body::from(高さバイト列.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/0/0/高さ格子").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    assert_eq!(本体.to_vec(), 高さバイト列);

    let 重みバイト列 = [255u8, 0, 0, 0].repeat(頂点数 * 頂点数);
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/材質重み")
                .header("content-type", "application/octet-stream")
                .body(Body::from(重みバイト列.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/0/0/材質重み").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    assert_eq!(本体.to_vec(), 重みバイト列);
}

#[tokio::test]
async fn 寸法が違う高さ格子は422を返す() {
    let 一時 = common::一時プロジェクト::生成する("chunk_heightmap_wrong_dim");
    common::区画割りを保存する(&一時).await;
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/高さ格子")
                .header("content-type", "application/octet-stream")
                .body(Body::from(vec![0u8; 10]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
