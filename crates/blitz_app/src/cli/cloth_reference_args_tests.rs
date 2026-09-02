//! `--cloth-xpbd-reference`の後ろに置く曲げのコンプライアンスと題材の形の指定の解析を検証する(Issue #38)。
//! 参照比較の方式より前に置いた指定は、どの題材へ与えるかが無いため型付きの失敗になる(黙って読み捨てない)ことを固定する。

use super::{
    参照比較の床の下の固定点, 参照比較の題材の形, 布モード, 引数を解析する, 起動引数エラー, 起動要求
};
use crate::error::起動エラー;

fn 布モードを解析する(引数一覧: &[&str]) -> Result<布モード, 起動引数エラー> {
    let 引数一覧: Vec<String> = 引数一覧.iter().map(|引数| (*引数).to_string()).collect();
    match 引数を解析する(&引数一覧) {
        Ok(起動要求::描画実行(設定)) => Ok(設定.布モード),
        Ok(報告) => panic!("描画実行の要求になるはず(報告の種別{})", 報告.呼び名()),
        Err(起動エラー::起動引数不正(誤り)) => Err(誤り),
        Err(誤り) => panic!("引数の失敗になるはず: {誤り}"),
    }
}

#[test]
fn 参照比較の既定は垂直に吊るした上端の行の固定で曲げのコンプライアンスは10である() {
    match 布モードを解析する(&["--cloth-xpbd-reference", "0.01"]) {
        Ok(布モード::XPBD参照比較 {
            床の下の固定点,
            曲げのコンプライアンス,
            題材の形,
            ..
        }) => {
            assert_eq!(床の下の固定点, 参照比較の床の下の固定点::持たない);
            assert_eq!(曲げのコンプライアンス.値(), 10.0);
            assert_eq!(題材の形, 参照比較の題材の形::垂直に吊るして上端の行を固定);
        }
        他 => panic!("参照比較の方式になるはず: {他:?}"),
    }
}

#[test]
fn 参照比較の後ろの曲げと題材の形の指定はその方式へ入る() {
    match 布モードを解析する(&[
        "--cloth-xpbd-reference",
        "0",
        "--cloth-xpbd-reference-shape",
        "horizontal-one-point",
        "--cloth-xpbd-reference-bending",
        "1000",
    ]) {
        Ok(布モード::XPBD参照比較 {
            曲げのコンプライアンス,
            題材の形,
            ..
        }) => {
            assert_eq!(曲げのコンプライアンス.値(), 1000.0);
            assert_eq!(題材の形, 参照比較の題材の形::水平に敷いて上端の左の一点を固定);
        }
        他 => panic!("参照比較の方式になるはず: {他:?}"),
    }
}

/// 反証: 方式より前の指定を黙って読み捨てると、曲げを与えたつもりの検収が既定の曲げで通る。
#[test]
fn 参照比較より前の指定と読めない綴りは型付きの失敗になる() {
    assert!(matches!(
        布モードを解析する(&["--cloth-xpbd-reference-bending", "1", "--cloth-xpbd-reference", "0"]),
        Err(起動引数エラー::布の曲げのコンプライアンス不正(_))
    ));
    assert!(matches!(
        布モードを解析する(&["--cloth-xpbd-reference-shape", "horizontal-top-row"]),
        Err(起動引数エラー::参照比較の題材の形不正(_))
    ));
    assert!(matches!(
        布モードを解析する(&["--cloth-xpbd-reference", "0", "--cloth-xpbd-reference-shape", "diagonal"]),
        Err(起動引数エラー::参照比較の題材の形不正(_))
    ));
    assert!(matches!(
        布モードを解析する(&["--cloth-xpbd-reference", "0", "--cloth-xpbd-reference-bending", "-1"]),
        Err(起動引数エラー::布の曲げのコンプライアンス不正(_))
    ));
}
