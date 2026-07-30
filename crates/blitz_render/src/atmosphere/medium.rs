//! 大気そのものの構造を所有するモジュール。担当するのは、惑星と大気の半径・成分ごとの密度分布と散乱係数を持つ媒体の定義と、
//! 媒体の中の1点における密度と散乱係数の評価である。
//!
//! 媒体と点の評価を同じ場所に置くのは、点の評価が媒体の私的な構造(層の並び・成分の順序)を読むためである。
//! ここを分けると、成分の並びという1つの不変条件を2つのモジュールが知ることになる。
//! 積分の手順とベイク済み画像の写像はこのモジュールを読むだけであり、逆向きの依存は無い。

pub(in crate::atmosphere) mod density;
pub(in crate::atmosphere) mod density_layer;
pub(in crate::atmosphere) mod density_profile;
pub(in crate::atmosphere) mod extinction_medium;
pub(in crate::atmosphere) mod medium_sample;
pub(in crate::atmosphere) mod observation_point;
pub(in crate::atmosphere) mod scattering_medium;
