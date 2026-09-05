//! 保存したマテリアル台帳の層割当が、明示の書き出しから実行時の地表層テクスチャ集まで届く境界を確かめる。
//! 台帳を保存していないプロジェクトが既定の割当で焼けることと、解決できない割当を拒むことも合わせて見る。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blitz_engine::surface_layer_textures::{地表層テクスチャ集, 実行時形式から地表層テクスチャ集を読む};
use editor_server::{プロジェクト保管庫, マテリアル台帳, マテリアル定義, 層割当};
use tower::ServiceExt;

/// 4層の材質定義。識別色は台帳の検証を通すためだけに置く。
fn 材質定義一覧(追加の材質名: &[&str]) -> Vec<マテリアル定義> {
    ["grass_a", "dirt_a", "rock_a", "sand_a"]
        .iter()
        .chain(追加の材質名)
        .map(|材質名| マテリアル定義 {
            エンジン材質名: (*材質名).to_string(),
            識別色: "#2d5a27".to_string(),
        })
        .collect()
}

fn 層割当を作る(草: &str, 泥: &str, 岩: &str, 砂: &str) -> 層割当 {
    層割当 {
        草: 草.to_string(),
        泥: 泥.to_string(),
        岩: 岩.to_string(),
        砂: 砂.to_string(),
    }
}

fn 一時プロジェクトを整える(識別子: &str) -> (crate::common::一時プロジェクト, editor_server::ファイル保管庫) {
    let 一時 = crate::common::一時プロジェクト::生成する(識別子);
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = crate::common::小さな区画割り();
    crate::common::大域世界を保存する(&保管庫, 区画割り);
    crate::common::零のマザーを保存する(&保管庫, 区画割り);
    crate::common::フォックスのソースを配置する(&一時);
    crate::common::地表層のタイルを配置する(&一時);
    (一時, 保管庫)
}

async fn 書き出しの応答の状態を得る(一時: &crate::common::一時プロジェクト) -> StatusCode {
    crate::common::ルーターを作る(一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

async fn 書き出して地表層テクスチャ集を読む(一時: &crate::common::一時プロジェクト) -> 地表層テクスチャ集 {
    assert_eq!(書き出しの応答の状態を得る(一時).await, StatusCode::OK);
    let パス = 一時.ルート().join("target/editor_world_assets/world_surface_layer_textures.blitzasset");
    実行時形式から地表層テクスチャ集を読む(&std::fs::read(&パス).unwrap()).unwrap()
}

#[tokio::test]
async fn 保存した層割当のとおりに層とタイルが結ばれる() {
    let (一時, 保管庫) = 一時プロジェクトを整える("export_surface_layers");
    // 草と岩を入れ替えた割当を保存する。既定の並びのままでも通る検査にならないようにするためである。
    保管庫
        .マテリアル台帳を検証して保存する(マテリアル台帳 {
            マテリアル一覧: 材質定義一覧(&[]),
            層割当: 層割当を作る("rock_a", "dirt_a", "grass_a", "sand_a"),
        })
        .unwrap();

    let テクスチャ集 = 書き出して地表層テクスチャ集を読む(&一時).await;
    assert_eq!(テクスチャ集.層のタイル(0).材質名().綴り(), "rock_a");
    assert_eq!(テクスチャ集.層のタイル(1).材質名().綴り(), "dirt_a");
    assert_eq!(テクスチャ集.層のタイル(2).材質名().綴り(), "grass_a");
    assert_eq!(テクスチャ集.層のタイル(3).材質名().綴り(), "sand_a");
}

#[tokio::test]
async fn 台帳を保存していないプロジェクトは既定の層割当で焼ける() {
    let (一時, _保管庫) = 一時プロジェクトを整える("export_surface_layers_default");
    let テクスチャ集 = 書き出して地表層テクスチャ集を読む(&一時).await;
    assert_eq!(テクスチャ集.層のタイル(0).材質名().綴り(), "grass_a");
    assert_eq!(テクスチャ集.層のタイル(3).材質名().綴り(), "sand_a");
}

#[tokio::test]
async fn タイルの無い材質名を指す層割当は書き出しを拒む() {
    let (一時, 保管庫) = 一時プロジェクトを整える("export_surface_layers_missing");
    保管庫
        .マテリアル台帳を検証して保存する(マテリアル台帳 {
            マテリアル一覧: 材質定義一覧(&["タイルの無い材質"]),
            層割当: 層割当を作る("タイルの無い材質", "dirt_a", "rock_a", "sand_a"),
        })
        .unwrap();
    let 状態 = 書き出しの応答の状態を得る(&一時).await;
    assert_ne!(状態, StatusCode::OK, "解決できない層割当を無言で焼いてはならない");
}
