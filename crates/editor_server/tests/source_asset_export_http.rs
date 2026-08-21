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
use blitz_asset_compiler::{チャンク目録ソースを読み込む, フォックスのソース, 高さ格子を読み込む};
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
async fn 書き出したソースの依存が欠ける場合は422を返す() {
    let 一時 = common::一時プロジェクト::生成する("export_invalid_source");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::小さな区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn 実行時出力先を作れない場合は500を返す() {
    let 一時 = common::一時プロジェクト::生成する("export_io_failure");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::小さな区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);
    フォックスのソースを配置する(&一時);
    let 出力先 = 一時.ルート().join("target/editor_world_assets");
    std::fs::create_dir_all(出力先.parent().unwrap()).unwrap();
    std::fs::write(出力先, b"directory creation must fail").unwrap();
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn 正常な書き出しはファイル数と出力先を返しチャンク目録を読み戻せる() {
    let 一時 = common::一時プロジェクト::生成する("export_ok");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::小さな区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);
    フォックスのソースを配置する(&一時);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    let 本体: serde_json::Value = serde_json::from_slice(&本体).unwrap();
    assert_eq!(本体["書いたファイル数"], 5); // チャンク4 + 目録1

    let 目録パス = 一時.ルート().join("assets/editor_world/chunk_directory.txt");
    let 目録ソース = チャンク目録ソースを読み込む(&目録パス).unwrap();
    assert_eq!(目録ソース.一辺().f32値(), 4.0);
    let 項目一覧 = 目録ソース.項目一覧();
    assert_eq!(項目一覧.len(), 4);
    for 項目 in &項目一覧 {
        let ソースパス = 目録パス.parent().unwrap().join(項目.ソース相対パス());
        let 格子 = 高さ格子を読み込む(&ソースパス).unwrap();
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
    フォックスのソースを配置する(&一時);
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);

    let 書き出した目録 = std::fs::read(一時.ルート().join("assets/editor_world/chunk_directory.txt")).unwrap();
    let 既存目録 = std::fs::read(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/editor_world/chunk_directory.txt")).unwrap();
    assert_eq!(書き出した目録, 既存目録);
}

fn フォックスのソースを配置する(一時: &common::一時プロジェクト) {
    let 相対 = std::path::Path::new(フォックスのソース);
    let 元 = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets").join(相対);
    let 先 = 一時.ルート().join("assets").join(相対);
    let 親 = 先.parent().unwrap_or_else(|| {
        panic!(
            "フォックスの配置先に親が無い。`cargo xtask fetch-assets`で素材を取得し直す: {}",
            先.display()
        )
    });
    std::fs::create_dir_all(親).unwrap_or_else(|誤り| panic!("フォックスの配置先を作れない。`cargo xtask fetch-assets`で素材を取得し直す: {誤り}"));
    std::fs::copy(元, 先).unwrap_or_else(|誤り| panic!("フォックスのソースを複製できない。`cargo xtask fetch-assets`で素材を取得し直す: {誤り}"));
}
