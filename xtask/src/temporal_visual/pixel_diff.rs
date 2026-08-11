//! 同じ寸法の2枚の絵が何画素で食い違うかを数える工程。受け取るのは2枚のRGBA8のバイト列、返すのは食い違った画素数である。
//! 何画素までを許すかは呼び出し元が決め、ここは数えることだけを担う。

pub(super) fn 食い違う画素を数える(左: &[u8], 右: &[u8]) -> Result<usize, String> {
    if 左.len() != 右.len() {
        return Err(format!("比べる2枚のバイト数が違う: {}と{}", 左.len(), 右.len()));
    }
    Ok(左
        .chunks_exact(4)
        .zip(右.chunks_exact(4))
        .filter(|(左画素, 右画素)| 左画素 != 右画素)
        .count())
}
