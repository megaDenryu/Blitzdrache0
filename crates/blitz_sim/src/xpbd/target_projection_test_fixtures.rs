//! 目標拘束の正典式の検査が共有する材料: 座標の点、コンプライアンスと刻み幅から導いた係数、1回の射影で位置と乗数を進める補助。

use blitz_math::{メートル, ワールド, 位置, 秒, 逆キログラム};

use super::compliance::コンプライアンス;
use super::distance_constraint_participant::距離拘束の参加点;
use super::lagrange_multiplier::ラグランジュ乗数;
use super::target_canonical_projection::目標拘束の一刻みの係数;
use super::target_constraint_parameters::目標拘束の引数;
use super::target_projection_result::目標拘束の一回の射影の結果;
use super::time_step_width::刻み幅;

pub(super) fn 点(x: f32, y: f32) -> 位置<ワールド> {
    位置::生成する(メートル::生成する(x), メートル::生成する(y), メートル::生成する(0.0))
}

pub(super) fn 逆質量(値: f32) -> 逆キログラム {
    逆キログラム::生成する(値).unwrap_or_else(|誤り| panic!("{誤り}"))
}

pub(super) fn 係数(コンプライアンス: f32, 刻み幅の秒: f32) -> 目標拘束の一刻みの係数 {
    let 引数 = 目標拘束の引数::生成する(コンプライアンス::生成する(コンプライアンス).unwrap_or_else(|誤り| panic!("{誤り}")));
    let 刻み幅 = 刻み幅::生成する(秒::生成する(刻み幅の秒)).unwrap_or_else(|誤り| panic!("{誤り}"));
    引数.刻み幅で解く係数を導く(刻み幅).unwrap_or_else(|誤り| panic!("{誤り}"))
}

/// 1回の射影を当てて、補正後の位置と更新後の乗数を返す。退化の枝は検査の失敗である。
pub(super) fn 一回射影して進める(
    係数: &目標拘束の一刻みの係数,
    粒子: 位置<ワールド>,
    逆質量の値: f32,
    目標: 位置<ワールド>,
    乗数: ラグランジュ乗数,
) -> (位置<ワールド>, ラグランジュ乗数) {
    let 参加点 = 距離拘束の参加点::生成する(粒子, 逆質量(逆質量の値));
    match 係数.一回射影する(参加点, 目標, 乗数) {
        目標拘束の一回の射影の結果::補正した {
            補正,
            更新後のラグランジュ乗数,
        } => (粒子.変位を足す(補正), 更新後のラグランジュ乗数),
        退化 => panic!("退化の枝に入った: {退化:?}"),
    }
}
