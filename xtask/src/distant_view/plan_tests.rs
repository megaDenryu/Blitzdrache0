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
        ("--capture-reference-no-post", 実行の別::後処理なし対照を採る),
        ("--capture-candidate-no-post", 実行の別::後処理なし候補を採る),
        ("--capture-shadow-reference", 実行の別::影の対照を採る),
        ("--capture-shadow-candidate", 実行の別::影の候補を採る),
        ("--capture-shadow-reference-visibility", 実行の別::影の対照の可視度を採る),
        ("--capture-shadow-candidate-visibility", 実行の別::影の候補の可視度を採る),
        ("--judge-shadow", 実行の別::影を判定する),
        ("--print-plan", 実行の別::計画を表示する),
        ("--judge", 実行の別::判定する),
    ] {
        assert_eq!(引数を読む(&[綴り.to_string()]).unwrap(), 期待);
    }
}

/// 主判定が読む対だけが後処理を組まない。旗が採取の別と食い違うと、判定が別の構成の絵を突き合わせる。
#[test]
fn 後処理なしの採取だけが後処理を切る() {
    for (別, 後処理を使わない) in [
        (実行の別::対照を採る, false),
        (実行の別::候補を採る, false),
        (実行の別::Ssaoなし候補を採る, false),
        (実行の別::遠景影なし候補を採る, false),
        (実行の別::後処理なし対照を採る, true),
        (実行の別::後処理なし候補を採る, true),
    ] {
        assert_eq!(別.採取条件().unwrap().後処理を使わない, 後処理を使わない);
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

/// 影の検査点の4つの採取は、対照側だけが再配分を切り、可視度側だけが診断色を出す。
/// 4つとも後処理なしである。1つでも取り違えると、判定は別の条件の絵を突き合わせる。
#[test]
fn 影の検査点の四つの採取は対照だけが再配分を切る() {
    for (別, 明示境界を使わない, 影可視度を可視化する) in [
        (実行の別::影の対照を採る, true, false),
        (実行の別::影の候補を採る, false, false),
        (実行の別::影の対照の可視度を採る, true, true),
        (実行の別::影の候補の可視度を採る, false, true),
    ] {
        let 条件 = 別.採取条件().unwrap();
        assert_eq!(条件.明示境界を使わない, 明示境界を使わない);
        assert_eq!(条件.影可視度を可視化する, 影可視度を可視化する);
        assert!(条件.後処理を使わない, "{}は後処理なしで採る", 条件.名前);
    }
}

/// 遠景の検査点が読む採取は再配分にも診断にも触れない。影の旗が遠景の対へ漏れると、
/// 工程Dで閉じた判定が別の条件の絵を見ることになる。
#[test]
fn 遠景の検査点の採取は影の旗を立てない() {
    for 別 in [
        実行の別::対照を採る,
        実行の別::候補を採る,
        実行の別::後処理なし対照を採る,
        実行の別::後処理なし候補を採る,
    ] {
        let 条件 = 別.採取条件().unwrap();
        assert!(!条件.明示境界を使わない);
        assert!(!条件.影可視度を可視化する);
    }
}
