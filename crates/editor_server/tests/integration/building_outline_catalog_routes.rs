//! 建物外形カタログのHTTP境界が、状態へ渡した版付きJSONをそのまま公開することを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn 建物外形カタログは形式版と定義一覧を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("building_outline_catalog_get");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::get("/api/建物外形カタログ").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), axum::http::StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本文: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(本文["形式版"], editor_server::建物外形カタログの現在の形式版);
    let 期待するカタログ = crate::common::建物外形カタログを作る();
    let 期待する識別子一覧 = 期待するカタログ
        .建物定義一覧
        .iter()
        .map(|定義| 定義.識別子.綴り().to_string())
        .collect::<Vec<_>>();
    let 配った識別子一覧 = 本文["建物定義一覧"]
        .as_array()
        .unwrap()
        .iter()
        .map(|定義| 定義["識別子"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(配った識別子一覧, 期待する識別子一覧);
    assert!(本文["建物定義一覧"][0]["外接箱"]["最大"].is_array());
    assert!(
        本文["建物定義一覧"][0]["高さメートル"].is_null(),
        "外接箱から導ける高さを二重に持っている"
    );
}
