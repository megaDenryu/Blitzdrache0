//! 多重散乱のベイク済み画像の1テクセルの値を求める工程。受け取るのは媒体・透過率表・テクセルの添字、返すのはそのテクセルの多重散乱である。
//!
//! これがベイク済み画像の内容の正本であり、`shaders/atmosphere_multiscatter.slang`はこの式の写しである。
//!
//! 無限回の散乱を、64本の向きで測った「1回の散乱を経ても残る割合」を公比とする等比級数の和として閉じる。
//! 級数を閉じる操作と公比の上限は`multiscatter_series`が持つ。
//! 参照: Sebastien Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020)の
//! `NewMultiScattCS`(`MULTI_SCATTERING_POWER_SERIE`の腕)。

use crate::atmosphere::geometry::sphere_directions::{一本あたりの立体角, 方向数, 球面の向き};
use crate::atmosphere::integration::multiscatter_march::経路を積分する;
use crate::atmosphere::integration::multiscatter_series::等比級数を閉じる;
use crate::atmosphere::mapping::multiscatter_mapping::多重散乱のベイク済み画像の条件を求める;
use crate::atmosphere::table::transmittance_table::透過率表;
use crate::atmosphere::{多重散乱RGB, 大気のベイク済み画像の解像度, 大気散乱媒体, 大気数学エラー};

pub fn 多重散乱のベイク済み画像のテクセル値(
    媒体: &大気散乱媒体,
    透過率表: &透過率表,
    解像度: 大気のベイク済み画像の解像度,
    横: u32,
    縦: u32,
) -> Result<多重散乱RGB, 大気数学エラー> {
    let 条件 = 多重散乱のベイク済み画像の条件を求める(媒体.消散(), 解像度, 横, 縦)?;
    let 太陽天頂余弦 = f64::from(条件.太陽天頂余弦.値());
    // 天頂を+Zに取る。方位角は結果に効かないため、太陽をXZ平面へ置く。
    let 太陽 = [(1.0 - 太陽天頂余弦 * 太陽天頂余弦).max(0.0).sqrt(), 0.0, 太陽天頂余弦];
    let 観測位置 = [0.0, 0.0, f64::from(条件.半径.値())];
    let mut 放射輝度 = [0.0_f64; 3];
    let mut 残存率 = [0.0_f64; 3];
    for 添字 in 0..方向数 {
        let 積分 = 経路を積分する(媒体, 透過率表, 観測位置, 球面の向き(添字), 太陽);
        for 成分 in 0..3 {
            放射輝度[成分] += 積分.放射輝度[成分];
            残存率[成分] += 積分.残存率[成分];
        }
    }
    合成する(放射輝度, 残存率)
}

/// ベイク済み画像全体を走査して各テクセルの多重散乱を求める。GPUが焼いた結果と突き合わせる期待値がこれである。
/// テクセルの並びは横方向が先で、GPUの画像を行優先で読み戻した並びと一致する。
pub fn 多重散乱のベイク済み画像を焼く(
    媒体: &大気散乱媒体,
    透過率表: &透過率表,
    解像度: 大気のベイク済み画像の解像度,
) -> Result<Vec<多重散乱RGB>, 大気数学エラー> {
    let 一辺 = 解像度.多重散乱の一辺();
    let mut 一覧 = Vec::with_capacity(解像度.多重散乱のテクセル数());
    for 縦 in 0..一辺 {
        for 横 in 0..一辺 {
            一覧.push(多重散乱のベイク済み画像のテクセル値(媒体, 透過率表, 解像度, 横, 縦)?);
        }
    }
    Ok(一覧)
}

/// 向きごとの積分を球面全体の平均へ直し、等比級数の和で無限次散乱まで閉じる。
/// 立体角の重みは全方向で等しいため、本数で割る代わりに1本あたりの立体角を掛けてから等方位相で割り戻す形になる。
fn 合成する(放射輝度: [f64; 3], 残存率: [f64; 3]) -> Result<多重散乱RGB, 大気数学エラー> {
    let 立体角 = 一本あたりの立体角();
    let 等方位相 = 1.0 / (4.0 * std::f64::consts::PI);
    let mut 結果 = [0.0_f32; 3];
    for 添字 in 0..3 {
        let 一次散乱 = 放射輝度[添字] * 立体角 * 等方位相;
        let 公比 = 残存率[添字] * 立体角 * 等方位相;
        結果[添字] = 等比級数を閉じる(一次散乱, 公比)?;
    }
    多重散乱RGB::生成する(結果[0], 結果[1], 結果[2])
}
