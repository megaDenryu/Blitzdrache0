//! 自動露出: 明るさの圧縮の前のHDRカラーの輝度ヒストグラムから、露出の補正段を決める純関数の層。
//! 目盛・集計・導出・時間適応と語彙の数が多いため、クレートの平坦な再エクスポートへ混ぜずモジュールごと公開する。
//!
//! 放射輝度の問い合わせの外側にある(照明の供給ではなく画の完成の段である)。この層はGPUもフレームも実時間も
//! 知らず、同じ入力からは常に同じ出力を返す。集計・導出・時間適応を別の純関数に分けるのは、GPUのslang写しと
//! 段ごとに突き合わせるためである(`参照: _doc/設計/放射輝度問い合わせ階層.md`「IIb(自動露出)の設計」)。
//!
//! この層が外へ出す生の単精度は`境界の線形輝度`だけである。GPUの写しへ同じ列を渡すために、
//! コンポジションルートが読んでレンダラーへ運ぶ必要があるためである。目盛の定数と相対輝度の重みは
//! 外へ出さない(重みはslangの写しが自分の定数として持ち、値の一致は`cargo xtask conform`が見る)。

mod adaptation;
mod adaptation_speed;
mod adaptation_speed_pair;
mod adaptation_state;
mod aggregation;
mod clamp;
mod derivation;
mod derivation_result;
mod error;
mod exposure_method;
mod final_compensation;
mod histogram;
mod histogram_boundary;
mod histogram_scale;
mod log_luminance;
mod luminance;
mod middle_gray;
mod pixel_count;
mod pixel_ratio;
mod positive_luminance;

#[cfg(test)]
mod adaptation_tests;
#[cfg(test)]
mod derivation_tests;
#[cfg(test)]
mod final_compensation_tests;
#[cfg(test)]
mod histogram_scale_tests;
#[cfg(test)]
mod histogram_tests;
#[cfg(test)]
mod quantity_tests;
#[cfg(test)]
mod value_object_tests;

pub use adaptation::次の適応状態を求める;
pub use adaptation_speed::補正段毎秒;
pub use adaptation_speed_pair::露出の適応速度;
pub use adaptation_state::露出適応状態;
pub use aggregation::{画素列から集計する, 輝度列から集計する};
pub use clamp::自動補正段の上下限;
pub use derivation::目標補正段を導く;
pub use derivation_result::露出の導出結果;
pub use error::自動露出エラー;
pub use exposure_method::露出方式;
pub use final_compensation::最終補正段を合成する;
pub use histogram::輝度ヒストグラム;
pub use histogram_boundary::{ビンの添字を求める, 境界の線形輝度};
pub use histogram_scale::{ビンの添字, ビン数, 全ビンの添字, 最後のビンの添字};
pub use log_luminance::対数輝度;
pub use luminance::{画素を区分する, 輝度の区分, 輝度を区分する};
pub use middle_gray::目標中間灰;
pub use pixel_count::画素件数;
pub use pixel_ratio::画素の割合;
pub use positive_luminance::正の相対輝度;
