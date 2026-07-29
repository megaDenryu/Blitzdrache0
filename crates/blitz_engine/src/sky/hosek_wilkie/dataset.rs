//! Hosek-Wilkie解析近似の係数データセットの取り込み口。担当する工程は「配列の種類と添字を受け取り1つの係数を返す」ことである。
//! 原著者の公開ヘッダ`ArHosekSkyModelData_RGB.h`を`cargo xtask gen-sky-dataset`でリトルエンディアンのf32列へ焼いたものを取り込む
//! (出典・書庫のSHA-256・変換手順はそのコマンドの冒頭にある)。原データは3条項BSDライセンスであり、
//! 著作権表示は`crates/blitz_engine/data/hosek_wilkie_rgb_f32le.LICENSE.txt`に置く。
//!
//! 並びは前半が方向係数(datasetRGB1〜3を順に連結)、後半が放射輝度スケール(datasetRGBRad1〜3を順に連結)である。
//! どちらもアルベド2段 × 濁度10段 × 制御点6個の順に並び、方向係数はその内側に係数9個を持つ。

const バイト列: &[u8] = include_bytes!("../../../data/hosek_wilkie_rgb_f32le.bin");

pub(in crate::sky) const チャネル数: usize = 3;
pub(in crate::sky) const 係数数: usize = 9;
pub(super) const 制御点数: usize = 6;
pub(super) const 濁度段数: usize = 10;

const 方向係数のチャネル長: usize = 2 * 濁度段数 * 制御点数 * 係数数;
const 放射輝度のチャネル長: usize = 2 * 濁度段数 * 制御点数;
const 放射輝度の先頭: usize = チャネル数 * 方向係数のチャネル長;

/// アルベド1側の並びが始まる、チャネル内の相対位置。
pub(super) const 方向係数のアルベド1開始: usize = 濁度段数 * 制御点数 * 係数数;
pub(super) const 放射輝度のアルベド1開始: usize = 濁度段数 * 制御点数;

pub(super) fn 方向係数(チャネル: usize, チャネル内添字: usize) -> f32 {
    読む(チャネル * 方向係数のチャネル長 + チャネル内添字)
}

pub(super) fn 放射輝度(チャネル: usize, チャネル内添字: usize) -> f32 {
    読む(放射輝度の先頭 + チャネル * 放射輝度のチャネル長 + チャネル内添字)
}

/// 焼いた列の長さが期待と違えばコンパイルを止める。データセットを差し替えたときに、
/// 添字だけが静かにずれて絵の異常として現れることを防ぐ。
const _: () = assert!(
    バイト列.len() == 4 * チャネル数 * (方向係数のチャネル長 + 放射輝度のチャネル長),
    "焼いた空データセットの長さが期待と違う"
);

fn 要素数() -> usize {
    バイト列.len() / 4
}

/// 位置は要素単位(f32単位)であり、バイト単位ではない。
/// 範囲外の位置はプログラムのバグであるため、破れた不変条件を示して停止する。
fn 読む(位置: usize) -> f32 {
    let 開始 = 位置 * 4;
    let Some(四バイト) = バイト列.get(開始..開始 + 4) else {
        panic!("空データセットの範囲外を読もうとした: 位置{位置}、要素数{}", 要素数());
    };
    let mut 器 = [0u8; 4];
    器.copy_from_slice(四バイト);
    f32::from_le_bytes(器)
}
