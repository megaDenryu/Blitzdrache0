//! 提示色の差を診断し、候補の輪郭から離れた重大差だけを拒む。

#[cfg(test)]
mod tests;

use super::super::depth_image::{深度画像, 逆向き深度の奥行き};
use super::super::error::逆Z検収エラー;
use crate::acceptance::読み戻し画像;

const 重大差の下限: u8 = 1;
const 色輪郭の下限: u8 = 24;
const 輪郭から許す距離: usize = 2;

pub(super) fn 色を比べる(
    対照: &読み戻し画像, 候補: &読み戻し画像, 候補深度: &深度画像
) -> Result<(u8, usize), 逆Z検収エラー> {
    let 幅 = 候補.幅().画素数();
    let 高さ = 候補.高さ().画素数();
    let 輪郭: Vec<bool> = (0..候補.画素数()).map(|番号| 輪郭か(番号, 幅, 高さ, 候補, 候補深度)).collect();
    let mut 最大差 = 0;
    let mut 差画素数 = 0;
    let mut 輪郭外の重大差 = 0;
    for (番号, (左, 右)) in 対照.画素列().zip(候補.画素列()).enumerate() {
        let 差 = 左.iter().zip(右).fold(0, |最大, (a, b)| 最大.max(a.abs_diff(*b)));
        最大差 = 最大差.max(差);
        差画素数 += usize::from(差 > 0);
        輪郭外の重大差 += usize::from(差 > 重大差の下限 && !近くに輪郭があるか(番号, 幅, 高さ, &輪郭));
    }
    if 輪郭外の重大差 > 0 {
        return Err(逆Z検収エラー::判定が不合格(format!(
            "輪郭から2px超離れた提示色の2階調以上の差が{輪郭外の重大差}画素ある"
        )));
    }
    Ok((最大差, 差画素数))
}

fn 輪郭か(番号: usize, 幅: usize, 高さ: usize, 色: &読み戻し画像, 深度: &深度画像) -> bool {
    let 横 = 番号 % 幅;
    let 縦 = 番号 / 幅;
    隣接位置(横, 縦, 幅, 高さ).any(|隣| {
        深度が不連続か(深度.深度列()[番号], 深度.深度列()[隣])
            || 画素差(&色.バイト列()[番号 * 4..番号 * 4 + 3], &色.バイト列()[隣 * 4..隣 * 4 + 3]) > 色輪郭の下限
    })
}

fn 深度が不連続か(左: f32, 右: f32) -> bool {
    if (左 <= 0.0) != (右 <= 0.0) {
        return true;
    }
    if 左 <= 0.0 {
        return false;
    }
    let 左奥行き = 逆向き深度の奥行き(左);
    let 右奥行き = 逆向き深度の奥行き(右);
    (左奥行き - 右奥行き).abs() > 0.05_f64.max(左奥行き * 0.001)
}

fn 近くに輪郭があるか(番号: usize, 幅: usize, 高さ: usize, 輪郭: &[bool]) -> bool {
    let 横 = 番号 % 幅;
    let 縦 = 番号 / 幅;
    let 左 = 横.saturating_sub(輪郭から許す距離);
    let 上 = 縦.saturating_sub(輪郭から許す距離);
    (上..=(縦 + 輪郭から許す距離).min(高さ - 1)).any(|y| (左..=(横 + 輪郭から許す距離).min(幅 - 1)).any(|x| 輪郭[y * 幅 + x]))
}

fn 隣接位置(横: usize, 縦: usize, 幅: usize, 高さ: usize) -> impl Iterator<Item = usize> {
    let mut 一覧 = [None; 4];
    if 横 > 0 {
        一覧[0] = Some(縦 * 幅 + 横 - 1)
    }
    if 横 + 1 < 幅 {
        一覧[1] = Some(縦 * 幅 + 横 + 1)
    }
    if 縦 > 0 {
        一覧[2] = Some((縦 - 1) * 幅 + 横)
    }
    if 縦 + 1 < 高さ {
        一覧[3] = Some((縦 + 1) * 幅 + 横)
    }
    一覧.into_iter().flatten()
}

fn 画素差(左: &[u8], 右: &[u8]) -> u8 {
    左.iter().zip(右).fold(0, |最大, (a, b)| 最大.max(a.abs_diff(*b)))
}
