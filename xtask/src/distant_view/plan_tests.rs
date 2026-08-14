//! 遠景固定構図の入口と、終了報告から読む視点高契約の検査。

#![allow(clippy::unwrap_used)]

use super::plan::{実行の別, 引数を読む, 構図を検査する};
use crate::acceptance::{検収の実行名, 終了時報告};

#[test]
fn 全ての実行の別を一つずつ受け付ける() {
    for (綴り, 期待) in [
        ("--capture-reference", 実行の別::対照を採る),
        ("--capture-candidate", 実行の別::候補を採る),
        ("--capture-reference-no-ssao", 実行の別::Ssaoなし対照を採る),
        ("--capture-candidate-no-ssao", 実行の別::Ssaoなし候補を採る),
        ("--capture-candidate-no-distant-shadow", 実行の別::遠景影なし候補を採る),
        ("--print-plan", 実行の別::計画を表示する),
        ("--judge", 実行の別::判定する),
    ] {
        assert_eq!(引数を読む(&[綴り.to_string()]).unwrap(), 期待);
    }
}

#[test]
fn 複数の実行指定を拒む() {
    let 引数 = ["--capture-reference".to_string(), "--capture-candidate".to_string()];
    assert!(引数を読む(&引数).is_err());
}

#[test]
fn 報告したカメラ高が地表高より一メートル半高いことを課す() {
    assert!(構図を検査する(&報告("3.750000")).is_ok());
    assert!(構図を検査する(&報告("3.749000")).is_err());
}

fn 報告(カメラ高: &str) -> 終了時報告 {
    let 本文 = format!("  プレイヤーの大域位置: 東0.000000 天頂2.250000 南0.000000\n  カメラの大域位置: 東0.000000 天頂{カメラ高} 南9.000000\n");
    終了時報告::取り込む(&検収の実行名::生成する("view_contract").unwrap(), 本文, String::new())
}
