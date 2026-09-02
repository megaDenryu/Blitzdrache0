//! 反発係数: 衝突の前後の法線方向の相対速度の比を表す無次元の量(判断15)。0で跳ね返らず、1で速さが保たれる。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断15: 接触物性は両表面の物性から混合則で決め、既定則と材質対の上書き表を持つ」

use super::property_error::接触物性エラー;

/// 0以上1以下の有限の無次元量。1を超える値は衝突がエネルギーを生む状態であるため拒む。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 反発係数(f32);

impl 反発係数 {
    /// 0未満と1超と非有限を型付きエラーで拒む。
    pub fn 生成する(値: f32) -> Result<Self, 接触物性エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(接触物性エラー::反発係数が値域の外 { 値 });
        }
        Ok(Self(値))
    }

    /// 跳ね返らない表面。
    pub fn 零() -> Self {
        Self(0.0)
    }

    /// 既定則が2つの表面の反発を混ぜる規則である最大値。よく弾む側が知覚を支配し、引数の順で結果が変わらない。
    pub fn 大きい方を求める(self, 相手: Self) -> Self {
        if 相手.0 > self.0 { 相手 } else { self }
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}
