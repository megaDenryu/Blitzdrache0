//! 時間再構成のパスが押し込む即時定数の並び。受け取るのは射影の復元・画面寸法・描画設定・履歴を混ぜるかであり、
//! 返すのはそのままGPUへ渡せるバイト列である。
//!
//! 並びの写しは`shaders/temporal_reconstruction_setting.slang`の`TemporalReconstructionSetting`にあり、
//! この工程が並びの正本である。混合の2つの数はblitz_engineの純関数層が値の正本を持ち、
//! コンポジションルートが開いた値を描画設定として受け取るだけであるため、ここは値そのものを1つも持たない。

use ash::vk;

use crate::local_visibility::射影の復元;
use crate::temporal_reconstruction::時間再構成の描画設定;

/// 押し込む定数のバイト数。単精度4個と符号なし整数3個の並びである。
pub(crate) const 即時定数バイト数: u32 = 28;

/// 即時定数の並びを組み立てる。
pub(crate) fn 即時定数を組み立てる(
    射影: 射影の復元, 寸法: vk::Extent2D, 設定: 時間再構成の描画設定, 履歴を混ぜるか: bool
) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(usize::try_from(即時定数バイト数).unwrap_or(0));
    単精度を足す(&mut バイト列, 射影.近クリップ().値());
    単精度を足す(&mut バイト列, 射影.遠クリップ().値());
    単精度を足す(&mut バイト列, 設定.今のフレームの寄与率);
    単精度を足す(&mut バイト列, 設定.出現領域の相対許容);
    整数を足す(&mut バイト列, 寸法.width);
    整数を足す(&mut バイト列, 寸法.height);
    整数を足す(&mut バイト列, u32::from(履歴を混ぜるか));
    バイト列
}

fn 単精度を足す(バイト列: &mut Vec<u8>, 値: f32) {
    バイト列.extend_from_slice(&値.to_le_bytes());
}

fn 整数を足す(バイト列: &mut Vec<u8>, 値: u32) {
    バイト列.extend_from_slice(&値.to_le_bytes());
}
