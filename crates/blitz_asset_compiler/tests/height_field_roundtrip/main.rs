//! 高さ格子→高さ場アセット→実行時読込→問い合わせの往復テストの共通部。`cargo xtask gen-source-assets`で
//! 生成済みのassets/terrain_world/の25チャンクを読む段取りと、チャンク座標から大域の添字への変換を持つ。
//! 検査本体は、角の一致が`corner_match_tests`、決定性と広がりの外が`determinism_tests`にある。
//!
//! 注意: `cargo test`のテストバイナリはパッケージディレクトリを作業ディレクトリとして実行されるため、
//! `CARGO_MANIFEST_DIR`からの相対パスでリポジトリルート直下のassets/を参照する。

mod corner_match_tests;
mod determinism_tests;

use std::path::PathBuf;

use blitz_asset_compiler::チャンク目録ソースを読み込む;
use blitz_engine::チャンク座標;

/// 地形の世界は東西南北とも-2から2までの25チャンクである。
const 端のチャンク: i32 = 2;
/// 高さ格子の切り出しに使っている1チャンクの一辺。
const チャンクの一辺メートル: f64 = 100.0;

fn リポジトリルートからのパス(相対パス: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(相対パス)
}

fn 地形の世界のソース一覧() -> Vec<(チャンク座標, PathBuf)> {
    let 目録ソースパス = リポジトリルートからのパス("assets/terrain_world/chunk_directory.txt");
    let 項目一覧 = match チャンク目録ソースを読み込む(&目録ソースパス) {
        Ok(一覧) => 一覧,
        Err(誤り) => panic!("チャンク目録ソースの読込に失敗した(cargo xtask gen-source-assetsで生成済みか確認): {誤り}"),
    };
    項目一覧
        .iter()
        .map(|項目| (項目.チャンク(), 目録ソースパス.with_file_name(項目.ソース相対パス())))
        .collect()
}

/// チャンク座標の1成分を、端のチャンクを原点とする大域の添字へ写す。負になったら世界の範囲と端の値が食い違っている。
fn 端からの大域添字へ変換する(成分: i32) -> u32 {
    u32::try_from(成分 + 端のチャンク)
        .unwrap_or_else(|_| panic!("チャンク座標{成分}が端のチャンク{端のチャンク}を足しても負である。世界の範囲と端の値の食い違い"))
}
