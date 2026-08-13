//! 引数を移行の段へ写す。対照の上書きと候補比較を明示的に分ける。

use super::error::逆Z検収エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 実行の別 {
    対照を採る,
    候補を比較する,
    計画を表示する,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の別, 逆Z検収エラー> {
    match 引数一覧 {
        [引数] if 引数 == "--capture-reference" => Ok(実行の別::対照を採る),
        [引数] if 引数 == "--compare-candidate" => Ok(実行の別::候補を比較する),
        [引数] if 引数 == "--print-plan" => Ok(実行の別::計画を表示する),
        _ => Err(逆Z検収エラー::引数が不正(引数一覧.join(" "))),
    }
}

pub(super) fn 計画を表示する() -> String {
    "scene=terrain_origin frames=120 time-of-day=43200 depth-prepass=equal streaming-radius=2 color=RGBA8 depth=D32".to_string()
}
