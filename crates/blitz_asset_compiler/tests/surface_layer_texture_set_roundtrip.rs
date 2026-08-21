//! 地表層のタイル画像→地表層テクスチャ集アセット→実行時読込の往復テスト。
//! `cargo xtask gen-source-assets`で生成済みのassets/surface_layer_textures/の4枚のタイルを読む。
//!
//! 注意: `cargo test`のテストバイナリはパッケージディレクトリを作業ディレクトリとして実行されるため、
//! `CARGO_MANIFEST_DIR`からの相対パスでリポジトリルート直下のassets/を参照する。

#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use std::path::PathBuf;

use blitz_asset_compiler::{
    ソースルート, テクスチャ格納方針, 地表層タイルの置き場, 地表層テクスチャ集アセットをコンパイルする, 地表材質の層割当
};
use blitz_engine::surface_layer_textures::実行時形式から地表層テクスチャ集を読む;

/// 生成器が書き出す4層の純色。`examples/generate_source_assets/surface_layer_tiles.rs`の写しであり、
/// 焼いた結果の画素がこの色のままであることを確かめるためにここへ持つ。
const 層ごとの純色: [[u8; 4]; 4] = [[60, 140, 60, 255], [130, 90, 50, 255], [140, 140, 140, 255], [200, 180, 110, 255]];

fn タイルの置き場を作る() -> 地表層タイルの置き場 {
    let ソースルート = ソースルート::生成する(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets"));
    地表層タイルの置き場::ソースルートの下に作る(&ソースルート)
}

fn 焼く(層割当: &地表材質の層割当) -> Result<Vec<u8>, String> {
    地表層テクスチャ集アセットをコンパイルする(&タイルの置き場を作る(), 層割当, テクスチャ格納方針::全てRGBA8)
        .map(|結果| 結果.実行時バイト列)
        .map_err(|誤り| 誤り.to_string())
}

#[test]
fn 既定の層割当で焼いた集は実行時形式から層ごとの純色として読み戻せる() {
    let バイト列 = 焼く(&地表材質の層割当::既定を作る()).unwrap();
    let テクスチャ集 = 実行時形式から地表層テクスチャ集を読む(&バイト列).unwrap();
    assert_eq!(テクスチャ集.材質ごとのタイル().len(), 4);
    for (層番号, 期待する色) in 層ごとの純色.iter().enumerate() {
        let タイル = テクスチャ集.層のタイル(層番号);
        let 画素列 = タイル.ベースカラー().rgba8の原寸の画素列を返す().unwrap();
        assert_eq!(&画素列[0..4], 期待する色, "層{層番号}の先頭の画素が期待の純色でない");
    }
}

#[test]
fn 同じ層割当で2度焼いたバイト列は一致する() {
    let 一度目 = 焼く(&地表材質の層割当::既定を作る()).unwrap();
    let 二度目 = 焼く(&地表材質の層割当::既定を作る()).unwrap();
    assert_eq!(一度目, 二度目);
}

#[test]
fn 同じ材質名を2つの層が指しても焼けて索引が一致する() {
    let 層割当 = 地表材質の層割当::生成する(["grass_a", "grass_a", "rock_a", "sand_a"].map(str::to_string)).unwrap();
    let テクスチャ集 = 実行時形式から地表層テクスチャ集を読む(&焼く(&層割当).unwrap()).unwrap();
    // タイルは重複を除いて3枚になり、層0と層1が同じ索引を指す。
    assert_eq!(テクスチャ集.材質ごとのタイル().len(), 3);
    let 索引 = テクスチャ集.層ごとの材質索引().層ごとの索引();
    assert_eq!(索引[0], 索引[1]);
}

#[test]
fn 置き場にタイルが無い材質名を指す層割当は型付きエラーになる() {
    let 層割当 = 地表材質の層割当::生成する(["存在しない材質", "dirt_a", "rock_a", "sand_a"].map(str::to_string)).unwrap();
    let 誤り = 焼く(&層割当).unwrap_err();
    assert!(誤り.contains("存在しない材質"), "失敗の文言が材質名を名指していない: {誤り}");
}

#[test]
fn 空の材質名を持つ層割当は生成の時点で拒む() {
    assert!(地表材質の層割当::生成する(["  ", "dirt_a", "rock_a", "sand_a"].map(str::to_string)).is_err());
}
