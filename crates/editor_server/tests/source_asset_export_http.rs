//! `POST /api/書き出し/ソースアセット`のREST経路のふるまいを確かめる。担当するのは、
//! 大域未保存の422と、正常な書き出しの応答本体・書き出したチャンク目録がblitz_asset_compilerの
//! 読み手で読めることの2点である。幾何(1px重複共有・縁クランプ・編集済み優先)は
//! `source_asset_export_geometry.rs`が担当する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blitz_asset_compiler::{チャンク目録ソースを読み込む, 高さ格子を読み込む};
use tower::ServiceExt;

#[tokio::test]
async fn 大域世界未保存での書き出しは422を返す() {
    let 一時 = common::一時プロジェクト::生成する("export_no_world");
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn 正常な書き出しはファイル数と出力先を返しチャンク目録を読み戻せる() {
    let 一時 = common::一時プロジェクト::生成する("export_ok");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::小さな区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本体: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(本体["書いたファイル数"], 5); // チャンク4 + 目録1

    let 目録パス = 一時.ルート().join("assets/editor_world/chunk_directory.txt");
    let 項目一覧 = チャンク目録ソースを読み込む(&目録パス).unwrap();
    assert_eq!(項目一覧.len(), 4);
    for 項目 in &項目一覧 {
        let ソースパス = 目録パス.parent().unwrap().join(項目.ソース相対パス());
        let 格子 = 高さ格子を読み込む(&ソースパス).unwrap();
        assert_eq!(格子.諸元().格子点数(), 3);
    }
}
