//! 接触の場面の試験が共通して読む、接触の一刻みの工程そのものを組む部分(判断18・19)。
//! 細分数16と細分の刻み幅1/960秒は判断19が確定した値である(2026-09-04の箱の塔の実測)。
//! 剛体の登録(`tower_fixture` など)と分けているのは、こちらが刻み幅と品質と混合則という起動時の設定を決める工程であり、あちらが世界に物を置く工程だからである。

use blitz_collision::dynamic_index::{動く形の空間索引, 箱に持たせるゆとりの幅};
use blitz_math::{メートル毎秒, メートル毎秒毎秒, 秒};

use super::pipeline_def::剛体の接触の一刻みの工程;
use super::pipeline_history::接触履歴の保持;
use super::pipeline_policy::接触の品質と時間方針;
use super::pipeline_solver::接触の解法ソルバー;
use super::pipeline_space::接触の空間と世界;
use crate::constraint_graph::一様な加速度;
use crate::contact::material_id::材質の識別子;
use crate::contact::mixing_rule_builder::混合則の組み立て;
use crate::contact::solver_quality::接触を解く品質の設定;
use crate::contact::velocity_stage::接触の速度段階;
use crate::rigid_xpbd::{ジャイロ項の扱い, 細分の予測器, 細分数};
use crate::xpbd::刻み幅;

/// 判断19が2026-09-04の箱の塔の実測で確定した細分数。
pub(super) const 確定した細分数: u32 = 16;

pub(super) fn 試験の材質() -> 材質の識別子 {
    材質の識別子::生成する(0)
}

/// 判断19が確定した細分数16で組む。細分数を変えた比較の検査だけが `細分数を指定して試験の接触の一刻みの工程を組む` を読む。
pub(super) fn 試験の接触の一刻みの工程を組む() -> 剛体の接触の一刻みの工程 {
    細分数を指定して試験の接触の一刻みの工程を組む(確定した細分数)
}

pub(super) fn 細分数を指定して試験の接触の一刻みの工程を組む(細分の数: u32) -> 剛体の接触の一刻みの工程 {
    let 基本秒 = 秒::生成する(1.0 / 60.0);
    let Ok(基本幅) = 刻み幅::生成する(基本秒) else {
        panic!();
    };
    let Ok(n) = 細分数::生成する(細分の数) else {
        panic!();
    };
    let Ok(細分の数の非零) = u16::try_from(細分の数) else {
        panic!();
    };
    let 細分秒 = 秒::生成する(1.0 / (60.0 * f32::from(細分の数の非零)));
    let Ok(細分幅) = 刻み幅::生成する(細分秒) else {
        panic!();
    };
    let Ok(品質) = 接触を解く品質の設定::生成する(2, 4) else {
        panic!();
    };
    let mut 組み立て = 混合則の組み立て::生成する();
    let mat = 試験の材質();
    let Ok(摩擦) = crate::contact::friction_coefficient::摩擦係数::生成する(0.5) else {
        panic!();
    };
    let 反発 = crate::contact::restitution_coefficient::反発係数::零();
    let Ok(物性) = crate::contact::surface_property::表面物性::生成する(摩擦, 摩擦, 反発) else {
        panic!();
    };
    let Ok(()) = 組み立て.表面物性を登録する(mat, 物性) else {
        panic!();
    };
    let Ok(混合則) = 組み立て.組み立てる() else {
        panic!();
    };
    let 重力 = 一様な加速度::成分から生成する(
        メートル毎秒毎秒::生成する(0.0),
        メートル毎秒毎秒::生成する(-9.8),
        メートル毎秒毎秒::生成する(0.0),
    );
    let 予測器 = 細分の予測器::生成する(細分幅, 重力, ジャイロ項の扱い::陰的に一段解く);
    let 速度段階 = 接触の速度段階::生成する(細分幅, メートル毎秒::生成する(0.01));
    let 空間索引 = 動く形の空間索引::ゆとりの幅から生成する(箱に持たせるゆとりの幅::人型と家具の大きさに見合う既定の幅());
    剛体の接触の一刻みの工程::生成する(
        接触の品質と時間方針::生成する(基本幅, n, 細分幅, 品質, 混合則),
        接触の空間と世界::生成する(空間索引),
        接触履歴の保持::見込みの接触点の数で生成する(64),
        接触の解法ソルバー::生成する(予測器, 速度段階),
    )
}
