//! `POST /api/書き出し/ソースアセット`が成果物一式を公開する境界を確かめる。
//! 正常応答・チャンク目録の読み戻し・同じ出力先への再公開を担当する。
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
async fn 正常な書き出しはファイル数と出力先を返しチャンク目録を読み戻せる() {
    let 一時 = common::一時プロジェクト::生成する("export_ok");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::小さな区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);
    common::フォックスのソースを配置する(&一時);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本体: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(本体["書いたファイル数"], 9); // 高さ格子4 + 版付きチャンクソース4 + 目録1

    let 目録パス = 一時.ルート().join("assets/editor_world/chunk_directory.txt");
    let 目録ソース = チャンク目録ソースを読み込む(&目録パス).unwrap();
    assert_eq!(目録ソース.一辺().f32値(), 4.0);
    let 項目一覧 = 目録ソース.項目一覧();
    assert_eq!(項目一覧.len(), 4);
    for 項目 in &項目一覧 {
        let ソースパス = 目録パス.parent().unwrap().join(項目.ソース相対パス());
        let ソース: serde_json::Value = serde_json::from_slice(&std::fs::read(&ソースパス).unwrap()).unwrap();
        assert_eq!(ソース["形式版"], 1);
        assert_eq!(ソース["建物配置一覧"], serde_json::json!([]));
        let 高さ格子パス = ソースパス.parent().unwrap().join(ソース["高さ格子"].as_str().unwrap());
        let 格子 = 高さ格子を読み込む(&高さ格子パス).unwrap();
        assert_eq!(格子.諸元().格子点数(), 3);
    }
    assert!(一時.ルート().join("target/editor_world_assets/catalog.blitzcatalog").is_file());
    assert!(一時.ルート().join("target/editor_world_assets/chunk_directory.blitzchunks").is_file());
    assert!(一時.ルート().join("target/editor_world_assets/generation_ledger.txt").is_file());
    let 再書き出し = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(再書き出し.status(), StatusCode::OK);
}

#[tokio::test]
async fn 次回書き出す目録は既存のエディター世界の更新済み目録と一致する() {
    let 一時 = common::一時プロジェクト::生成する("export_editor_directory");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::エディターの区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::零のマザーを保存する(&保管庫, 区画割り);
    common::フォックスのソースを配置する(&一時);
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);

    let 目録パス = 一時.ルート().join("assets/editor_world/chunk_directory.txt");
    let 目録 = チャンク目録ソースを読み込む(&目録パス).unwrap();
    let 項目一覧 = 目録.項目一覧();
    assert_eq!(項目一覧.len(), 16);
    for 項目 in 項目一覧 {
        assert!(項目.ソース相対パス().to_string_lossy().ends_with(".json"));
        assert!(目録パス.parent().unwrap().join(項目.ソース相対パス()).is_file());
    }

    let 書き出した目録 = std::fs::read(&目録パス).unwrap();
    let 既存目録 = std::fs::read(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/editor_world/chunk_directory.txt")).unwrap();
    assert_eq!(書き出した目録, 既存目録, "次回の書き出しがリポジトリの目録と食い違う");
}
