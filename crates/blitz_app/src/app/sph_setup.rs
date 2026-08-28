//! M10のSPH GPU試作用に、入力領域と出力領域を連結した初期バッファを作る。

use blitz_render::粒子素材;

use crate::error::起動エラー;

const 更新スレッド数: u32 = 64;

pub(super) fn 素材を作る(粒子数: u32) -> Result<粒子素材, 起動エラー> {
    let 粒子数usize = usize::try_from(粒子数).unwrap_or_else(|_| panic!("u32がusizeへ変換できないプラットフォームは対象外"));
    let 一辺 = 一辺粒子数を得る(粒子数);
    let mut 入力 = Vec::with_capacity(粒子数usize * 32);
    for 添字 in 0..粒子数 {
        let x = 添字 % 一辺;
        let y = (添字 / 一辺) % 一辺;
        let z = 添字 / (一辺 * 一辺);
        let 位置 = [
            格子座標へ変換する(x, 一辺),
            格子座標へ変換する(y, 一辺) + 0.7,
            格子座標へ変換する(z, 一辺),
        ];
        for 成分 in [
            位置[0],
            位置[1],
            位置[2],
            0.0,
            0.0,
            0.0,
            0.0,
            if 添字 == 0 { 粒子数をf32へ変換する(粒子数) } else { 0.0 },
        ] {
            入力.extend_from_slice(&成分.to_le_bytes());
        }
    }
    let mut バイト列 = 入力.clone();
    バイト列.extend_from_slice(&入力);
    粒子素材::個別件数で生成する(バイト列, 粒子数 * 2, 更新スレッド数, 粒子数).map_err(Into::into)
}

fn 一辺粒子数を得る(粒子数: u32) -> u32 {
    match 粒子数 {
        512 => 8,
        1024 => 11,
        2048 => 13,
        _ => panic!("M10のSPH試作が対応しない粒子数: {粒子数}"),
    }
}

fn 格子座標へ変換する(添字: u32, 一辺: u32) -> f32 {
    (粒子数をf32へ変換する(添字) - 粒子数をf32へ変換する(一辺 - 1) * 0.5) * 0.11
}

fn 粒子数をf32へ変換する(値: u32) -> f32 {
    let 値u16 = u16::try_from(値).unwrap_or_else(|_| panic!("M10の粒子数がu16に収まらない: {値}"));
    f32::from(値u16)
}
