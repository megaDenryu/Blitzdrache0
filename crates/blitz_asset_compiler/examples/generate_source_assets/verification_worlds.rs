//! 検証用の世界一式の書き出し。担当するのは、どの世界をどのディレクトリへどの順で書き出すかだけである。
//! 世界1つの中身はそれぞれの世界のモジュールが持つ。
//!
//! 場所巡りの世界をここへ入れないのは、あちらが乱数の種を要り、種を変えるたびにここの判定値を持つ入口の入力まで
//! 書き直すことになるためである。`--game-map-seed`を渡した実行はこの一式を1つも書き出さない。

use std::path::Path;

use crate::{
    chunk_world, smoke_assets, terrain_visual_world, terrain_world, texture_compression_world, vegetation_world, village_world, ディレクトリを作る,
};

pub(crate) fn 一式を書き出す() -> Result<(), String> {
    let スモーク出力先 = Path::new("assets/smoke");
    ディレクトリを作る(スモーク出力先)?;
    smoke_assets::書き出す(スモーク出力先)?;
    println!("[generate_source_assets] {}へ生成完了", スモーク出力先.display());

    let チャンク世界出力先 = Path::new("assets/chunk_world");
    ディレクトリを作る(チャンク世界出力先)?;
    chunk_world::書き出す(チャンク世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", チャンク世界出力先.display());

    let 地形世界出力先 = Path::new("assets/terrain_world");
    ディレクトリを作る(地形世界出力先)?;
    terrain_world::書き出す(地形世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 地形世界出力先.display());

    let 植生世界出力先 = Path::new("assets/vegetation_world");
    ディレクトリを作る(植生世界出力先)?;
    vegetation_world::書き出す(植生世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 植生世界出力先.display());

    let 見本の集落出力先 = Path::new("assets/village_world");
    ディレクトリを作る(見本の集落出力先)?;
    village_world::書き出す(見本の集落出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 見本の集落出力先.display());

    let 目視見本出力先 = Path::new("assets/terrain_visual_world");
    ディレクトリを作る(目視見本出力先)?;
    terrain_visual_world::書き出す(目視見本出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 目視見本出力先.display());

    let ブロック圧縮の対照出力先 = Path::new("assets/texture_compression_world");
    ディレクトリを作る(ブロック圧縮の対照出力先)?;
    texture_compression_world::対照素材のソース一式を書き出す(ブロック圧縮の対照出力先)?;
    println!("[generate_source_assets] {}へ生成完了", ブロック圧縮の対照出力先.display());
    Ok(())
}
