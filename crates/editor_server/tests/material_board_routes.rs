//! `マテリアル台帳`のREST経路のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

fn 台帳Json() -> serde_json::Value {
    serde_json::json!({
        "マテリアル一覧": [
            { "エンジン材質名": "grass_a", "識別色": "#2d5a27" },
            { "エンジン材質名": "dirt_a", "識別色": "#5c4033" },
            { "エンジン材質名": "rock_a", "識別色": "#64748b" },
            { "エンジン材質名": "sand_a", "識別色": "#d4b483" }
        ],
        "層割当": { "草": "grass_a", "泥": "dirt_a", "岩": "rock_a", "砂": "sand_a" }
    })
}

#[tokio::test]
async fn マテリアル台帳の初期状態はnullを返す() {
    let 一時 = common::一時プロジェクト::生成する("material_board_get_null");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/マテリアル台帳").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}

#[tokio::test]
async fn マテリアル台帳を保存して取得できる() {
    let 一時 = common::一時プロジェクト::生成する("material_board_put_get");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/マテリアル台帳")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&台帳Json()).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/マテリアル台帳").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 取得データ: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(取得データ, 台帳Json());
}

#[tokio::test]
async fn 参照先が存在しない層割当は422を返し正本を変えない() {
    let 一時 = common::一時プロジェクト::生成する("material_board_reject");
    let mut 不正データ = 台帳Json();
    不正データ["層割当"]["草"] = serde_json::json!("存在しない材質");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/マテリアル台帳")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&不正データ).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/マテリアル台帳").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert!(本文.is_null());
}
