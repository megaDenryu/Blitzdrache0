//! 屋内の多光源の検収世界の定義と、そのソースアセットの書き出し。焼くのは石積みの小屋を載せる地面だけであり、
//! 小屋そのものは外部のアセットリポジトリから来る(宣言は`crates/blitz_asset_compiler/examples/compile_assets/world/stone_hut_declaration.rs`)。
//!
//! 地面を起伏の無い平面にするのは、小屋の4枚の壁の底辺が1つの水平面に載るためである。起伏があると、
//! 壁の底辺と地面のあいだに隙間が空く辺と、壁が地面へ埋まる辺が同時に生まれ、戸口から差す光が
//! 壁の下の隙間から漏れる。屋内が点光源だけで照らされていることを画素で見る検収であるから、
//! 意図しない漏れ口を1つも作らない。

use std::path::Path;

use blitz_asset_compiler::{高さ格子を切り出す, 高さ格子を格納する, 高さ格子諸元};
use blitz_engine::チャンク座標;

use crate::directory_source::{目録ソースを作る, 目録項目};

/// 小屋1棟は1チャンクの内側へ収まるため、世界は原点チャンク1つだけを持つ。
const チャンクのX: i32 = 0;
const チャンクのZ: i32 = 0;
const 一辺メートル: f32 = 100.0;

/// 地形の世界と同じ細かさにして、段の作られ方と接地の求め方をそのまま流用できるようにする。
const 辺分割数: u16 = 64;

/// 隣接チャンクと重ねる縁の幅。中央差分で法線を求めるのに前後1点あれば足りる。
const 重なり幅: u8 = 1;

/// 屋内の検収シーンを指す安定ID。`prop_`で始めることが、書き換えもピクセル判定も行わない読み戻しだけの
/// 検収計画を選ばせる。参照: `crates/blitz_app/src/smoke/readback_only_scene.rs`
const シーン名: &str = "prop_stone_hut_interior";

const 格子ファイル名: &str = "prop_stone_hut_interior.heightgrid";
const 目録ソースファイル名: &str = "chunk_directory.txt";

pub(crate) fn 書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let 諸元 = 高さ格子諸元::生成する(辺分割数, 重なり幅, 一辺メートル).map_err(|誤り| 誤り.to_string())?;
    let 座標 = チャンク座標::生成する(チャンクのX, チャンクのZ);
    let 格子 = 高さ格子を切り出す(諸元, 座標, 平らな地面の高さを求める).map_err(|誤り| 誤り.to_string())?;
    書き込む(&出力先ディレクトリ.join(格子ファイル名), &高さ格子を格納する(&格子))?;
    let 項目一覧 = vec![目録項目 {
        座標,
        アセット識別子: シーン名.to_string(),
        ソース相対パス: 格子ファイル名.to_string(),
    }];
    書き込む(&出力先ディレクトリ.join(目録ソースファイル名), 目録ソースを作る(&項目一覧).as_bytes())
}

/// どの大域サンプル添字でも高さは0である。
fn 平らな地面の高さを求める(_添字x: i16, _添字z: i16) -> f32 {
    0.0
}

fn 書き込む(パス: &Path, バイト列: &[u8]) -> Result<(), String> {
    std::fs::write(パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
