//! Bruneton参照との一致検査の共通の器。読み込み・誤差の測り方・焼いた条件の照合をここに置き、
//! 量ごとの比較は2つの子モジュールが持つ。
//!
//! 比較するのはLUTのテクセル番号でなく物理量である。参照実装は同じ台形則・同じ区間数500を使うため、
//! 一致検査が見るのは離散化の違いでなく式そのものの違いである。離散化の誤差は閉形式との比較が別に見る。

use blitz_math::メートル;

use super::super::{大気内観測点, 天頂余弦, 消散媒体};
use super::reference_parse::読み解く;
use super::reference_record::参照レコード;
use crate::atmosphere::narrowing::実数へ狭める;

mod geometry_tests;
mod integral_tests;
mod scalar_field_tests;

pub(super) const 参照本文: &str = include_str!("../../../data/bruneton_reference.txt");

/// 参照実装が使う区間数。焼いたデータのパラメータ行と一致することを検査が確かめる。
pub(super) const 参照の区間数: u32 = 500;

/// 量ごとの最大相対誤差の実測は、密度5.26e-8・地平線3.69e-8・上端距離5.47e-8・地表距離4.99e-8・
/// 光学距離5.70e-8・透過率5.04e-8・位相3.42e-8である。参照実装と同じ式・同じ台形則・同じ区間数500を使うため、
/// f64のまま比べれば丸め以外の差は出ない。残る差はこちらが結果をf32へ狭めることだけであり、
/// その半ulpにあたる相対6.0e-8を下回る閾値は実装が正しくても再現しない。判定はその2倍の1.2e-7を上限とする。
/// この上限を超えるのは、式か刻み方が参照と食い違ったときだけである。
pub(super) const 相対誤差の上限: f64 = 1.2e-7;

pub(super) fn 参照レコード一覧() -> Vec<参照レコード> {
    読み解く(参照本文)
}

/// 期待値が0のときは相対誤差を定義できないため、絶対誤差をそのまま返す。
pub(super) fn 相対誤差(実測: f32, 期待: f64) -> f64 {
    let 実測 = f64::from(実測);
    if 期待 == 0.0 {
        return 実測.abs();
    }
    (実測 - 期待).abs() / 期待.abs()
}

/// f32が正規化数として表せる最小の正値。大気上端でのミーの光学距離のように、参照が1e-46級の値を返す場合、
/// f32へ狭めた時点で0か非正規化数へ潰れるため相対誤差では判定できない。この域は「双方が下限を下回ること」で見る。
/// この扱いになる比較は焼いた条件では26件であり、いずれも高い観測点でのミーと吸収の光学距離である。
fn 表現下限() -> f64 {
    f64::from(f32::MIN_POSITIVE)
}

/// 参照との一致判定。表現できない大きさの期待値と、比較できる大きさの期待値で判定の仕方を分ける。
pub(super) fn 一致するか(実測: f32, 期待: f64) -> bool {
    if 期待.abs() < 表現下限() {
        return f64::from(実測).abs() < 表現下限();
    }
    相対誤差(実測, 期待) < 相対誤差の上限
}

/// 焼いたf64の引数を、比較相手の実装が持つf32へ狭めて視線を組み立てる。
/// 焼く側が引数をf32で表せる値だけに限っているため、この縮小で情報は落ちない。
pub(super) fn 視線(媒体: &消散媒体, 半径: f64, 余弦: f64) -> (大気内観測点<'_>, 天頂余弦) {
    let 観測点 = 大気内観測点::生成する(媒体, メートル::生成する(実数へ狭める(半径))).unwrap();
    (観測点, 天頂余弦::生成する(実数へ狭める(余弦)).unwrap())
}

/// 参照を焼いた条件と、検査が使う媒体の値が同じであることを確かめる。
/// これが無いと、別の大気で焼いた期待値と比べて食い違いの原因を見誤る。
#[test]
fn 焼いた条件が器の媒体と一致する() {
    let 期待 = [
        ("bottom_radius", f64::from(super::下端半径)),
        ("top_radius", f64::from(super::上端半径)),
        ("rayleigh_scattering_r", f64::from(super::レイリー散乱係数[0])),
        ("rayleigh_scattering_g", f64::from(super::レイリー散乱係数[1])),
        ("rayleigh_scattering_b", f64::from(super::レイリー散乱係数[2])),
        ("rayleigh_scale_height", f64::from(super::レイリー尺度高度)),
        ("mie_scattering", f64::from(super::ミー散乱係数)),
        ("mie_extinction", f64::from(super::ミー消散係数)),
        ("mie_scale_height", f64::from(super::ミー尺度高度)),
        ("absorption_extinction_r", f64::from(super::吸収消散係数[0])),
        ("absorption_extinction_g", f64::from(super::吸収消散係数[1])),
        ("absorption_extinction_b", f64::from(super::吸収消散係数[2])),
        ("absorption_bottom", f64::from(super::吸収下端高度)),
        ("absorption_peak", f64::from(super::吸収頂点高度)),
        ("absorption_top", f64::from(super::吸収上端高度)),
        ("asymmetry", f64::from(super::位相非対称係数)),
        ("sample_count", f64::from(参照の区間数)),
    ];
    let 一覧 = 参照レコード一覧();
    for (名前, 期待値) in 期待 {
        let 焼いた値 = 一覧.iter().find_map(|レコード| match レコード {
            参照レコード::パラメータ { 名前: 記録名, 値 } if 記録名 == 名前 => Some(*値),
            _ => None,
        });
        assert_eq!(焼いた値, Some(期待値), "パラメータ{名前}が食い違う");
    }
}
