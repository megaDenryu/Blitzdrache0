//! 楽曲のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

fn 楽曲のjson() -> serde_json::Value {
    serde_json::to_value(common::楽曲の例()).unwrap()
}

async fn 本文を読む(応答: axum::response::Response) -> serde_json::Value {
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&本体).unwrap()
}

async fn 保存を要求する(一時: &common::一時プロジェクト, 経路の名乗り: &str, 本文: &serde_json::Value) -> StatusCode {
    common::ルーターを作る(一時)
        .oneshot(
            Request::put(format!("/api/楽曲/{経路の名乗り}"))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(本文).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
}

async fn 取得を要求する(一時: &common::一時プロジェクト, 経路: &str) -> axum::response::Response {
    common::ルーターを作る(一時)
        .oneshot(Request::get(経路).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn 未保存の楽曲はnullを返す() {
    let 一時 = common::一時プロジェクト::生成する("music_get_null");
    let 応答 = 取得を要求する(&一時, "/api/楽曲/試験の楽曲").await;
    assert_eq!(応答.status(), StatusCode::OK);
    assert!(本文を読む(応答).await.is_null());
}

#[tokio::test]
async fn 保存した楽曲を取得でき一覧にも並ぶ() {
    let 一時 = common::一時プロジェクト::生成する("music_put_get");
    assert_eq!(保存を要求する(&一時, "試験の楽曲", &楽曲のjson()).await, StatusCode::NO_CONTENT);

    let 応答 = 取得を要求する(&一時, "/api/楽曲/試験の楽曲").await;
    assert_eq!(応答.status(), StatusCode::OK);
    assert_eq!(本文を読む(応答).await, 楽曲のjson());

    let 一覧の応答 = 取得を要求する(&一時, "/api/楽曲一覧").await;
    assert_eq!(本文を読む(一覧の応答).await, serde_json::json!(["試験の楽曲"]));
}

#[tokio::test]
async fn 経路と本文の名乗りが食い違うと422を返し正本を作らない() {
    let 一時 = common::一時プロジェクト::生成する("music_put_mismatch");
    assert_eq!(保存を要求する(&一時, "別の名乗り", &楽曲のjson()).await, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(本文を読む(取得を要求する(&一時, "/api/楽曲/別の名乗り").await).await.is_null());
}

#[tokio::test]
async fn 検証に落ちる楽曲は422を返し種別と説明を持つ() {
    let 一時 = common::一時プロジェクト::生成する("music_put_invalid");
    let mut 不正な楽曲 = 楽曲のjson();
    不正な楽曲["拍毎分"] = serde_json::json!(10);
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/楽曲/試験の楽曲")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&不正な楽曲).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let 本文 = 本文を読む(応答).await;
    assert_eq!(本文["種別"], serde_json::json!("構造検証エラー"));
    assert!(本文["説明"].as_str().unwrap().contains("拍毎分"));
}
