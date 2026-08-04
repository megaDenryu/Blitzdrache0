//! 目視見本の世界の定義と、そのソースアセット一式の書き出し。地形の世界とも見本の集落とも別の目録を持つ独立した世界である。
//! 世界の広がり(1チャンク)・一辺長・格子の細かさという世界そのものの決め事をこのモジュールが持ち、高さの合成関数と
//! 材質見本の立体の形はそれぞれの子モジュールが持つ。
//! 小物の原型は本体リポジトリの外にあるアセットリポジトリから来るため、ここが書き出すのは地面と材質見本の立体と遠景地面だけである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Ic-3bの実装」

mod distant_ground;
mod geometry;
mod gltf_json;
mod height;
mod pedestal_mesh;
mod sample_bodies;
mod solid_grid;
mod sphere_mesh;

use std::path::Path;

use blitz_asset_compiler::{高さ格子を切り出す, 高さ格子を格納する, 高さ格子諸元};
use blitz_engine::チャンク座標;

use crate::directory_source::{目録ソースを作る, 目録項目};

/// 庭は1チャンクの内側へ収まるため、世界は原点チャンク1つだけを持つ。広げないのは、この世界の目的が
/// 間接照明の絵を目で確かめることであり、チャンクの出入りは地形の世界と植生の世界がすでに検収しているためである。
const チャンクのX: i32 = 0;
const チャンクのZ: i32 = 0;
const 一辺メートル: f32 = 100.0;

/// 地形の世界と同じ細かさにして、段の作られ方と接地の求め方をそのまま流用できるようにする。
const 辺分割数: u16 = 64;

/// 隣接チャンクと重ねる縁の幅。中央差分で法線を求めるのに前後1点あれば足りる。
const 重なり幅: u8 = 1;

/// 目視見本のシーンを指す安定ID。`terrain`で始めることが、この世界の空方針と間接照明方針を天空の遠方環境にする。
/// 参照: `crates/blitz_app/src/app/time_of_day/scene_policy.rs`
const シーン名: &str = "terrain_visual";

const 格子ファイル名: &str = "terrain_visual.heightgrid";
const 目録ソースファイル名: &str = "chunk_directory.txt";
const 立体のバイナリファイル名: &str = "material_sample_bodies.bin";
const 立体の文書ファイル名: &str = "material_sample_bodies.gltf";
const 遠景地面のバイナリファイル名: &str = "distant_ground.bin";
const 遠景地面の文書ファイル名: &str = "distant_ground.gltf";

pub(crate) fn 書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    地面を書き出す(出力先ディレクトリ)?;
    材質見本の立体を書き出す(出力先ディレクトリ)?;
    遠景地面を書き出す(出力先ディレクトリ)
}

fn 地面を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let 諸元 = 高さ格子諸元::生成する(辺分割数, 重なり幅, 一辺メートル).map_err(|誤り| 誤り.to_string())?;
    let 座標 = チャンク座標::生成する(チャンクのX, チャンクのZ);
    let 格子 = 高さ格子を切り出す(諸元, 座標, height::大域高さを求める).map_err(|誤り| 誤り.to_string())?;
    書き込む(&出力先ディレクトリ.join(格子ファイル名), &高さ格子を格納する(&格子))?;
    let 項目一覧 = vec![目録項目 {
        座標,
        アセット識別子: シーン名.to_string(),
        ソース相対パス: 格子ファイル名.to_string(),
    }];
    書き込む(&出力先ディレクトリ.join(目録ソースファイル名), 目録ソースを作る(&項目一覧).as_bytes())
}

fn 材質見本の立体を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let 連結 = geometry::組み立てる();
    書き込む(&出力先ディレクトリ.join(立体のバイナリファイル名), &連結.バッファバイト列を作る())?;
    書き込む(
        &出力先ディレクトリ.join(立体の文書ファイル名),
        gltf_json::文書(&連結, 立体のバイナリファイル名).as_bytes(),
    )
}

fn 遠景地面を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(
        &出力先ディレクトリ.join(遠景地面のバイナリファイル名),
        &distant_ground::バッファバイト列を作る(),
    )?;
    書き込む(
        &出力先ディレクトリ.join(遠景地面の文書ファイル名),
        distant_ground::文書(遠景地面のバイナリファイル名).as_bytes(),
    )
}

fn 書き込む(パス: &Path, バイト列: &[u8]) -> Result<(), String> {
    std::fs::write(パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
