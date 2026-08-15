//! 対照の幾何輪郭2pxと、近傍・遠景の間に残る1px空隙を求める。

use super::image_pair::検収画像;

pub(super) fn 輪郭を二画素広げる(画像: &検収画像) -> Vec<bool> {
    let mut 輪郭 = vec![false; 画像.画素数()];
    for y in 0..画像.高さ {
        for x in 0..画像.幅 {
            let 添字 = y * 画像.幅 + x;
            let 幾何 = 画像.深度[添字] > 0.0;
            if 隣に別領域がある(画像, x, y, 幾何) {
                広げる(&mut 輪郭, 画像.幅, 画像.高さ, x, y);
            }
        }
    }
    輪郭
}

/// 印の付いた画素を2画素ぶん外へ広げた帯。散布の検査点が「変わってよい範囲を散布物の画素とその輪郭2画素に限る」
/// ために使う。広げ方を輪郭の工程と1つにするのは、2つの検査点が違う幅の帯を持つと免除の意味が食い違うためである。
pub(super) fn 印を二画素広げる(幅: usize, 高さ: usize, 印: &[bool]) -> Vec<bool> {
    let mut 帯 = vec![false; 印.len()];
    for y in 0..高さ {
        for x in 0..幅 {
            if 印[y * 幅 + x] {
                広げる(&mut 帯, 幅, 高さ, x, y);
            }
        }
    }
    帯
}

fn 隣に別領域がある(画像: &検収画像, x: usize, y: usize, 幾何: bool) -> bool {
    隣接位置(画像.幅, 画像.高さ, x, y).any(|添字| (画像.深度[添字] > 0.0) != 幾何)
}

fn 広げる(結果: &mut [bool], 幅: usize, 高さ: usize, x: usize, y: usize) {
    let 最小x = x.saturating_sub(2);
    let 最大x = (x + 2).min(幅 - 1);
    let 最小y = y.saturating_sub(2);
    let 最大y = (y + 2).min(高さ - 1);
    for 周辺y in 最小y..=最大y {
        for 周辺x in 最小x..=最大x {
            結果[周辺y * 幅 + 周辺x] = true;
        }
    }
}

fn 隣接位置(幅: usize, 高さ: usize, x: usize, y: usize) -> impl Iterator<Item = usize> {
    let 左 = x.saturating_sub(1);
    let 右 = (x + 1).min(幅 - 1);
    let 上 = y.saturating_sub(1);
    let 下 = (y + 1).min(高さ - 1);
    (上..=下)
        .flat_map(move |周辺y| (左..=右).map(move |周辺x| 周辺y * 幅 + 周辺x))
        .filter(move |添字| *添字 != y * 幅 + x)
}
