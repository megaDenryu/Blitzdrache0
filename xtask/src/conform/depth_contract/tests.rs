//! 片側だけを動かした綴りを深度契約が拒むことの検査。

use super::指定の綴りがあるか;

#[test]
fn 同じ消去値は受ける() {
    assert!(指定の綴りがあるか("深度領域::カメラ => 0.0,", "深度領域::カメラ => 0.0"));
}

#[test]
fn 消去値の片側変更を拒む() {
    assert!(!指定の綴りがあるか("深度領域::カメラ => 1.0,", "深度領域::カメラ => 0.0"));
}

#[test]
fn 比較演算の片側変更を拒む() {
    assert!(!指定の綴りがあるか(
        "depth_compare_op(vk::CompareOp::LESS)",
        "depth_compare_op(vk::CompareOp::GREATER)",
    ));
}
