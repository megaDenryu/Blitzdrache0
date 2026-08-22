//! エディターが未保存のチャンクを開いて保存する流れを、書き出しと焼きまで通して確かめる。
//! 担当するのは、配られた初期値をそのまま保存した世界が焼きの縁の一致検査を通ることと、
//! 大域から切り出していない高さを保存した世界がその検査で落ちることの2点である。
//! 配られる値そのものの検査は`chunk_heightmap_mother_cutout.rs`が担当する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use common::チャンク一辺頂点数;

async fn 高さ格子を保存する(一時: &common::一時プロジェクト, バイト列: Vec<u8>) {
    let 応答 = common::ルーターを作る(一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/高さ格子")
                .header("content-type", "application/octet-stream")
                .body(Body::from(バイト列))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);
}

async fn 書き出して焼く(一時: &common::一時プロジェクト) -> (StatusCode, String) {
    let 応答 = common::ルーターを作る(一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 状態 = 応答.status();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    (状態, String::from_utf8_lossy(&本体).to_string())
}

fn 焼ける世界を用意する(識別子: &str) -> common::一時プロジェクト {
    let 一時 = common::一時プロジェクト::生成する(識別子);
    common::大域を一意な値で用意する(&一時);
    common::フォックスのソースを配置する(&一時);
    common::地表層のタイルを配置する(&一時);
    一時
}

#[tokio::test]
async fn 配られた初期値を保存してから書き出すと縁の一致検査を通る() {
    let 一時 = 焼ける世界を用意する("chunk_height_cutout_export");

    // チャンクを開いて保存する流れ。配られた初期値をそのまま保存し、隣の3チャンクは未保存のまま残す。
    let (状態, 本体) = common::高さ格子を取得する(&一時, 0, 0).await;
    assert_eq!(状態, StatusCode::OK);
    高さ格子を保存する(&一時, 本体).await;

    let (状態, 説明) = 書き出して焼く(&一時).await;
    assert_eq!(状態, StatusCode::OK, "縁の一致検査が落ちた: {説明}");
    assert!(一時.ルート().join("target/editor_world_assets/chunk_directory.blitzchunks").is_file());
}

#[tokio::test]
async fn 大域と食い違う高さを保存した書き出しは縁の一致検査で落ちる() {
    let 一時 = 焼ける世界を用意する("chunk_height_cutout_seam_break");

    // 大域から切り出さずに作った高さ(ここでは一様な0メートル)を保存すると、隣接チャンクと縁が食い違う。
    // 座標を種にした初期生成をそのまま保存していたときに起きていた食い違いの再現である。
    高さ格子を保存する(&一時, vec![0u8; チャンク一辺頂点数 * チャンク一辺頂点数 * 4]).await;

    let (状態, 説明) = 書き出して焼く(&一時).await;
    assert_eq!(状態, StatusCode::UNPROCESSABLE_ENTITY, "縁の食い違いが検出されていない");
    assert!(
        説明.contains("重なる帯の格子点") && 説明.contains("高さが食い違う"),
        "落ちた理由が縁の一致検査でない: {説明}"
    );
}
