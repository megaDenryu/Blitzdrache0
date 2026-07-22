//! 指定数の描画対象を固定カメラ内の格子へ収めるワールド空間変換。

use blitz_math::{クォータニオン, メートル, ワールド, 位置, 変換};

const 配置幅メートル: f32 = 2.4;
const セル占有率: f32 = 0.8;

pub(super) fn 配置する(添字: usize, 件数: usize) -> 変換<ワールド, ワールド> {
    let 列数 = 正方格子の列数(件数);
    let 行数 = 件数.div_ceil(列数);
    let 列 = 添字 % 列数;
    let 行 = 添字 / 列数;
    let 最大辺 = 列数.max(行数);
    let 間隔 = 配置幅メートル / usizeをf32へ変換する(最大辺);
    let スケール = (間隔 * セル占有率).min(1.0);
    let x = 中心基準座標(列, 列数) * 間隔;
    let y = -中心基準座標(行, 行数) * 間隔;
    変換::trsから生成する(
        位置::生成する(メートル::生成する(x), メートル::生成する(y), メートル::生成する(0.0)),
        クォータニオン::恒等(),
        [スケール; 3],
    )
}

fn 正方格子の列数(件数: usize) -> usize {
    let mut 列数 = 1usize;
    while 列数.saturating_mul(列数) < 件数 {
        列数 += 1;
    }
    列数
}

fn 中心基準座標(添字: usize, 件数: usize) -> f32 {
    usizeをf32へ変換する(添字) - (usizeをf32へ変換する(件数) - 1.0) * 0.5
}

fn usizeをf32へ変換する(値: usize) -> f32 {
    let 値u32 = u32::try_from(値).unwrap_or_else(|_| panic!("描画対象の格子値がu32に収まらない: {値}"));
    let 上位 = u16::try_from(値u32 >> 16).unwrap_or_else(|_| panic!("u32上位値がu16に収まらない"));
    let 下位 = u16::try_from(値u32 & 0xffff).unwrap_or_else(|_| panic!("u32下位値がu16に収まらない"));
    f32::from(上位) * 65_536.0 + f32::from(下位)
}
