//! 大気の濁り具合。

use super::天空状態エラー;

/// 完全に澄んだ大気。Hosek-Wilkieの解析近似が定義する下限である。
const 最小濁度: f32 = 1.0;
/// Hosek-Wilkieの解析近似が定義する上限。これを超える濁りは近似の定義域外になる。
const 最大濁度: f32 = 10.0;
/// 晴天の代表値。
const 晴天の濁度: f32 = 3.0;

/// 大気の濁り具合を表す係数。空の放射輝度を解析近似で評価するときの入力になる。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 濁度(f32);

impl 濁度 {
    pub fn 生成する(値: f32) -> Result<Self, 天空状態エラー> {
        if !値.is_finite() || !(最小濁度..=最大濁度).contains(&値) {
            return Err(天空状態エラー::値域外("濁度", 値));
        }
        Ok(Self(値))
    }

    pub fn 晴天() -> Self {
        Self(晴天の濁度)
    }

    pub fn 値(&self) -> f32 {
        self.0
    }
}
