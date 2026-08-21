//! `チャンク/{x}/{z}/構造`のPUTが、建物定義IDをカタログと突き合わせる境界のふるまいを確かめる。
//! 実在する識別子を通すことと、実在しない識別子を拒んで正本を書き換えないことの2点を担当する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn カタログに実在する建物定義IDを持つ構造は保存できる() {
    let 一時 = common::一時プロジェクト::生成する("chunk_structure_known_building");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(建物1件を持つ構造の保存要求(common::一間四方の家の識別子))
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/0/0/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 取得データ: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(取得データ["建物一覧"][0]["建物定義ID"], common::一間四方の家の識別子);
}

#[tokio::test]
async fn カタログに無い建物定義IDを持つ構造の保存は422を返す() {
    let 一時 = common::一時プロジェクト::生成する("chunk_structure_unknown_building");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(建物1件を持つ構造の保存要求(common::カタログに無い識別子))
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/チャンク/0/0/構造").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null(), "拒んだ保存が正本を書き換えている");
}

fn 建物1件を持つ構造の保存要求(建物定義ID: &str) -> Request<Body> {
    let 送信データ = serde_json::json!({
        "道路一覧": [],
        "建物一覧": [{
            "識別子": "poi_house_0001",
            "建物定義ID": 建物定義ID,
            "位置": { "x": 1.0, "y": 0.0, "z": 2.0 },
            "向きラジアン": 0.5,
            "基礎半径メートル": 7.0,
            "なじみ半径メートル": 12.0
        }],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 7 }
    });
    Request::put("/api/チャンク/0/0/構造")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&送信データ).unwrap()))
        .unwrap()
}
