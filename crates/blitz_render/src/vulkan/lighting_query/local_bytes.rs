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

/// 影の有無の生値。shaders/local_light_records.slangのhasShadowと同じ規約であり、0が影なしである
/// (参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断c」「判断k」)。
const 影を持たない: u32 = 0;
const 影を持つ: u32 = 1;

/// 影を持たない光の影資源添字へ書く値。GPU側は影の有無を先に読むためこの値を参照しない。番兵として使う値ではない。
const 影を持たない光の影資源添字: u32 = 0;

pub(crate) fn 局所光レコード内容をバイト列にする(内容: &局所光レコード内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    let mut 位置 = 0usize;
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.カメラ相対位置, 0.0));
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.色, 内容.強度));
    uvec4を書き込む(&mut バイト列, &mut 位置, [種別をuintにする(内容.種別), 0, 0, 0]);
    // GPU境界(生値境界): 影響半径をメートルの生値へ戻すのはこの1箇所だけである。
    バイト列[48..52].copy_from_slice(&内容.影響半径.長さ().値().to_le_bytes());
    let (影の有無, 影資源添字) = match 内容.影資源添字 {
        Some(添字) => (影を持つ, 添字.値()),
        None => (影を持たない, 影を持たない光の影資源添字),
    };
    バイト列[52..56].copy_from_slice(&影の有無.to_le_bytes());
    バイト列[56..60].copy_from_slice(&影資源添字.to_le_bytes());
    バイト列
}

fn 種別をuintにする(種別: 局所光の種別) -> u32 {
    match 種別 {
        局所光の種別::点光 => 点光の種別,
    }
}
