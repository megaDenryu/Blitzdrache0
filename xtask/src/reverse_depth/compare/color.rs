//! 提示色の差を診断し、候補の輪郭から離れた重大差だけを拒む。

mod edge;
#[cfg(test)]
mod tests;

use super::super::depth_image::深度画像;
use super::super::error::逆Z検収エラー;
use crate::acceptance::読み戻し画像;

// 1階調差は8ビット量子化の丸めで生じるため、2階調以上だけを破綻として扱う。
const 重大差の下限: u8 = 1;
// ラスタ勝者差が時間再構成で2画素まで広がった2000mの実測を包む。
const 輪郭から許す距離: usize = 2;

pub(super) struct 色差の集計 {
    pub(super) 最大差: u8,
    pub(super) 差画素数: usize,
    pub(super) 輪郭外の検査画素数: usize,
    pub(super) 輪郭外の幾何画素数: usize,
}

pub(super) fn 色を比べる(
    対照: &読み戻し画像, 候補: &読み戻し画像, 候補深度: &深度画像
) -> Result<色差の集計, 逆Z検収エラー> {
    let 幅 = 候補.幅().画素数();
    let 高さ = 候補.高さ().画素数();
    let 輪郭 = edge::輪郭画像を作る(候補, 候補深度);
    let mut 最大差 = 0;
    let mut 差画素数 = 0;
    let mut 輪郭外の重大差 = 0;
    let mut 輪郭外の検査画素数 = 0;
    let mut 輪郭外の幾何画素数 = 0;
    for (番号, (左, 右)) in 対照.画素列().zip(候補.画素列()).enumerate() {
        let 差 = 左.iter().zip(右).fold(0, |最大, (a, b)| 最大.max(a.abs_diff(*b)));
        最大差 = 最大差.max(差);
        差画素数 += usize::from(差 > 0);
        if !近くに輪郭があるか(番号, 幅, 高さ, &輪郭) {
            輪郭外の検査画素数 += 1;
            輪郭外の幾何画素数 += usize::from(候補深度.深度列()[番号] > 0.0);
            輪郭外の重大差 += usize::from(差 > 重大差の下限);
        }
    }
    if 輪郭外の幾何画素数 == 0 {
        return Err(逆Z検収エラー::判定が不合格(
            "輪郭外で検査したジオメトリ画素が0件である".to_string(),
        ));
    }
    if 輪郭外の重大差 > 0 {
        return Err(逆Z検収エラー::判定が不合格(format!(
            "輪郭から2px超離れた提示色の2階調以上の差が{輪郭外の重大差}画素ある"
        )));
    }
    Ok(色差の集計 {
        最大差,
        差画素数,
        輪郭外の検査画素数,
        輪郭外の幾何画素数,
    })
}

fn 近くに輪郭があるか(番号: usize, 幅: usize, 高さ: usize, 輪郭: &[bool]) -> bool {
    let 横 = 番号 % 幅;
    let 縦 = 番号 / 幅;
    let 左 = 横.saturating_sub(輪郭から許す距離);
    let 上 = 縦.saturating_sub(輪郭から許す距離);
    (上..=(縦 + 輪郭から許す距離).min(高さ - 1)).any(|y| (左..=(横 + 輪郭から許す距離).min(幅 - 1)).any(|x| 輪郭[y * 幅 + x]))
}
