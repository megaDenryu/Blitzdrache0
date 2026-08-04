//! ヒストグラムが数える画素の件数と、件数を重み付き平均の重みへ写す境界。
//!
//! 標準ライブラリは32ビット整数から単精度への型付き変換を持たず、数値の`as`キャストは規約で禁じているため、
//! 上位16ビットと下位16ビットへ分けて足す。どちらも16ビット整数であり単精度で正確に表せるため、この分解が
//! 入れる丸めは最後の1回の加算だけであり、結果は最近接の単精度と一致する。

/// 数え上げた画素の件数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct 画素件数(u32);

impl 画素件数 {
    pub fn 生成する(値: u32) -> Self {
        Self(値)
    }

    pub fn 零件() -> Self {
        Self(0)
    }

    pub fn 値(&self) -> u32 {
        self.0
    }

    /// 1件数える。上限で頭打ちにするのは、件数が巻き戻ると分布が別物になるためである。
    /// 前提: 1フレームの画素数は32ビット整数の上限より桁違いに少ない。
    pub fn 増やす(&self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn 足す(&self, 相手: Self) -> Self {
        Self(self.0.saturating_add(相手.0))
    }

    /// 重み付き平均の重みとして使う実数の値。
    pub fn 実数で表す(&self) -> f32 {
        let [下位の下, 下位の上, 上位の下, 上位の上] = self.0.to_le_bytes();
        let 上位 = u16::from_le_bytes([上位の下, 上位の上]);
        let 下位 = u16::from_le_bytes([下位の下, 下位の上]);
        f32::from(上位) * 65536.0 + f32::from(下位)
    }
}
