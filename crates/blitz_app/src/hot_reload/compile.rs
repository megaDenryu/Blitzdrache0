//! ホットリロード時の再コンパイル。一時ディレクトリへ共有頂点段1本と、契約と材質変種の組ごとの画素段4本を書き出して読み戻す。
//!
//! 5本すべてが成功したときにだけ束を返すのは、一部の組だけが新しい混成の束を作らないためである。
//! 途中で失敗すると呼び出し元は差し替えを行わず、旧いパイプラインがそのまま描き続ける。
//! どの組がどのソースから来るかは`fragment_contract`が答える。

mod fragment_contract;

use std::path::Path;
use std::process::Command;

use blitz_render::indirect_lighting::{契約と材質変種ごとの画素段, 契約別のシーン描画シェーダー};

use super::compile_error::シェーダー再コンパイルエラー;
use super::slangc::{self, スランガー位置};
use fragment_contract::画素段の組;

pub(super) fn シーン描画シェーダーをコンパイルする(
    監視先: &Path,
) -> Result<契約別のシーン描画シェーダー, シェーダー再コンパイルエラー> {
    let slangc = slangc::発見する()?;
    let 頂点の説明 = format!("共有頂点段({})", 監視先.display());
    let 頂点spirv = エントリを1つコンパイルする(&slangc, 監視先, "vertexMain", "vertex", "vertex.spv", &頂点の説明)?;
    let mut 組ごとのバイト列 = Vec::with_capacity(画素段の組::全ての組.len());
    for 組 in 画素段の組::全ての組 {
        組ごとのバイト列.push(画素段をコンパイルする(&slangc, 組, 監視先)?);
    }
    let [
        定数近似の標準金属粗さpbr,
        定数近似の地表の層の重ね合わせ,
        遠方環境の標準金属粗さpbr,
        遠方環境の地表の層の重ね合わせ,
    ] = 組ごとのバイト列
        .try_into()
        .unwrap_or_else(|_| panic!("画素段の組の数と焼いたバイト列の本数が食い違った"));
    Ok(契約別のシーン描画シェーダー::生成する(
        頂点spirv,
        契約と材質変種ごとの画素段 {
            定数近似の標準金属粗さpbr,
            定数近似の地表の層の重ね合わせ,
            遠方環境の標準金属粗さpbr,
            遠方環境の地表の層の重ね合わせ,
        },
    )?)
}

/// 組の名前とソースパスを失敗へ載せるのは、4本の画素段の入口名が`fragmentMain`で同じであり、
/// slangcの出す文面だけではどの組が落ちたのか読み手に決められないためである。
fn 画素段をコンパイルする(
    slangc: &スランガー位置,
    組: 画素段の組,
    監視先: &Path,
) -> Result<Vec<u8>, シェーダー再コンパイルエラー> {
    let ソースパス = 組.ソースパスを解決する(監視先)?;
    let 説明 = format!("{}の画素段({})", 組.名前(), ソースパス.display());
    エントリを1つコンパイルする(slangc, &ソースパス, "fragmentMain", "fragment", 組.出力ファイル名(), &説明)
}

fn エントリを1つコンパイルする(
    slangc: &スランガー位置,
    ソースパス: &Path,
    エントリ名: &str,
    ステージ: &str,
    出力ファイル名: &str,
    段の説明: &str,
) -> Result<Vec<u8>, シェーダー再コンパイルエラー> {
    let 出力パス = std::env::temp_dir().join("blitzdrache0_hot_reload").join(出力ファイル名);
    if let Some(親) = 出力パス.parent() {
        std::fs::create_dir_all(親).map_err(|誤り| シェーダー再コンパイルエラー::一時ディレクトリを作れない { 誤り })?;
    }

    let 実行結果 = Command::new(slangc.プログラム名())
        .arg(ソースパス)
        .args(["-entry", エントリ名])
        .args(["-stage", ステージ])
        .args(["-target", "spirv"])
        .arg("-fvk-use-entrypoint-name")
        .arg("-o")
        .arg(&出力パス)
        .output()
        .map_err(|誤り| シェーダー再コンパイルエラー::スランガーを起動できない { 誤り })?;

    if !実行結果.status.success() {
        return Err(シェーダー再コンパイルエラー::スランガーが失敗した {
            段の説明: 段の説明.to_string(),
            標準エラー出力: String::from_utf8_lossy(&実行結果.stderr).into_owned(),
        });
    }

    std::fs::read(&出力パス).map_err(|誤り| シェーダー再コンパイルエラー::コンパイル結果を読めない { 誤り })
}
