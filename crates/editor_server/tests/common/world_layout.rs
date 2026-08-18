//! 複数のテストファイルが共有する、大域世界構造(区画割り)の固定値と保存ヘルパー。
#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use super::一時プロジェクト;

pub fn 区画割りJson() -> serde_json::Value {
    serde_json::json!({
        "区画割り": { "一辺のメートル": 1024.0, "軸あたりチャンク数": 4, "チャンクあたり格子解像度": 128 },
        "広域道路": { "制御点列": [], "全幅メートル": 12.0, "細分割数": 120 }
    })
}

pub async fn 区画割りを保存する(一時: &一時プロジェクト) {
    let 応答 = super::ルーターを作る(一時)
        .oneshot(
            Request::put("/api/大域世界/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&区画割りJson()).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);
}
