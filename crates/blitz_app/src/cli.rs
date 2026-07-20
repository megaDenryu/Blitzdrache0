//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。

use crate::error::起動エラー;

/// 起動時に指定できる実行モード。
#[derive(Debug, Clone, Copy)]
pub(crate) enum 起動モード {
    /// ユーザーが閉じるまで無期限に実行する。
    無期限実行,
    /// 指定フレーム数を描画したら自動終了する（DoDのスモーク検証用）。
    スモーク実行 { フレーム数: u32 },
}

/// `--frames N` を解析する。指定が無ければ無期限実行。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動モード, 起動エラー> {
    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        if 引数値 != "--frames" {
            continue;
        }
        let 値 = 引数.next().ok_or_else(|| {
            起動エラー::フレーム数引数不正("--framesに値が指定されていない".to_string())
        })?;
        let フレーム数 = 値
            .parse::<u32>()
            .map_err(|_| 起動エラー::フレーム数引数不正(値.clone()))?;
        return Ok(起動モード::スモーク実行 { フレーム数 });
    }
    Ok(起動モード::無期限実行)
}
