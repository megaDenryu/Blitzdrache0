//! CPUの参照計算を、GPUの方式と同じ更新の順序・同じ加速度の予定・同じ反復回数で回す。
//! グラフ彩色は色順の逐次、原子加算と二段階は緩和係数つきの同時であり、対応はここが1箇所で決める。

use blitz_render::xpbd_solver_bench_probe::XPBD並列方式;
use blitz_sim::constraint_graph::{反復の更新の順序, 拘束グラフの参照計算};
use blitz_sim::刻み幅;

use super::acceleration_schedule;
use super::fixture::計測の題材;
use crate::error::起動エラー;

/// 方式がGPUで実現する更新の順序。参照計算はこれと同じ順序で回す。
pub(super) fn 方式の更新の順序(方式: XPBD並列方式) -> 反復の更新の順序 {
    match 方式 {
        XPBD並列方式::グラフ彩色 => 反復の更新の順序::逐次,
        XPBD並列方式::原子加算 | XPBD並列方式::二段階 => 反復の更新の順序::同時,
    }
}

pub(super) fn 参照計算を回す(
    題材: &計測の題材,
    方式: XPBD並列方式,
    刻み数: u32,
    反復回数: u32,
    刻み幅: 刻み幅,
) -> Result<拘束グラフの参照計算, 起動エラー> {
    let mut 参照 = 拘束グラフの参照計算::生成する(&題材.グラフ, 方式の更新の順序(方式), 刻み幅)?;
    for 刻み in 0..刻み数 {
        参照.一刻み進める(acceleration_schedule::刻みの加速度(刻み), 反復回数);
    }
    Ok(参照)
}
