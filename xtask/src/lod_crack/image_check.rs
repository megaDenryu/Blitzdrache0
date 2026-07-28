//! 地形外周を除いた内側で、白い地形に両側を挟まれた番兵背景画素を継ぎ目の亀裂として数える。

use super::run::読み戻し画像;

const 背景RGB: [u8; 3] = [63, 75, 97];
const 探索半径: usize = 8;

pub(super) struct 検査結果 {
    pub(super) roi画素数: u64,
}

pub(super) fn 継ぎ目を検査する(画像: &読み戻し画像) -> Result<検査結果, String> {
    let 期待長 = 画像
        .幅
        .checked_mul(画像.高さ)
        .and_then(|数| 数.checked_mul(4))
        .ok_or_else(|| "寸法が大きすぎる".to_string())?;
    if 画像.rgba8.len() != 期待長 {
        return Err(format!("寸法とRGBA8長が違う: {期待長} と {}", 画像.rgba8.len()));
    }
    let x余白 = 画像.幅 / 10;
    let y余白 = 画像.高さ / 10;
    let mut roi画素数 = 0_u64;
    let mut 背景画素数 = 0_u64;
    for y in y余白.max(探索半径)..画像.高さ - y余白.max(探索半径) {
        for x in x余白.max(探索半径)..画像.幅 - x余白.max(探索半径) {
            roi画素数 += 1;
            if 継ぎ目背景か(画像, x, y) {
                背景画素数 += 1;
            }
        }
    }
    if 背景画素数 > 0 {
        Err(format!("継ぎ目ROIに番兵背景が{背景画素数}画素露出した"))
    } else {
        Ok(検査結果 { roi画素数 })
    }
}

fn 継ぎ目背景か(画像: &読み戻し画像, x: usize, y: usize) -> bool {
    if !背景か(画像, x, y) {
        return false;
    }
    (1..=探索半径).any(|距離| {
        (白か(画像, x - 距離, y) && 白か(画像, x + 距離, y))
            || (白か(画像, x, y - 距離) && 白か(画像, x, y + 距離))
            || (白か(画像, x - 距離, y - 距離) && 白か(画像, x + 距離, y + 距離))
            || (白か(画像, x + 距離, y - 距離) && 白か(画像, x - 距離, y + 距離))
    })
}

fn 背景か(画像: &読み戻し画像, x: usize, y: usize) -> bool {
    let 開始 = (y * 画像.幅 + x) * 4;
    画像.rgba8[開始..開始 + 3] == 背景RGB
}

fn 白か(画像: &読み戻し画像, x: usize, y: usize) -> bool {
    let 開始 = (y * 画像.幅 + x) * 4;
    画像.rgba8[開始..開始 + 3] == [255, 255, 255]
}
