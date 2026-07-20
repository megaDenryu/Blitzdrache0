//! ミップ数の計算。

/// `floor(log2(max(幅,高さ)))+1`。ビット長として整数演算のみで求め、
/// 浮動小数の丸め誤差を避ける。
pub(super) fn 計算する(幅: u32, 高さ: u32) -> u32 {
    let 最大辺 = 幅.max(高さ).max(1);
    u32::BITS - 最大辺.leading_zeros()
}
