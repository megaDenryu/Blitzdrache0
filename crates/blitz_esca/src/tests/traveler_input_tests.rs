//! 旅行者のキーボード入力解釈テスト(Issue #137)。

use crate::traveler::旅行者の意図;
use crate::traveler_input::キーボード歩行入力;
use crate::walking_direction::歩行方向;

#[test]
fn 入力解釈_何も押していない時は静止になる() {
    let 入力 = キーボード歩行入力::default();
    assert_eq!(入力.歩行入力を解釈する(), 旅行者の意図::静止);
}

#[test]
fn 入力解釈_単一方向キーで該当方角へ歩く意図になる() {
    let 入力_上 = キーボード歩行入力 { 上: true, ..Default::default() };
    assert_eq!(入力_上.歩行入力を解釈する(), 旅行者の意図::歩く { 方向: 歩行方向::北() });

    let 入力_右 = キーボード歩行入力 { 右: true, ..Default::default() };
    assert_eq!(入力_右.歩行入力を解釈する(), 旅行者の意図::歩く { 方向: 歩行方向::東() });
}

#[test]
fn 入力解釈_相反するキー同時押しは相殺されて静止になる() {
    let 入力 = キーボード歩行入力 { 上: true, 下: true, 左: true, 右: true };
    assert_eq!(入力.歩行入力を解釈する(), 旅行者の意図::静止);
}
