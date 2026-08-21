//! 保存した建物が明示の書き出しから実行時の部品群まで届く境界を確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blitz_engine::{実行時形式からシーンを読む, 描画形状};
use editor_server::{チャンク座標, チャンク構造, プロジェクト保管庫, 位置3次元, 建物の配置, 散布の設定};
use tower::ServiceExt;

#[tokio::test]
async fn 保存した家屋は中心基準座標を変換して高さ格子上へ部品群として焼かれる() {
    let 一時 = common::一時プロジェクト::生成する("export_building");
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = common::建物を据えられる区画割り();
    common::大域世界を保存する(&保管庫, 区画割り);
    common::マザーを一意な値で保存する(&保管庫, 区画割り);
    common::フォックスのソースを配置する(&一時);
    保管庫
        .チャンクの構造を検証して保存する(
            チャンク座標::生成する(0, 0),
            チャンク構造 {
                道路一覧: Vec::new(),
                建物一覧: vec![建物の配置 {
                    識別子: "stable-house-1".to_string(),
                    建物定義ID: editor_server::建物定義ID::生成する(common::一間四方の家の識別子).unwrap(),
                    位置: 位置3次元 { x: 0.0, y: 999.0, z: 0.0 },
                    向きラジアン: 0.25,
                    基礎半径メートル: 4.0,
                    なじみ半径メートル: 8.0,
                }],
                散布: 散布の設定 {
                    最小間隔メートル: 5.5,
                    乱数の種: 1,
                },
            },
        )
        .unwrap();
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    焼いた建物を確かめる(&一時);
}

fn 焼いた建物を確かめる(一時: &common::一時プロジェクト) {
    let バイト列 = std::fs::read(一時.ルート().join("target/editor_world_assets/editor_terrain_x0_z0.blitzasset")).unwrap();
    let シーン = 実行時形式からシーンを読む(&バイト列).unwrap();
    let 群一覧 = シーン
        .描画対象一覧()
        .iter()
        .filter_map(|対象| match 対象.形状() {
            描画形状::インスタンス群(群) => Some(群),
            描画形状::通常メッシュ(_) | 描画形状::地形LODメッシュ群(_) => None,
        })
        .collect::<Vec<_>>();
    assert!(!群一覧.is_empty());
    assert!(群一覧.iter().map(|群| 群.個体数()).sum::<usize>() > 1);
    assert!(群一覧.iter().flat_map(|群| 群.配置一覧()).all(|配置| 配置.平行移動()[1] < 100.0));
}
