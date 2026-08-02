//! 梱包工程の検証の検査。係数・特徴ビット・スロットの整合をレコードを作った直後に確かめることを固定する。

#![allow(clippy::unwrap_used)]

use crate::error::{レンダラーエラー, 材質資源表エラー};
use crate::vulkan::material_table::generation_build::構築する;
use crate::vulkan::material_table::generation_id::資源表世代ID;
use crate::vulkan::material_table::{material_id::大域材質ID, pack_input::梱包対象材質};

use super::fixture::{余裕のあるレイアウト容量, 検査用供給元};

#[test]
fn 範囲外の係数を型付きの失敗にする() {
    let mut 供給元 = 検査用供給元::常に成功する();
    let 材質一覧 = [梱包対象材質::生成する(
        大域材質ID::生成する(1),
        [1.0, 1.0, 1.0, 1.0],
        1.5,
        0.5,
        [None, None, None],
    )];
    let 結果 = 構築する(&mut 供給元, 資源表世代ID::最初(), 余裕のあるレイアウト容量(), &材質一覧);
    assert!(matches!(
        結果,
        Err(レンダラーエラー::材質資源表不正(材質資源表エラー::係数の範囲外 {
            材質id: 1,
            ..
        }))
    ));
}
