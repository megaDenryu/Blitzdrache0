//! 接触の粘着の候補を倍精度で解く参照計算(Issue #59の数値契約の診断。試験と計器の専用)。
//! 単精度の本番の経路(`normal_tangential_system`・`symmetric_system`・`static_friction`)が解く連立と同じ式を
//! 倍精度で持ち、同じ入力(細分の予測が進めた配置・錨・接触点集合・法線の乗数)から出発して同じ順で解く。
//! 本番の型を倍精度へ変えず写しを置くのは、単精度の経路の結果を1ビットも変えないためである。参照計算の役目は
//! 「同じ入力に対して精度だけを上げたら解法が何を返すか」を答えることであり、本番の解法を置き換えることではない。
//!
//! この参照計算が測る範囲と測らない範囲を分けて書く。測るのは、行の組み立て(世界位置の差・勾配・交差の有効
//! 逆質量・右辺)から先の演算の精度と、解けたと見なす許容差という取り決めの効き方である。測らないのは、
//! 入力として受け取る配置そのものの座標の量子化である。座標の量子化は、同じ相対配置を世界内で平行移動する
//! 診断が別に測る。この線引きにより、Issue #59が並べた連鎖のうち「擬似逆の階数落ちと許容差 → 残差」の環を
//! 単独で見られる。
//!
//! 本番の型を精度で汎化して1つにする案は採らなかった。汎化すると`blitz_math`の単位と座標系の型(メートル・
//! 位置・方向・逆慣性テンソル)まで精度の型引数を持つことになり、この診断1件のために `blitz_sim` と
//! `blitz_math` の公開面が広く変わる。写しを置く費用はこのモジュールに閉じ、写し違いは単純な材料で単精度と
//! 倍精度が丸めの範囲で一致することを固定する試験(`double_reference_tests`)が守る。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

#![cfg(test)]
#![allow(clippy::unwrap_used)]

mod active_set;
mod complementarity;
mod cone_judgement;
mod constraint_violation_tests;
mod double_reference_fixture;
mod gradient;
mod inverse_inertia;
mod jacobi;
mod manifold_system;
mod multiplier_tests;
mod normal_row;
mod orientation;
mod participant;
mod participant_point;
mod placement;
mod pseudo_inverse;
mod reduced_system;
mod reference_solution;
mod released_point_effect;
mod residual;
mod row_order;
mod single_precision_bridge;
mod solution;
mod solution_breakdown;
mod subset_search;
mod tangential_row;
mod tolerance_origin;
mod vector;
mod widened_normal_state;

pub(in crate::contact) use cone_judgement::倍精度の円錐の判定;
pub(in crate::contact) use reference_solution::倍精度の参照の結末;
pub(in crate::contact) use solution_breakdown::倍精度の解の内訳;
pub(in crate::contact) use tolerance_origin::参照計算の許容差の由来;
