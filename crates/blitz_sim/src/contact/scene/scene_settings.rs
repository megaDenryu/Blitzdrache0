//! 場面の設定と、細分1本の観測(判断19の細分の工程の試験専用の実装)。どちらも振る舞いを持たないデータであり、
//! 場面の状態と工程を持つ`substep_harness`から分けている。設定の欄は同じ型の値が多く並ぶため、位置でなく名前で渡す。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断19: 剛体は基本刻みを整数nで細分し、細分1本の中で予測・接触・反復・速度再構成・速度段階を回す」

#![cfg(test)]

use blitz_math::{キログラム, メートル, ワールド, 速度};

use super::super::normal_tangential_system::解けたと見なす許容差の倍率;
use super::super::solver_quality::接触を解く品質の設定;
use super::super::surface_property::表面物性;
use super::residual_separation::場面の残差の分離;
use super::static_friction_method::場面の静止摩擦の解き方;
use crate::constraint_graph::一様な加速度;
use crate::rigid_body::配置;
use crate::xpbd::{ラグランジュ乗数, 刻み幅};

// 場面を組み立てる設定。欄が同じ型の値を多く持つため、位置でなく名前で渡す。
pub(super) struct 場面の設定 {
    pub 箱の半分の長さ: メートル,
    pub 箱の質量: キログラム,
    pub 箱の配置: 配置,
    pub 箱の初速: 速度<ワールド>,
    pub 静的な直方体の配置: 配置,
    pub 静的な直方体の半分の長さ: メートル,
    pub 表面物性: 表面物性,
    pub 重力: 一様な加速度,
    pub 細分の刻み幅: 刻み幅,
    pub 解く品質: 接触を解く品質の設定,
    pub 静止摩擦の解き方: 場面の静止摩擦の解き方,
    pub 残差の分離: 場面の残差の分離,
    pub 解けたと見なす許容差の倍率: 解けたと見なす許容差の倍率,
}

// 細分1本で観測した接触の数と、反復を終えた法線の乗数の合計と最大の食い込み。滑走と履歴の対応付けと、
// 法線の乗数の合計が真の法線力積 m g cosθ h² からどれだけ離れるかと、位置の反復回数を増やしても
// 食い込みが増えないことを試験が読む。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct 細分の観測 {
    pub 接触点の数: usize,
    pub 開始した接触点の数: usize,
    pub 滑走中の接触点の数: usize,
    pub 法線の乗数の合計: ラグランジュ乗数,
    pub 最大の食い込み: メートル, // 反復を終えた配置で最も深い −隔たり。食い込みが無ければ零
}

impl Default for 細分の観測 {
    fn default() -> Self {
        Self {
            接触点の数: 0,
            開始した接触点の数: 0,
            滑走中の接触点の数: 0,
            法線の乗数の合計: ラグランジュ乗数::零(),
            最大の食い込み: メートル::生成する(0.0),
        }
    }
}
