//! 正接0.50の坂で接触点集合が初めてクーロン円錐を超える細分の内訳を記録する計器(判断13の原因の分離)。
//! 担当するのは、鉛直軸まわりの回し0度・30度・45度のそれぞれで、円錐を超えた最初の細分を探し、その1本前まで場面を
//! 進めてから、位置の反復の3つの地点(非貫通の前・非貫通の後・接線解の後)の量を綴ることである。
//! 合否は判定しない。判定する検査は`slope_tests`が持つ。実行は
//! `cargo test -p blitz_sim --release 円錐を超える細分の内訳を記録する -- --ignored --nocapture` である。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

#![cfg(test)]
#![allow(clippy::unwrap_used)]

mod instrumented_substep;
mod manifold_observation;
mod point_observation;
mod record_line;
mod tangential_system_observation;

use std::f32::consts::PI;

use super::super::friction_coefficient::摩擦係数;
use super::slope_fixture::坂の場面を組む;
use super::slope_geometry::坂の場面の条件;
use super::static_friction_method::場面の静止摩擦の解き方;

const 記録する傾きの正接: f32 = 0.50;
pub(super) const 静止摩擦係数の値: f32 = 0.6;
const 円錐を超える細分を探す上限: usize = 4800;

fn 記録する条件(鉛直軸まわりの回し: f32) -> 坂の場面の条件 {
    坂の場面の条件 {
        傾きの正接: 記録する傾きの正接,
        鉛直軸まわりの回し,
        静止摩擦係数: 摩擦係数::生成する(静止摩擦係数の値).unwrap(),
        動摩擦係数: 摩擦係数::生成する(0.2).unwrap(),
        下り向きの初速: 0.0,
        静止摩擦の解き方: 場面の静止摩擦の解き方::接触点集合の接線を同時に解く,
    }
}

// 接触点集合が初めて滑走中と印された細分の番号(0から数える)。見つからなければ無しである。
fn 円錐を初めて超えた細分の番号(条件: &坂の場面の条件) -> Option<usize> {
    let mut 場面 = 坂の場面を組む(条件);
    for 番号 in 0..円錐を超える細分を探す上限 {
        場面.一細分進める();
        if 場面.直前の細分の観測.滑走中の接触点の数 > 0 {
            return Some(番号);
        }
    }
    None
}

#[test]
#[ignore = "計器であり合否を判定しない。実行は --ignored --nocapture を付ける"]
fn 円錐を超える細分の内訳を記録する() {
    for (見出し, 回し) in [("0度", 0.0), ("30度", PI / 6.0), ("45度", PI / 4.0)] {
        let 条件 = 記録する条件(回し);
        let Some(番号) = 円錐を初めて超えた細分の番号(&条件) else {
            println!("[{見出し}] {円錐を超える細分を探す上限}細分のあいだ円錐を超えなかった");
            continue;
        };
        println!("[{見出し}] 円錐を初めて超えた細分は{番号}本目である");
        let mut 場面 = 坂の場面を組む(&条件);
        場面.細分を進める(番号);
        場面.次の細分の反復を記録する(見出し);
    }
}
