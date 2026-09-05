//! 接触生成の余白の導出(判断11)。剛体の対ごと・細分ごとに、その細分のあいだに2つの形の角が動きうる長さを求める。
//! 形1つぶんの運動を持つ型(`motion`)と、2つの形からその長さを導く工程(`derivation`)に分けている。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断11: 接触拘束は接触点集合から細分ごとに生成する一時のバッチであり、参加者の組ごとに別のバッチを持ち、座標系の写しは剛体の側が行う」

mod derivation;
#[cfg(test)]
mod derivation_tests;
mod motion;

pub(in crate::contact) use derivation::細分の運動から接触生成の余白を導く;
pub(in crate::contact) use motion::接触生成の余白を導く形の運動;
