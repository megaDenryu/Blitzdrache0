//! 建物エディターで作った格子由来の建物が、配置から書き出しを経て実行時の部品群まで焼かれることを確かめる。
//! 段Gの最小の縦切り(保存→カタログ→配置→焼き)がひと続きで通ることの機械の検収である。
//!
//! 部品の実体を読むため、外部アセットの置き場が無い環境では検査を飛ばす。

#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blitz_engine::{実行時形式からシーンを読む, 描画形状};
use editor_server::{チャンク座標, プロジェクト保管庫};
use serde::Deserialize;
use tower::ServiceExt;

#[derive(Deserialize)]
struct 読み取るチャンクソース {
    形式版: u32,
    建物の格子一覧: Vec<serde_json::Value>,
    建物配置一覧: Vec<serde_json::Value>,
}

#[tokio::test]
async fn 格子から作った建物は配置されて実行時の部品群まで焼かれる() {
    if !crate::common::外部アセットの置き場があるか() {
        println!("外部アセットの置き場が無いため、格子由来の建物の焼きの検査をスキップする");
        return;
    }
    let 一時 = crate::common::一時プロジェクト::生成する("grid_building_bake");
    let ルーター = crate::common::ルーターを作る(&一時);
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = crate::common::建物を据えられる区画割り();
    crate::common::大域世界を保存する(&保管庫, 区画割り);
    crate::common::零のマザーを保存する(&保管庫, 区画割り);
    crate::common::フォックスのソースを配置する(&一時);
    crate::common::地表層のタイルを配置する(&一時);

    let 保存 = ルーター
        .clone()
        .oneshot(
            Request::put(format!("/api/建物/{}/格子", crate::common::試験の建物定義の識別子))
                .header("content-type", "application/json")
                .body(Body::from(crate::common::初期の格子のjson().to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(保存.status(), StatusCode::NO_CONTENT);

    保管庫
        .チャンクの構造を検証して保存する(チャンク座標::生成する(0, 0), crate::common::格子の建物を1件置いた構造())
        .unwrap();
    let 書き出し = ルーター
        .clone()
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(書き出し.status(), StatusCode::OK);

    書き出したチャンクソースを確かめる(&一時);
    焼いた部品群を確かめる(&一時);
}

/// 書き出したマニフェストが形式版4で、そのチャンクが使う格子を1件だけ抱えていることを見る。
fn 書き出したチャンクソースを確かめる(一時: &crate::common::一時プロジェクト) {
    let パス = 一時.ルート().join("assets/editor_world/editor_chunk_x0_z0.json");
    let ソース: 読み取るチャンクソース = serde_json::from_slice(&std::fs::read(&パス).unwrap()).unwrap();
    assert_eq!(ソース.形式版, 4);
    assert_eq!(ソース.建物配置一覧.len(), 1);
    assert_eq!(ソース.建物の格子一覧.len(), 1, "チャンクが使う格子を抱えていない");
}

fn 焼いた部品群を確かめる(一時: &crate::common::一時プロジェクト) {
    let バイト列 = std::fs::read(一時.ルート().join("target/editor_world_assets/editor_terrain_x0_z0.blitzasset")).unwrap();
    let シーン = 実行時形式からシーンを読む(&バイト列).unwrap();
    let 個体の総数: usize = シーン
        .描画対象一覧()
        .iter()
        .filter_map(|対象| match 対象.形状() {
            描画形状::インスタンス群(群) => Some(群.個体数()),
            描画形状::通常メッシュ(_) | 描画形状::地形LODメッシュ群(_) => None,
        })
        .sum();
    assert!(個体の総数 >= 7, "格子の1升目が据える部品の数に届かない: {個体の総数}");
}
