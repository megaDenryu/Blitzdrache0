//! 画面空間の画像1枚の縦横の画素数という量そのもの。触れる状態を持たず、担うのは値域の保証と、
//! 寸法から導ける画素数と通し番号を1箇所で決めることだけである。
//!
//! 深度画像と局所可視度画像が同じ型を共有するのは、2つが必ず同じ寸法で対になるためである。
//! 別々の寸法型にすると、対にならない組み合わせを呼び出し側の規律でしか止められなくなる。

use super::error::局所可視性エラー;

/// 画面空間の画像の縦横の画素数。
/// 不変条件: 幅も高さも2以上である(中央差分の隣点が軸ごとに少なくとも1つ要る)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 画像の寸法 {
    幅: u32,
    高さ: u32,
}

impl 画像の寸法 {
    pub fn 生成する(幅: u32, 高さ: u32) -> Result<Self, 局所可視性エラー> {
        if 幅 < 2 {
            return Err(局所可視性エラー::整数値域外("画像の幅", 幅));
        }
        if 高さ < 2 {
            return Err(局所可視性エラー::整数値域外("画像の高さ", 高さ));
        }
        Ok(Self { 幅, 高さ })
    }

    pub fn 幅(self) -> u32 {
        self.幅
    }

    pub fn 高さ(self) -> u32 {
        self.高さ
    }

    pub fn 画素数(self) -> usize {
        let 幅 = usize::try_from(self.幅).unwrap_or_else(|_| panic!("画像の幅{}がusizeに収まらない", self.幅));
        let 高さ = usize::try_from(self.高さ).unwrap_or_else(|_| panic!("画像の高さ{}がusizeに収まらない", self.高さ));
        幅 * 高さ
    }

    pub fn 含むか(self, 横: u32, 縦: u32) -> bool {
        横 < self.幅 && 縦 < self.高さ
    }

    /// 行優先で数えた画素の位置。
    /// 前提: 呼び出し元は`含むか`で範囲を確かめている。
    pub fn 通し番号(self, 横: u32, 縦: u32) -> usize {
        let 幅 = usize::try_from(self.幅).unwrap_or_else(|_| panic!("画像の幅{}がusizeに収まらない", self.幅));
        let 横 = usize::try_from(横).unwrap_or_else(|_| panic!("横位置{横}がusizeに収まらない"));
        let 縦 = usize::try_from(縦).unwrap_or_else(|_| panic!("縦位置{縦}がusizeに収まらない"));
        縦 * 幅 + 横
    }
}
