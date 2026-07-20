//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。
//! 型定義は`types`、値を伴う引数の個別処理は`value_args`に委ねる。

mod types;
mod value_args;

use std::path::PathBuf;

pub(crate) use types::{起動モード, 起動設定};

use crate::error::起動エラー;

const 既定シェーダー監視パス: &str = "shaders/scene.slang";
const 既定シーン名: &str = "quad";
const 既定アセットルート: &str = "assets";

/// `--frames N` `--shader-source <path>` `--scene <id>` `--asset-root <dir>`
/// `--unlit` `--particles` `--report-gpu-times` `--dev-ui` を解析する。
/// `--shader-source`は監視・再コンパイル対象のエントリファイルを指す。`import`先の
/// 他ファイルは常にエントリと同じディレクトリから解決するため、このオプションでの
/// 指定は不要。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut モード = 起動モード::無期限実行;
    let mut シェーダー監視パス = PathBuf::from(既定シェーダー監視パス);
    let mut シーン名 = 既定シーン名.to_string();
    let mut アセットルート = PathBuf::from(既定アセットルート);
    let mut ライティング有効 = true;
    let mut 粒子有効 = false;
    let mut gpu時間報告 = false;
    let mut 開発ui初期有効 = false;

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        match 引数値.as_str() {
            "--frames" => {
                モード = value_args::frames引数を処理する(&mut 引数)?;
            }
            "--shader-source" => {
                シェーダー監視パス = value_args::shader_source引数を処理する(&mut 引数)?;
            }
            "--scene" => {
                シーン名 = value_args::scene引数を処理する(&mut 引数)?;
            }
            "--asset-root" => {
                アセットルート = value_args::asset_root引数を処理する(&mut 引数)?;
            }
            "--unlit" => {
                ライティング有効 = false;
            }
            "--particles" => {
                粒子有効 = true;
            }
            "--report-gpu-times" => {
                gpu時間報告 = true;
            }
            "--dev-ui" => {
                開発ui初期有効 = true;
            }
            _ => {}
        }
    }

    Ok(起動設定 {
        モード,
        シェーダー監視パス,
        シーン名,
        アセットルート,
        ライティング有効,
        粒子有効,
        gpu時間報告,
        開発ui初期有効,
    })
}
