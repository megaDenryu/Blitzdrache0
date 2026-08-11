//! 引数解釈の正例と負例の固定。担当するのは、候補の軸をちょうど1つだけ受ける契約と、構図が受け入れる軸の
//! 制限が実際に効いていることを固定することである。手で回す負例は退行を防がないため、検証列へ入る形にする。

use super::引数を読む;
use super::計器の様式;
use crate::shadow_loss::candidate_axis::計測軸;
use crate::shadow_loss::scene_choice::構図;

fn 読む(語一覧: &[&str]) -> Result<super::指定, super::影の欠落計器の引数の破れ> {
    引数を読む(&語一覧.iter().map(|語| (*語).to_string()).collect::<Vec<String>>())
}

#[test]
fn 候補が無ければ落ちる() {
    assert!(読む(&[]).is_err());
    assert!(読む(&["--layout", "range"]).is_err());
}

#[test]
fn rangeの構図は最大影距離の候補を受けない() {
    assert!(読む(&["--layout", "range", "--max-shadow-distance", "200"]).is_err());
}

#[test]
fn 別の軸を2つ渡すと落ちる() {
    assert!(読む(&["--max-shadow-distance", "200", "--shadow-caster-range", "80"]).is_err());
}

#[test]
fn 同じ軸を2回渡すと落ちる() {
    assert!(読む(&["--shadow-caster-range", "80", "--shadow-caster-range", "120"]).is_err());
}

#[test]
fn 構図を2回渡すと落ちる() {
    assert!(読む(&["--layout", "terrain", "--layout", "range", "--shadow-caster-range", "80"]).is_err());
}

#[test]
fn 正でない距離と数でない距離は落ちる() {
    assert!(読む(&["--shadow-caster-range", "-5"]).is_err());
    assert!(読む(&["--shadow-caster-range", "0"]).is_err());
    assert!(読む(&["--shadow-caster-range", "abc"]).is_err());
}

#[test]
fn 値の無い語と知らない語は落ちる() {
    assert!(読む(&["--shadow-caster-range"]).is_err());
    assert!(読む(&["--layout"]).is_err());
    assert!(読む(&["--knob", "1"]).is_err());
}

#[test]
fn 構図を省くと地形になり候補の綴りがそのまま渡る() {
    let 指定 = 読む(&["--shadow-caster-range", "80"]).unwrap_or_else(|誤り| panic!("正例が落ちた: {誤り}"));
    assert_eq!(指定.構図, 構図::地形);
    assert_eq!(指定.候補.軸(), 計測軸::影の視距離);
    assert_eq!(指定.候補.起動指定へ写す(), vec!["--shadow-caster-range".to_string(), "80".to_string()]);
}

#[test]
fn rangeの構図は影の視距離の候補を受ける() {
    let 指定 = 読む(&["--layout", "range", "--shadow-caster-range", "80"]).unwrap_or_else(|誤り| panic!("正例が落ちた: {誤り}"));
    assert_eq!(指定.構図, 構図::影視距離の検収);
    assert_eq!(指定.候補.軸(), 計測軸::影の視距離);
}

#[test]
fn 地形の構図は最大影距離の候補を受ける() {
    let 指定 = 読む(&["--layout", "terrain", "--max-shadow-distance", "200"]).unwrap_or_else(|誤り| panic!("正例が落ちた: {誤り}"));
    assert_eq!(指定.構図, 構図::地形);
    assert_eq!(指定.候補.軸(), 計測軸::最大影距離);
}

#[test]
fn 様式の既定は数える側であり最終色の旗で撮る側になる() {
    let 既定 = 読む(&["--shadow-caster-range", "80"]).unwrap_or_else(|誤り| panic!("正例が落ちた: {誤り}"));
    assert_eq!(既定.様式, 計器の様式::影の欠落を数える);
    let 撮る = 読む(&["--max-shadow-distance", "200", "--final-color"]).unwrap_or_else(|誤り| panic!("正例が落ちた: {誤り}"));
    assert_eq!(撮る.様式, 計器の様式::最終色の絵を撮る);
    assert_eq!(撮る.候補.距離の綴り(), "200", "距離の綴りは撮った絵の名前に入る");
}
