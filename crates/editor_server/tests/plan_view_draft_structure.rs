//! 見下ろし図の下書きを載せたチャンク構造の編集サーバーの統合試験。保存と読込の往復、下書きを持たない旧版の読込、
//! 大升の一辺が格子解像度を割り切らない保存の拒否の3件を確かめる。
//! 参照: `_doc/設計/見下ろし図による地形編集.md`「検収の形」
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use editor_server::{
    チャンク座標, チャンク構造, プロジェクト保管庫, 地表材質層, 大升の塗り, 平面の位置, 散布の設定, 既定の大升の一辺の升目数, 等高線,
    見下ろし図の下書き,
};
use tower::ServiceExt;

fn 下書きを載せた構造(大升の一辺の升目数: u32) -> チャンク構造 {
    チャンク構造 {
        道路一覧: Vec::new(),
        建物一覧: Vec::new(),
        散布: 散布の設定 {
            最小間隔メートル: 5.5,
            乱数の種: 3,
        },
        散布の個体一覧: Vec::new(),
        見下ろし図の下書き: 見下ろし図の下書き {
            等高線一覧: vec![等高線 {
                高さメートル: 12.5,
                頂点列: vec![平面の位置 { x: -4.0, z: 2.0 }, 平面の位置 { x: 6.0, z: -3.0 }],
                閉じている: true,
            }],
            大升の一辺の升目数,
            大升の塗り一覧: vec![大升の塗り {
                列: 1,
                行: 2,
                高さメートル: Some(8.0),
                層: Some(地表材質層::岩),
            }],
        },
    }
}

#[test]
fn 下書きを持つチャンク構造を保存して読み戻すと一致する() {
    let (_一時, 保管庫) = common::保管庫を作る("plan_view_draft_roundtrip");
    let 座標 = チャンク座標::生成する(1, -2);
    let 構造 = 下書きを載せた構造(既定の大升の一辺の升目数);
    保管庫.チャンクの構造を検証して保存する(座標, 構造.clone()).unwrap();
    assert_eq!(保管庫.チャンクの構造を読む(座標).unwrap(), Some(構造));
}

#[test]
fn 下書きを持たない旧版は空の下書きとして読める() {
    let (一時, 保管庫) = common::保管庫を作る("plan_view_draft_legacy");
    let 座標 = チャンク座標::生成する(0, 0);
    let 構造パス = 一時.ルート().join("editor_data/チャンク/0_0/構造.json");
    std::fs::create_dir_all(構造パス.parent().unwrap()).unwrap();
    let 旧版 = serde_json::json!({
        "道路一覧": [],
        "建物一覧": [],
        "散布": { "最小間隔メートル": 5.5, "乱数の種": 9 },
        "散布の個体一覧": [{ "安定識別子": "散布-x0z0-0", "チャンク中心からの東メートル": 1.0, "チャンク中心からの南メートル": 2.0 }]
    });
    std::fs::write(&構造パス, serde_json::to_vec_pretty(&旧版).unwrap()).unwrap();

    let 読み込み結果 = 保管庫.チャンクの構造を読む(座標).unwrap().unwrap();
    assert_eq!(読み込み結果.散布の個体一覧.len(), 1);
    assert_eq!(
        読み込み結果.見下ろし図の下書き,
        見下ろし図の下書き::空の下書きを作る(既定の大升の一辺の升目数)
    );
}

#[tokio::test]
async fn 大升の一辺が格子解像度を割り切らない構造の保存は拒まれる() {
    let 一時 = common::一時プロジェクト::生成する("plan_view_draft_indivisible");
    common::区画割りを保存する(&一時).await;
    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&下書きを載せた構造(7)).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let 応答 = common::ルーターを作る(&一時)
        .oneshot(
            Request::put("/api/チャンク/0/0/構造")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&下書きを載せた構造(16)).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::NO_CONTENT);
}
