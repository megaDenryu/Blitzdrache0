//! 反射率積分表を何テクセルで焼くかの品質方針。
//!
//! 立方体画像の解像度と別の型にするのは、この表が方向を持たない2次元の表であり、横と縦が別々の量
//! (法線と視線の余弦・粗さ)を表すためである。正方形に見えても2つの軸を1つの一辺へ畳むと、
//! 軸ごとに刻みを変えたくなったときに型が嘘をつく。
//!
//! 既定は256×256である(参照: `_doc/設計/放射輝度問い合わせ階層.md`「遠方環境の形式と生成(3-Ia)」)。

use super::error::派生表現エラー;

/// 反射率積分表の解像度。横が法線と視線の余弦の刻み、縦が粗さの刻みである。
/// 不変条件: 横も縦も2以上である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 反射率積分表の解像度 {
    横: u32,
    縦: u32,
}

impl 反射率積分表の解像度 {
    pub fn 生成する(横: u32, 縦: u32) -> Result<Self, 派生表現エラー> {
        if 横 < 2 {
            return Err(派生表現エラー::整数値域外("反射率積分表の横", 横));
        }
        if 縦 < 2 {
            return Err(派生表現エラー::整数値域外("反射率積分表の縦", 縦));
        }
        Ok(Self { 横, 縦 })
    }

    pub fn 既定値() -> Self {
        Self::生成する(256, 256).unwrap_or_else(|誤り| panic!("反射率積分表の既定の解像度が値域を外れた: {誤り}"))
    }

    pub fn 横(self) -> u32 {
        self.横
    }

    pub fn 縦(self) -> u32 {
        self.縦
    }

    pub fn 全テクセル数(self) -> usize {
        let 横 = usize::try_from(self.横).unwrap_or_else(|_| panic!("反射率積分表の横{}がusizeに収まらない", self.横));
        let 縦 = usize::try_from(self.縦).unwrap_or_else(|_| panic!("反射率積分表の縦{}がusizeに収まらない", self.縦));
        横 * 縦
    }
}
