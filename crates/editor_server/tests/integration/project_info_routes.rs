//! `GET /api/プロジェクト情報`のふるまいを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn プロジェクト情報はルートのフォルダ名を名前として返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("project_info_get");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/プロジェクト情報").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    let 期待名 = 一時.ルート().file_name().unwrap().to_string_lossy().into_owned();
    assert_eq!(本文["プロジェクト名"], 期待名);
    assert_eq!(本文["ルートパス"], 一時.ルート().to_string_lossy().into_owned());
}
