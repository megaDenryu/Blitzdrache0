//! 大気の計算に現れる量そのものを所有するモジュール。担当するのは、単位と値域を型で固定した値オブジェクトの定義である。
//!
//! ここにある型はどれも「検証済みの数を1つ(または3つ)持つ」だけであり、大気の構造も積分の手順も知らない。
//! 量と、量を使う計算を同じ場所に置かないのは、量の不変条件(値域・有限性・単位)が計算の都合で緩められることを
//! 防ぐためである。生の`f32`をドメインAPIへ出さない規律の受け皿がこのモジュールである
//! (参照: CLAUDE.md「型安全性」の数学DDD)。

pub(in crate::atmosphere) mod albedo_rgb;
pub(in crate::atmosphere) mod asymmetry;
pub(in crate::atmosphere) mod attenuating_component;
pub(in crate::atmosphere) mod azimuth_cosine;
pub(in crate::atmosphere) mod extinction_rgb;
pub(in crate::atmosphere) mod multiscatter_rgb;
pub(in crate::atmosphere) mod normalized_density;
pub(in crate::atmosphere) mod phase_value;
pub(in crate::atmosphere) mod sample_count;
pub(in crate::atmosphere) mod scattering_cosine;
pub(in crate::atmosphere) mod skyview_rgb;
pub(in crate::atmosphere) mod transmittance_rgb;
pub(in crate::atmosphere) mod zenith_cosine;
