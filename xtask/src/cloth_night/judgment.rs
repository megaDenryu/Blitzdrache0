//! 布の領域を切り出して平均輝度を測る工程。受け取るのは布ありと布なしの2枚、返すのは布の画素数と平均輝度である。
//!
//! 布の領域を色で決めず2枚の差で決めるのは、布の色が時刻ごとの光で変わるためである。
//! 同じ時刻の布ありと布なしを比べれば、差が出る画素はその時刻に布が覆った画素だけになる。

use crate::vegetation_run::実行結果;

/// 布が覆ったと数える成分差の下限。読み戻しの量子化ゆらぎを除く。
const 成分差下限: u8 = 4;

pub(super) struct 布領域 {
    pub(super) 画素数: usize,
    pub(super) 平均輝度: f64,
}

pub(super) fn 布領域を測る(布あり: &実行結果, 布なし: &実行結果) -> Result<布領域, String> {
    if 布あり.rgba8.len() != 布なし.rgba8.len() {
        return Err("布ありと布なしの読み戻し長が違う".to_string());
    }
    let mut 画素数 = 0usize;
    let mut 輝度合計 = 0.0_f64;
    for (左, 右) in 布あり.rgba8.chunks_exact(4).zip(布なし.rgba8.chunks_exact(4)) {
        let 差 = 左.iter().zip(右.iter()).take(3).map(|(&p, &q)| p.abs_diff(q)).max().unwrap_or(0);
        if 差 < 成分差下限 {
            continue;
        }
        画素数 += 1;
        輝度合計 += (f64::from(左[0]) + f64::from(左[1]) + f64::from(左[2])) / 3.0;
    }
    if 画素数 == 0 {
        return Err("布ありと布なしの差が0画素だった: 布が1画素も描かれていない".to_string());
    }
    let 割る数 = u32::try_from(画素数).map_err(|_| "布の画素数がu32に収まらない".to_string())?;
    Ok(布領域 {
        画素数,
        平均輝度: 輝度合計 / f64::from(割る数),
    })
}
