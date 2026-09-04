//! 剛体の接触の一刻みの工程の単体試験が使う場面の組み立て(判断19)。
//! 試験そのもの(`pipeline_tests`)と分けているのは、こちらが場面を組む工程であり、あちらが測って判定する工程だからである。

use blitz_collision::dynamic_index::{動く形の空間索引, 箱に持たせるゆとりの幅};
use blitz_collision::shape::直方体の軸ごとの半分の長さ;
use blitz_math::{キログラム, メートル, ワールド, 位置, 秒};

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
use crate::rigid_body::{剛体の台帳, 剛体の識別子, 姿勢, 質量特性, 運動状態, 運動種別, 配置};
use crate::rigid_xpbd::{ジャイロ項の扱い, 細分の予測器, 細分数};
use crate::xpbd::刻み幅;

pub(super) const 基本刻みの秒: f32 = 1.0 / 60.0;

fn テスト配置() -> 配置 {
    let 零 = メートル::生成する(0.0);
    配置::生成する(位置::<ワールド>::生成する(零, 零, 零), 姿勢::恒等())
}

pub(super) fn テスト工程を作る(細分の数: u32) -> (剛体の接触の一刻みの工程, 剛体の台帳, 剛体の識別子) {
    let mut 台帳 = 剛体の台帳::空();
    let Ok(質量) = キログラム::生成する(2.0) else {
        panic!();
    };
    let Ok(特性) = 質量特性::一様な箱(質量, メートル::生成する(1.0), メートル::生成する(1.0), メートル::生成する(1.0))
    else {
        panic!();
    };
    let Ok(id) = 台帳.登録する(テスト配置(), 特性, 運動種別::動的で始める(運動状態::静止())) else {
        panic!();
    };

    let 基本秒 = 秒::生成する(基本刻みの秒);
    let Ok(基本幅) = 刻み幅::生成する(基本秒) else {
        panic!();
    };
    let Ok(n) = 細分数::生成する(細分の数) else {
        panic!();
    };
    let Ok(n_u16) = u16::try_from(細分の数) else {
        panic!();
    };
    let n_f32 = f32::from(n_u16);
    let 細分秒 = 秒::生成する(基本刻みの秒 / n_f32);
    let Ok(細分幅) = 刻み幅::生成する(細分秒) else {
        panic!();
    };
    let Ok(品質) = 接触を解く品質の設定::生成する(2, 4) else {
        panic!();
    };
    let mut 組み立て = 混合則の組み立て::生成する();
    let mat = 材質の識別子::生成する(0);
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
    let 予測器 = 細分の予測器::生成する(細分幅, 一様な加速度::零(), ジャイロ項の扱い::陰的に一段解く);
    let 速度段階 = 接触の速度段階::生成する(細分幅, blitz_math::メートル毎秒::生成する(0.01));
    let 空間索引 = 動く形の空間索引::ゆとりの幅から生成する(箱に持たせるゆとりの幅::人型と家具の大きさに見合う既定の幅());

    let mut 工程 = 剛体の接触の一刻みの工程::生成する(
        接触の品質と時間方針::生成する(基本幅, n, 細分幅, 品質, 混合則),
        接触の空間と世界::生成する(空間索引),
        接触履歴の保持::見込みの接触点の数で生成する(8),
        接触の解法ソルバー::生成する(予測器, 速度段階),
    );
    let Ok(半分) = 直方体の軸ごとの半分の長さ::生成する([メートル::生成する(0.5); 3]) else {
        panic!();
    };
    let Ok(()) = 工程.剛体を登録する(id, &テスト配置(), 半分, mat) else {
        panic!();
    };
    (工程, 台帳, id)
}
