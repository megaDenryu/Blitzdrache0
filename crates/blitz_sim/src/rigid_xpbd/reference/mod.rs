//! 剛体の参照計算: 剛体の台帳と点自由度と4つの拘束のバッチ(剛体と世界の点・剛体と剛体の接続・剛体と剛体のねじり・剛体と点の距離)を持ち、
//! 細分の工程(作用の取り出し→衝撃の適用→予測→乗数の零化→反復→速度の再構成)をCPUで回す正典である(判断8・9・10・19の部分列)。
//! 接触・島・休止(Issue #40)はまだ無く、反復の順序はバッチの種類の順(目標→接続→ねじり→点との距離)の逐次である。
//! 一刻みの工程は`step`が持つ。

mod connection_batch;
mod constraint_lists;
mod point_distance_batch;
mod point_state;
mod step;
mod substep_body;
mod target_batch;
mod twist_batch;

use blitz_math::{ワールド, 位置};

pub use connection_batch::添字付き接続拘束;
pub use constraint_lists::剛体の拘束の一覧;
pub use point_distance_batch::添字付き点と剛体の距離拘束;
pub use target_batch::添字付き剛体の目標拘束;
pub use twist_batch::添字付きねじり拘束;

use super::error::剛体の参照計算エラー;
use super::predictor::細分の予測器;
use super::rotational_lagrange_multiplier::回転のラグランジュ乗数;
use crate::constraint_graph::点自由度の初期状態;
use crate::rigid_body::剛体の台帳;
use crate::xpbd::ラグランジュ乗数;

pub struct 剛体の参照計算 {
    台帳: 剛体の台帳,
    点一覧: Vec<point_state::点の細分の状態>,
    剛体の目標拘束: target_batch::剛体の目標拘束のバッチ,
    接続拘束: connection_batch::接続拘束のバッチ,
    ねじり拘束: twist_batch::ねじり拘束のバッチ,
    点と剛体の距離拘束: point_distance_batch::点と剛体の距離拘束のバッチ,
    予測器: 細分の予測器,
    反復回数: u32,
}

impl 剛体の参照計算 {
    /// 予測器の刻み幅(細分1本の長さ)で全拘束の一刻みの係数を1度だけ導く。
    pub fn 生成する(
        台帳: 剛体の台帳,
        点一覧: &[点自由度の初期状態],
        拘束: 剛体の拘束の一覧,
        予測器: 細分の予測器,
        反復回数: u32,
    ) -> Result<Self, 剛体の参照計算エラー> {
        let 刻み幅 = 予測器.刻み幅();
        Ok(Self {
            台帳,
            点一覧: 点一覧.iter().map(point_state::点の細分の状態::初期状態から生成する).collect(),
            剛体の目標拘束: target_batch::剛体の目標拘束のバッチ::生成する(拘束.剛体の目標拘束, 刻み幅)?,
            接続拘束: connection_batch::接続拘束のバッチ::生成する(拘束.接続拘束, 刻み幅)?,
            ねじり拘束: twist_batch::ねじり拘束のバッチ::生成する(拘束.ねじり拘束, 刻み幅)?,
            点と剛体の距離拘束: point_distance_batch::点と剛体の距離拘束のバッチ::生成する(
                拘束.点と剛体の距離拘束,
                点一覧.len(),
                刻み幅,
            )?,
            予測器,
            反復回数,
        })
    }

    pub fn 台帳(&self) -> &剛体の台帳 {
        &self.台帳
    }

    /// ゲーム側の作用の投入に当たる口。検査が力と衝撃を剛体へ与えるために台帳を借りる。
    pub fn 台帳を書き換える(&mut self) -> &mut 剛体の台帳 {
        &mut self.台帳
    }

    pub fn 点の位置一覧(&self) -> Vec<位置<ワールド>> {
        self.点一覧.iter().map(point_state::点の細分の状態::位置).collect()
    }

    pub fn 剛体の目標拘束の乗数一覧(&self) -> &[ラグランジュ乗数] {
        self.剛体の目標拘束.乗数一覧()
    }

    pub fn 接続拘束の乗数一覧(&self) -> &[ラグランジュ乗数] {
        self.接続拘束.乗数一覧()
    }

    pub fn ねじり拘束の乗数一覧(&self) -> &[回転のラグランジュ乗数] {
        self.ねじり拘束.乗数一覧()
    }

    pub fn 点と剛体の距離拘束の乗数一覧(&self) -> &[ラグランジュ乗数] {
        self.点と剛体の距離拘束.乗数一覧()
    }
}
