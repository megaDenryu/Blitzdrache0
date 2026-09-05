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
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

#![cfg(test)]
#![allow(clippy::unwrap_used)]

mod active_set;
mod complementarity;
mod cone_judgement;
mod double_reference_tests;
mod gradient;
mod inverse_inertia;
mod jacobi;
mod manifold_system;
mod normal_row;
mod participant_point;
mod placement;
mod pseudo_inverse;
mod reduced_system;
mod reference_solution;
mod residual;
mod row_order;
mod single_precision_bridge;
mod solution;
mod subset_search;
mod tangential_row;
mod vector;

pub(in crate::contact) use cone_judgement::倍精度の円錐の判定;
pub(in crate::contact) use reference_solution::{倍精度の参照の結末, 倍精度の参照の解};
