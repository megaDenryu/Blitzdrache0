//! 曲げ拘束のバッチのGPU表現(判断9)。静的な引数を1本24バイト(辺a添字u32・辺b添字u32・翼c添字u32・翼d添字u32・静止角f32・コンプライアンスf32)で符号化する。
//! 並びは`曲げ拘束の彩色`が色ごとに並べ替えた順である。ラグランジュ乗数は距離拘束と目標拘束の乗数の後ろへ続けて置き(1本4バイト)、
//! その初期値は`ラグランジュ乗数の初期バイト列にする`が3つの合計で作る。blitz_renderの布の曲げ拘束のslangがこの並びで読む(バインディング表は`shaders/cloth_step.slang`の冒頭)。

use crate::constraint_graph::曲げ拘束の彩色;

pub const 曲げ拘束1件のバイト数: usize = 24;

pub fn 曲げ拘束の引数バイト列にする(彩色: &曲げ拘束の彩色) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(彩色.拘束の数() * 曲げ拘束1件のバイト数);
    for 拘束 in 彩色.拘束一覧() {
        for 点 in 拘束.点一覧() {
            バイト列.extend_from_slice(&点.値().to_le_bytes());
        }
        バイト列.extend_from_slice(&拘束.引数.静止角.値().値().to_le_bytes());
        バイト列.extend_from_slice(&拘束.引数.コンプライアンス.値().to_le_bytes());
    }
    バイト列
}
