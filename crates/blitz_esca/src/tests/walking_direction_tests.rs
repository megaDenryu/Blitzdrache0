//! 歩行方向の導出テスト(Issue #137)。長さ1の不変条件と、相殺と正規化を固定する。

#![allow(clippy::expect_used)]

use crate::traveler_input::キーボード歩行入力;
use crate::walking_direction::歩行方向;

#[test]
fn 歩行方向_斜め入力は長さ1へ正規化される() {
    let 入力 = キーボード歩行入力 { 上: true, 右: true, ..Default::default() };
    let 方向 = 歩行方向::キー入力から導く(&入力).expect("斜めの入力からは向きが導けるべきである");
    let 期待値 = 1.0_f32 / 2.0_f32.sqrt();
    assert!((方向.東の比率() - 期待値).abs() < 1e-6);
    assert!((方向.北の比率() - 期待値).abs() < 1e-6);
    let 長さの二乗 = 方向.東の比率() * 方向.東の比率() + 方向.北の比率() * 方向.北の比率();
    assert!((長さの二乗 - 1.0).abs() < 1e-6);
}

#[test]
fn 歩行方向_相反するキーは相殺されて向きが無い() {
    let 入力 = キーボード歩行入力 { 左: true, 右: true, ..Default::default() };
    assert_eq!(歩行方向::キー入力から導く(&入力), None);
}

#[test]
fn 歩行方向_単位方向の関連関数はキー入力からの導出と一致する() {
    let 入力_左 = キーボード歩行入力 { 左: true, ..Default::default() };
    assert_eq!(歩行方向::キー入力から導く(&入力_左), Some(歩行方向::西()));
    let 入力_下 = キーボード歩行入力 { 下: true, ..Default::default() };
    assert_eq!(歩行方向::キー入力から導く(&入力_下), Some(歩行方向::南()));
}
