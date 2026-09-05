//! `POST /api/書き出し/ソースアセット`の失敗をHTTP状態へ分類する境界を確かめる。
//! 保存不足とソース依存の不正は422、実行時出力先の入出力失敗は500を担当する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn 大域世界未保存での書き出しは422を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("export_no_world");
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn 書き出したソースの依存が欠ける場合は422を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("export_invalid_source");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = crate::common::小さな区画割り();
    crate::common::大域世界を保存する(&保管庫, 区画割り);
    crate::common::マザーを一意な値で保存する(&保管庫, 区画割り);
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn 実行時出力先を作れない場合は500を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("export_io_failure");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = crate::common::小さな区画割り();
    crate::common::大域世界を保存する(&保管庫, 区画割り);
    crate::common::マザーを一意な値で保存する(&保管庫, 区画割り);
    crate::common::フォックスのソースを配置する(&一時);
    crate::common::地表層のタイルを配置する(&一時);
    let 出力先 = 一時.ルート().join("target/editor_world_assets");
    std::fs::create_dir_all(出力先.parent().unwrap()).unwrap();
    std::fs::write(出力先, b"directory creation must fail").unwrap();
    let 応答 = crate::common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
