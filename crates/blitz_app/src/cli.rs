//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。
//! 型定義は`types`、値を伴う引数の個別処理は`value_args`に委ねる。

#[cfg(test)]
mod cli_tests;

mod types;
mod value_args;
pub(crate) use types::{布モード, 粒子表示モード, 起動モード, 起動設定};

use crate::error::起動エラー;

/// 起動設定を表すCLI引数を解析する。粒子系の検証対象は`--particles`または`--surface-flow`で選ぶ。
/// `--shader-source`は監視・再コンパイル対象のエントリファイルを指す。`import`先の他ファイルは常にエントリと同じディレクトリから解決するため個別指定は不要。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut 起動設定 = 起動設定::既定値();

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        match 引数値.as_str() {
            "--frames" => 起動設定.モード = value_args::frames引数を処理する(&mut 引数)?,
            "--shader-source" => {
                起動設定.シェーダー監視パス = value_args::shader_source引数を処理する(&mut 引数)?;
            }
            "--scene" => {
                起動設定.シーン名 = value_args::scene引数を処理する(&mut 引数)?;
            }
            "--asset-root" => {
                起動設定.アセットルート = value_args::asset_root引数を処理する(&mut 引数)?;
            }
            "--unlit" => {
                起動設定.ライティング有効 = false;
            }
            "--particles" => {
                起動設定.粒子表示 = 粒子表示モード::粒子トイ;
            }
            "--surface-flow" => {
                起動設定.粒子表示 = 粒子表示モード::表面流;
            }
            "--sph-512" => 起動設定.粒子表示 = 粒子表示モード::Sph512,
            "--sph-1024" => 起動設定.粒子表示 = 粒子表示モード::Sph1024,
            "--sph-2048" => 起動設定.粒子表示 = 粒子表示モード::Sph2048,
            "--report-gpu-times" => {
                起動設定.gpu時間報告 = true;
            }
            "--report-frame-times" => {
                起動設定.フレーム時間報告 = true;
            }
            "--dev-ui" => {
                起動設定.開発ui初期有効 = true;
            }
            "--dump-frame" => {
                起動設定.フレームダンプ先 = Some(value_args::dump_frame引数を処理する(&mut 引数)?);
            }
            "--no-post" => {
                起動設定.ポスト処理有効 = false;
            }
            "--cloth" => {
                起動設定.布モード = 布モード::吊るし布;
            }
            "--cloth-cape" => {
                起動設定.布モード = 布モード::マント;
            }
            "--exposure" => {
                起動設定.露出 = value_args::exposure引数を処理する(&mut 引数)?;
            }
            "--blend" => {
                起動設定.ブレンド = value_args::blend引数を処理する(&mut 引数)?;
            }
            _ => {}
        }
    }

    Ok(起動設定)
}
