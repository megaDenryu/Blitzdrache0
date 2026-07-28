//! 高さ格子をファイルのバイト列へ格納する工程。ヘッダーに諸元、本体に標本をリトルエンディアンのf32で並べる。
//! 走査順は格子の保持順(Zが外側、Xが内側)をそのまま使うため、同じ格子からは常に同じバイト列を得る。

use super::grid::高さ格子;
use super::{ヘッダー長, 固定識別値, 形式版};

pub fn 高さ格子を格納する(格子: &高さ格子) -> Vec<u8> {
    let 諸元 = 格子.諸元();
    let 本体長 = 格子.高さ一覧().len().saturating_mul(4);
    let mut バイト列 = Vec::with_capacity(ヘッダー長.saturating_add(本体長));
    バイト列.extend_from_slice(&固定識別値);
    バイト列.extend_from_slice(&形式版.to_le_bytes());
    バイト列.extend_from_slice(&諸元.辺分割数().to_le_bytes());
    バイト列.extend_from_slice(&諸元.重なり幅().to_le_bytes());
    バイト列.extend_from_slice(&諸元.一辺メートル().to_le_bytes());
    for 高さ in 格子.高さ一覧() {
        バイト列.extend_from_slice(&高さ.to_le_bytes());
    }
    バイト列
}
