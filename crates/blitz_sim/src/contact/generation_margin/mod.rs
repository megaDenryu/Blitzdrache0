//! 接触生成の余白と、参照軸の選択で外積の軸へ切り替える優位幅の導出(判断11)。剛体の対ごと・細分ごとに、
//! その細分のあいだに2つの形の角が動きうる長さと、静穏と見なす運動だけで角が動きうる長さを対で求める。
//! 形1つぶんの運動を持つ型(`motion`)と、2つの形からその長さを導く工程(`derivation`)に分けている。
//! 導出の反証は、余白が下回ってはならない長さを固定する側(`derivation_lower_limit_tests`)と、速度が余白を
//! 決めるときに余白が運動をそのまま写すことを固定する側(`derivation_motion_tests`)に分かれ、両方が同じ形と
//! 同じ刻み幅で測るための材料を `derivation_test_fixtures` が持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断11: 接触拘束は接触点集合から細分ごとに生成する一時のバッチであり、参加者の組ごとに別のバッチを持ち、座標系の写しは剛体の側が行う」

mod derivation;
#[cfg(test)]
mod derivation_lower_limit_tests;
#[cfg(test)]
mod derivation_motion_tests;
#[cfg(test)]
mod derivation_test_fixtures;
mod motion;

pub(in crate::contact) use derivation::{
    細分の運動から接触生成の余白を導く, 細分の静穏の運動から外積の軸へ切り替える優位幅を導く
};
pub(in crate::contact) use motion::接触生成の余白を導く形の運動;
