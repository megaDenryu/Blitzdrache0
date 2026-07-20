//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。

use std::path::PathBuf;

use crate::error::起動エラー;

const 既定シェーダー監視パス: &str = "shaders/cube.slang";

/// 起動時に指定できる実行モード。
#[derive(Debug, Clone, Copy)]
pub(crate) enum 起動モード {
    /// ユーザーが閉じるまで無期限に実行する。
    無期限実行,
    /// 指定フレーム数を描画したら自動終了する（DoDのスモーク検証用）。
    スモーク実行 { フレーム数: u32 },
}

/// CLI引数から得た起動設定一式。
pub(crate) struct 起動設定 {
    pub(crate) モード: 起動モード,
    /// ホットリロードの監視対象。既定は`shaders/triangle.slang`(存在しなければ監視無効)。
    pub(crate) シェーダー監視パス: PathBuf,
}

/// `--frames N` と `--shader-source <path>` を解析する。
/// `--frames`指定が無ければ無期限実行、`--shader-source`指定が無ければ既定パス。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut モード = 起動モード::無期限実行;
    let mut シェーダー監視パス = PathBuf::from(既定シェーダー監視パス);

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        match 引数値.as_str() {
            "--frames" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::フレーム数引数不正("--framesに値が指定されていない".to_string())
                })?;
                let フレーム数 = 値
                    .parse::<u32>()
                    .map_err(|_| 起動エラー::フレーム数引数不正(値.clone()))?;
                モード = 起動モード::スモーク実行 { フレーム数 };
            }
            "--shader-source" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::シェーダーソース引数不正(
                        "--shader-sourceに値が指定されていない".to_string(),
                    )
                })?;
                シェーダー監視パス = PathBuf::from(値);
            }
            _ => {}
        }
    }

    Ok(起動設定 { モード, シェーダー監視パス })
}
