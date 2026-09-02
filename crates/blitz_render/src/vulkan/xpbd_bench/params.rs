//! 計測の定数UBO(XpbdParams、48バイト)のバイト列化。レイアウトは`shaders/xpbd_step.slang`冒頭の構造体と一致させる
//! (std140。全メンバがfloat4/uint4のため詰め物なし)。
//! 固定小数の尺度は原子加算の方式の量子化の桁を決める。2の20乗(約0.95マイクロメートル)にするのは、補正がメートルの
//! 桁の小数であり、32ビット整数の範囲(±2048メートル)を超えず、単精度の仮数(24ビット)より粗い桁で丸めるためである。

use crate::xpbd_solver_bench_probe::XPBD計測の刻みの定数;

pub(super) const バイト長: usize = 48;

/// 原子加算の方式が補正をメートルから固定小数へ写す尺度。`shaders/xpbd_atomic.slang`が定数UBOから読む。
pub(crate) const 固定小数の尺度: f32 = 1048576.0;

pub(super) fn バイト列にする(点の数: u32, 拘束の数: u32, 刻み: &XPBD計測の刻みの定数) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(バイト長);
    for 値 in [
        刻み.加速度による変位[0],
        刻み.加速度による変位[1],
        刻み.加速度による変位[2],
        刻み.刻み幅の2乗の逆数,
    ] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [点の数, 拘束の数, 0, 0] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [固定小数の尺度, 0.0, 0.0, 0.0] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    バイト列
}
