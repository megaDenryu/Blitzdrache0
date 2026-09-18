//! 移動可能範囲の生成が失敗する理由。破られた不変条件を選択肢で数え上げ、文で言う。

use blitz_math::メートル;

/// 移動可能範囲の2つの軸。失敗がどの軸で起きたかを言うために持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 移動可能範囲の軸 {
    東西,
    南北,
}

impl std::fmt::Display for 移動可能範囲の軸 {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::東西 => write!(書き手, "東西"),
            Self::南北 => write!(書き手, "南北"),
        }
    }
}

/// 移動可能範囲の指定が不変条件を満たさなかった理由。
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum 移動可能範囲の生成の失敗 {
    #[error("{軸}の最小{最小:?}が最大{最大:?}を超えている(移動可能範囲の各軸は最小が最大以下でなければならない)")]
    最小が最大を超えている { 軸: 移動可能範囲の軸, 最小: メートル, 最大: メートル },
    #[error("{軸}の端に数値でない値がある(移動可能範囲の端は数値でなければならない)")]
    数値でない { 軸: 移動可能範囲の軸 },
}
