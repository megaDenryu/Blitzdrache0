//! 実行の指定の引数解釈の検査: 既定と絞り込みと、知らない語と零の拒みを見る。

#![allow(clippy::unwrap_used)]

use super::plan::引数を読む;
use super::schedule::{グラフ, 方式};
#[test]
fn 既定は全方式と全グラフで反復4回の刻み240本である() {
    let 指定 = 引数を読む(&[]).unwrap();
    assert_eq!(
        (
            指定.方式一覧.len(),
            指定.グラフ一覧.len(),
            指定.反復回数,
            指定.刻み数,
            指定.点の数,
            指定.比較の刻み数
        ),
        (3, 2, 4, 240, 1024, 10)
    );
}

#[test]
fn 方式とグラフを絞れて知らない語は落とす() {
    let 指定 = 引数を読む(&["--method".to_string(), "coloring".to_string(), "--graph".to_string(), "grid".to_string()]).unwrap();
    assert_eq!((指定.方式一覧, 指定.グラフ一覧), (vec![方式::グラフ彩色], vec![グラフ::規則格子]));
    assert!(引数を読む(&["--rounds".to_string()]).is_err());
    assert!(引数を読む(&["--steps".to_string(), "0".to_string()]).is_err());
    assert!(引数を読む(&["--method".to_string(), "fast".to_string()]).is_err());
}
