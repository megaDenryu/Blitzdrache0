//! 機材の機能(テクスチャのブロック圧縮・立方体の配列画像)による選び分けの検査。
//! 欠く候補を飛ばすことと、1台も残らないときにどちらの条件で落ちたのかを区別して機材名を並べることを見る。
//!
//! 条件を順に掛けるため、先に落ちた条件だけが報告される。この順序が変わると、
//! 2つとも欠けた機材で報告される機能名が入れ替わる。

#![allow(clippy::unwrap_used)]

use super::candidate_fixture::{ブロック圧縮を選べる候補, 候補, 機能を選べる候補};
use super::choose::選ぶ;
use crate::error::デバイス要件エラー;

#[test]
fn ブロック圧縮を欠く候補を飛ばして対応する候補を選ぶ() {
    let 候補一覧 = [
        ブロック圧縮を選べる候補(0, "discrete GPU", true, true, false),
        候補(1, "統合GPU", false, true),
    ];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
}

#[test]
fn 索引に対応する候補が全てブロック圧縮を欠くと機材名を並べて報告する() {
    let 候補一覧 = [ブロック圧縮を選べる候補(0, "discrete GPU", true, true, false)];
    let Err(デバイス要件エラー::テクスチャのブロック圧縮非対応(機材名一覧)) = 選ぶ(&候補一覧) else {
        panic!("ブロック圧縮の非対応以外の結果が返った");
    };
    assert_eq!(機材名一覧, vec!["discrete GPU".to_string()]);
}

/// 立方体の配列画像を欠く候補を飛ばし、1台も残らないなら機材名を並べて報告する。
#[test]
fn 立方体の配列画像を欠く候補を飛ばして対応する候補を選ぶ() {
    let 候補一覧 = [
        機能を選べる候補(0, "discrete GPU", true, true, true, false),
        候補(1, "統合GPU", false, true),
    ];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
    let Err(デバイス要件エラー::立方体の配列画像非対応(機材名一覧)) = 選ぶ(&[機能を選べる候補(0, "discrete GPU", true, true, true, false)])
    else {
        panic!("立方体の配列画像の非対応以外の結果が返った");
    };
    assert_eq!(機材名一覧, vec!["discrete GPU".to_string()]);
}
