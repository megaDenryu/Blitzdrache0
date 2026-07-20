//! ビルド時シェーダーコンパイルの入口。slangcの発見と各エントリのコンパイルを束ねる。
//! シェーダーはimportでモジュール分割されているため、rerun-if-changedはshaders/
//! ディレクトリ内の全.slangファイルへ拡張する(scene.slang単体を見るだけでは
//! pbr.slang等の変更を取りこぼす)。

mod bloom_spirv_compile;
mod cloth_spirv_compile;
mod particle_spirv_compile;
mod shadow_spirv_compile;
mod skinning_spirv_compile;
mod slangc_entry_compile;
mod slangc_locate;
mod spirv_compile;
mod tonemap_spirv_compile;
mod ui_spirv_compile;

use std::env;
use std::path::{Path, PathBuf};

const シェーダーディレクトリ相対パス: &str = "../../shaders";
const エントリファイル名: &str = "scene.slang";
const 粒子エントリファイル名: &str = "particle.slang";
const UIエントリファイル名: &str = "ui.slang";
const シャドウエントリファイル名: &str = "shadow.slang";
const トーンマップエントリファイル名: &str = "tonemap.slang";
const ブルーム縮小側エントリファイル名: &str = "bloom_down.slang";
const ブルーム拡大側エントリファイル名: &str = "bloom_up.slang";
const スキニングエントリファイル名: &str = "skinning.slang";

pub(crate) fn シェーダーをビルドする() -> Result<(), String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|誤り| format!("CARGO_MANIFEST_DIR環境変数が取得できない: {誤り}"))?;
    let シェーダーディレクトリ絶対パス = PathBuf::from(&manifest_dir).join(シェーダーディレクトリ相対パス);
    再ビルド対象を登録する(&シェーダーディレクトリ絶対パス)?;

    let out_dir =
        env::var("OUT_DIR").map_err(|誤り| format!("OUT_DIR環境変数が取得できない: {誤り}"))?;
    let 出力先ディレクトリ = PathBuf::from(out_dir);
    let slangc = slangc_locate::発見する()?;

    let ソース絶対パス = シェーダーディレクトリ絶対パス.join(エントリファイル名);
    spirv_compile::頂点とフラグメントをコンパイルする(&slangc, &ソース絶対パス, &出力先ディレクトリ)?;

    let 粒子ソース絶対パス = シェーダーディレクトリ絶対パス.join(粒子エントリファイル名);
    particle_spirv_compile::三エントリをコンパイルする(&slangc, &粒子ソース絶対パス, &出力先ディレクトリ)?;

    let uiソース絶対パス = シェーダーディレクトリ絶対パス.join(UIエントリファイル名);
    ui_spirv_compile::頂点とフラグメントをコンパイルする(&slangc, &uiソース絶対パス, &出力先ディレクトリ)?;

    let シャドウソース絶対パス = シェーダーディレクトリ絶対パス.join(シャドウエントリファイル名);
    shadow_spirv_compile::頂点とフラグメントをコンパイルする(&slangc, &シャドウソース絶対パス, &出力先ディレクトリ)?;

    let トーンマップソース絶対パス = シェーダーディレクトリ絶対パス.join(トーンマップエントリファイル名);
    tonemap_spirv_compile::頂点とフラグメントをコンパイルする(&slangc, &トーンマップソース絶対パス, &出力先ディレクトリ)?;

    let ブルーム縮小側パス = シェーダーディレクトリ絶対パス.join(ブルーム縮小側エントリファイル名);
    bloom_spirv_compile::縮小側をコンパイルする(&slangc, &ブルーム縮小側パス, &出力先ディレクトリ)?;

    let ブルーム拡大側パス = シェーダーディレクトリ絶対パス.join(ブルーム拡大側エントリファイル名);
    bloom_spirv_compile::拡大側をコンパイルする(&slangc, &ブルーム拡大側パス, &出力先ディレクトリ)?;

    let スキニングパス = シェーダーディレクトリ絶対パス.join(スキニングエントリファイル名);
    skinning_spirv_compile::コンピュートをコンパイルする(&slangc, &スキニングパス, &出力先ディレクトリ)?;

    cloth_spirv_compile::全部をコンパイルする(&slangc, &シェーダーディレクトリ絶対パス, &出力先ディレクトリ)
}

/// shaders/ディレクトリ自体と、直下の全.slangファイルをrerun-if-changed対象にする。
/// ディレクトリ自体も登録することで、ファイルの追加・削除にも追従する
/// (Cargoはディレクトリのmtimeでもこのトリガーを検知できる)。
fn 再ビルド対象を登録する(ディレクトリ: &Path) -> Result<(), String> {
    println!("cargo:rerun-if-changed={}", ディレクトリ.display());
    let 読み取り結果 = std::fs::read_dir(ディレクトリ)
        .map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ =
            エントリ結果.map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
        let パス = エントリ.path();
        if パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("slang") {
            println!("cargo:rerun-if-changed={}", パス.display());
        }
    }
    Ok(())
}
