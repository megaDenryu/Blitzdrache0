//! 地表高さの取り出しの検査。格子点の上では標本そのものを返すこと、升目の中では両端の間に収まること、
//! チャンクの外を指す座標を型付きエラーで拒むことを見る。

use crate::terrain::ground_height::地表高さを求める;

use super::fixture::{格子を切り出す, 検査用一辺メートル, 検査用辺分割数};

fn 刻みメートル() -> f32 {
    検査用一辺メートル / f32::from(検査用辺分割数)
}

#[test]
fn 格子点の上では標本そのものを返す() {
    let 格子 = 格子を切り出す(blitz_engine::チャンク座標::生成する(1, -1));
    let 刻み = 刻みメートル();
    for 格子添字z in 0..=4 {
        for 格子添字x in 0..=4 {
            let 期待 = 格子.高さ(格子添字x, 格子添字z).unwrap();
            let 実際 = 地表高さを求める(
                &格子,
                f32::from(i16::try_from(格子添字x).unwrap()) * 刻み,
                f32::from(i16::try_from(格子添字z).unwrap()) * 刻み,
            )
            .unwrap();
            assert!((実際 - 期待).abs() < 1.0e-3, "格子点({格子添字x},{格子添字z})で{実際}が標本{期待}と違う");
        }
    }
}

#[test]
fn 升目の中では四隅の最小と最大の間に収まる() {
    let 格子 = 格子を切り出す(blitz_engine::チャンク座標::生成する(0, 0));
    let 刻み = 刻みメートル();
    let 四隅: Vec<f32> = [(3, 5), (4, 5), (3, 6), (4, 6)]
        .into_iter()
        .map(|(x, z)| 格子.高さ(x, z).unwrap())
        .collect();
    let 最小 = 四隅.iter().copied().fold(f32::INFINITY, f32::min);
    let 最大 = 四隅.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let 実際 = 地表高さを求める(&格子, 3.5 * 刻み, 5.25 * 刻み).unwrap();
    assert!((最小..=最大).contains(&実際), "{実際}が四隅の範囲{最小}..={最大}の外にある");
}

#[test]
fn 負の座標を型付きエラーで拒む() {
    let 格子 = 格子を切り出す(blitz_engine::チャンク座標::生成する(0, 0));
    assert!(地表高さを求める(&格子, -1.0, 0.0).is_err());
}
