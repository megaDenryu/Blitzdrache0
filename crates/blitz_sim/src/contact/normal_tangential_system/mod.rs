//! 接触点集合1つの法線と接線を同じ対称行列へ入れて1回で解く連立(判断12・判断13)。粘着の候補はこの連立の解である。
//! 法線の行は非貫通(判断12)がその反復に解くべき右辺 −C_n − α̃ λ_n を持ち、接線の行は錨の接線変位を零へ戻す
//! 右辺 −C_t を持つ(C_t が解けたと見なす許容差の内側の点の行は積まない。判断13の2026-09-12の改訂)。求まるのは、貫通の解消と接線の補正と、接線の力が作る回転を打ち消す法線の荷重の
//! 移し替えの3つが同時に釣り合った1つの解である。
//! 法線を先に確定してから右辺が零の混合連立を足す2段の形にすると、前段の非貫通が作った接線変位を後段がもう一度
//! 払うことになる。前段の有限回転を後段の線形化が完全には取り消せず、錨の接線変位が反復のたびに作り直されて
//! 接線の乗数が積み増され、傾きの正接が静止摩擦係数より小さい坂でも円錐を超える。
//! 有効集合は、有効な点の数が多い階層から順に部分集合を走査し、相補条件を満たす候補が見つかった階層で打ち切って
//! 選ぶ(`subset_search`)。決着の第1の鍵が点の数であるため、結果は全走査と1ビットも変わらない。全走査は試験専用の
//! 正典(`full_scan_reference`)として残し、一致を反証が固定する。枢軸を順に動かす形は、外す条件と入れ直す条件が
//! 同じ点について同時に成り立つ材料が坂の場面に実在するため終了しない。
//! 行が2つの参加者への符号付きの勾配を持つのは、法線と接線の交差の項の符号が参加者ごとの勾配で決まるためである。
//! この連立と同じ式を倍精度で持つ試験専用の参照計算を`double_reference`が持つ(Issue #59の数値契約の診断)。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

mod acceptance;
mod active_set;
mod candidate;
mod complementarity;
#[cfg(test)]
mod decade_histogram;
#[cfg(test)]
mod direction_generalized_correction;
#[cfg(test)]
mod double_reference;
#[cfg(test)]
mod eigen_distribution;
#[cfg(test)]
mod eigen_relative_spread;
mod full_scan_reference;
#[cfg(test)]
mod increment_uncertainty_spread;
#[cfg(test)]
mod order_tests;
mod participant_correction;
#[cfg(test)]
mod pseudo_random_fixture;
#[cfg(test)]
mod random_system_fixture;
mod reduced_system;
mod reduced_system_point_mapping;
mod reduced_system_pseudo_inverse_inputs;
mod reduced_system_residual;
#[cfg(test)]
mod reduced_system_tolerance_scale;
mod row_order;
#[cfg(test)]
mod single_point_fixture;
#[cfg(test)]
mod single_point_tests;
mod solution;
#[cfg(test)]
mod solution_breakdown;
mod solve_count;
mod solve_outcome;
mod solved_quality;
mod subset_search;
#[cfg(test)]
mod subset_search_agreement_tests;
mod system;
#[cfg(test)]
mod system_fixture;
mod tangential_row;
#[cfg(test)]
mod tolerance_application;
#[cfg(test)]
mod tolerance_candidate;
#[cfg(test)]
mod tolerance_reach;
#[cfg(test)]
mod tolerance_scale;
#[cfg(test)]
mod tolerance_tests;
#[cfg(test)]
mod translation_tests;

#[cfg(test)]
pub(in crate::contact) use acceptance::受理の契約を破った理由;
pub use acceptance::混合連立の受理の結果;
#[cfg(test)]
pub(in crate::contact) use decade_histogram::桁ごとの度数;
#[cfg(test)]
pub(in crate::contact) use direction_generalized_correction::剛体の一般化自由度へ入る補正の大きさ;
#[cfg(test)]
pub(in crate::contact) use double_reference::{
    倍精度の円錐の判定, 倍精度の参照の結末, 倍精度の解の内訳, 参照計算の許容差の由来
};
#[cfg(test)]
pub(in crate::contact) use eigen_distribution::偽の固有値の下限に対する固有の向きの分布;
#[cfg(test)]
pub(in crate::contact) use eigen_relative_spread::固有値を最大で割った相対値の広がり;
#[cfg(test)]
pub(in crate::contact) use increment_uncertainty_spread::増分と不確かさの率の広がり;
#[cfg(test)]
pub(in crate::contact) use solution_breakdown::{単精度の固有の向きの内訳, 単精度の解の内訳};
pub use system::接触点集合の法線と接線の連立;
pub use tangential_row::錨の接線変位を零へ戻す一行;
#[cfg(test)]
pub(in crate::contact) use tolerance_application::{接線の行への許容差の当て方, 試験の許容差の当て方};
#[cfg(test)]
pub(in crate::contact) use tolerance_candidate::{
    偽の固有値の下限を導く取り決め, 擬似逆が右辺の向きを捨てる判定の当て方, 解けたと見なす許容差を導く座標の由来,
};
#[cfg(test)]
pub(in crate::contact) use tolerance_reach::{受理の倍率の届く先, 行を積む閾値の倍率};
#[cfg(test)]
pub(in crate::contact) use tolerance_scale::解けたと見なす許容差の倍率;

// 解と結末を外へ出す口。本番の細分の工程(判断19)が接触点集合ごとの粘着の候補を解くために読む。
pub use solution::接触点集合の法線と接線の同時解;
pub use solve_count::部分集合を解いた回数;
pub use solve_outcome::相補条件を満たす有効集合を探した結末;
