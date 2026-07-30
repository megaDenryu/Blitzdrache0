//! 多重散乱の積分が球面全体を覆うために使う、決まった向きの集合。
//!
//! 乱数でなく縦横8等分の格子から作るのは、同じ入力から常に同じベイク済み画像が出ることを契約にしているためである
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「更新の決定性」)。方位角を等間隔に、天頂角の余弦を
//! 等間隔に刻むことで、64本の向きが球面上でほぼ等しい立体角を受け持つ。
//! 参照: Sebastien Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020)の
//! `NewMultiScattCS`の8x8の向き。
//!
//! 注意: この並びは`shaders/atmosphere_multiscatter.slang`の写しを持つ。ここが正本である。

use std::f64::consts::PI;

/// 一辺の分割数。方位角と天頂角の余弦をそれぞれこの数だけ刻む。
pub(in crate::atmosphere) const 一辺の分割数: u32 = 8;
/// 球面を覆う向きの本数。
pub(in crate::atmosphere) const 方向数: u32 = 一辺の分割数 * 一辺の分割数;

/// 添字から向きの単位ベクトルを求める。天頂は+Zであり、多重散乱の計算はこの座標系で閉じる。
pub(in crate::atmosphere) fn 球面の向き(添字: u32) -> [f64; 3] {
    let 方位側 = 0.5 + f64::from(添字 / 一辺の分割数);
    let 天頂側 = 0.5 + f64::from(添字 % 一辺の分割数);
    let 分割数 = f64::from(一辺の分割数);
    let 方位角 = 2.0 * PI * (方位側 / 分割数);
    let 天頂角 = (1.0 - 2.0 * (天頂側 / 分割数)).clamp(-1.0, 1.0).acos();
    let 横成分 = 天頂角.sin();
    [方位角.cos() * 横成分, 方位角.sin() * 横成分, 天頂角.cos()]
}

/// 1本の向きが受け持つ立体角。球面全体の4πを本数で割った値である。
pub(in crate::atmosphere) fn 一本あたりの立体角() -> f64 {
    4.0 * PI / f64::from(方向数)
}
