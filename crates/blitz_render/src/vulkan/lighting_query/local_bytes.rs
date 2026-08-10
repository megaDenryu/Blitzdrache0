//! 局所光レコードをバイト列へ組み立てる工程。受け取るのはレコード内容、返すのは
//! shaders/lighting_query.slangのLocalLightRecordと同じ並びのバイト列である。
//!
//! 注意: 並びはシェーダー側の宣言順と完全に一致させること。ここを崩すと値化けとして現れる。

use super::local_content::{局所光の種別, 局所光レコード内容};
use crate::vulkan::uniform::write_bytes::{uvec4を書き込む, vec4を書き込む, ベクトル3をvec4化};

/// バイト長: float4が2個(32) + uint4が1個(16) + 第4の区画(16) = 64。
pub(crate) const バイト長: usize = 64;

/// 光種の生値。shaders/lighting_query.slangのlocalLightKindPointと同じ値である。
const 点光の種別: u32 = 0;

/// 影の有無の生値のうち「影を持たない」側。値の読み書きが揃うのは点光源の影を入れる段であり、
/// それまでの書き込みは常にこちらである(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断c」)。
const 影を持たない: u32 = 0;

/// 影資源添字の書き込み値。影を持たない光の添字は読まれないため0で残す。番兵として使う値ではない。
const 影資源添字の初期値: u32 = 0;

pub(crate) fn バイト列にする(内容: &局所光レコード内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    let mut 位置 = 0usize;
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.カメラ相対位置, 0.0));
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.色, 内容.強度));
    uvec4を書き込む(&mut バイト列, &mut 位置, [種別をuintにする(内容.種別), 0, 0, 0]);
    // GPU境界(生値境界): 影響半径をメートルの生値へ戻すのはこの1箇所だけである。
    バイト列[48..52].copy_from_slice(&内容.影響半径.長さ().値().to_le_bytes());
    バイト列[52..56].copy_from_slice(&影を持たない.to_le_bytes());
    バイト列[56..60].copy_from_slice(&影資源添字の初期値.to_le_bytes());
    バイト列
}

fn 種別をuintにする(種別: 局所光の種別) -> u32 {
    match 種別 {
        局所光の種別::点光 => 点光の種別,
    }
}
