//! 場面の設定と、細分1本の観測(判断19の細分の工程の試験専用の実装)。どちらも振る舞いを持たないデータであり、
//! 場面の状態と工程を持つ`substep_harness`から分けている。設定の欄は同じ型の値が多く並ぶため、位置でなく名前で渡す。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断19: 剛体は基本刻みを整数nで細分し、細分1本の中で予測・接触・反復・速度再構成・速度段階を回す」

#![cfg(test)]

use blitz_math::{ワールド, 速度};

use super::super::surface_property::表面物性;
use crate::constraint_graph::一様な加速度;
use crate::rigid_body::配置;
use crate::xpbd::刻み幅;

// 場面を組み立てる設定。欄が同じ型の値を多く持つため、位置でなく名前で渡す。
pub(super) struct 場面の設定 {
    pub 箱の半分の長さ: f32,
    pub 箱の質量: f32,
    pub 箱の配置: 配置,
    pub 箱の初速: 速度<ワールド>,
    pub 静的な直方体の配置: 配置,
    pub 静的な直方体の半分の長さ: f32,
    pub 表面物性: 表面物性,
    pub 重力: 一様な加速度,
    pub 細分の刻み幅: 刻み幅,
    pub 反復回数: usize,
    pub 速度段階の巡回数: usize,
    pub 静止摩擦の位置拘束を外すか: bool,
}

// 細分1本で観測した接触の数。滑走と履歴の対応付けを試験が読む。
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(super) struct 細分の観測 {
    pub 接触点の数: usize,
    pub 開始した接触点の数: usize,
    pub 滑走中の接触点の数: usize,
}
