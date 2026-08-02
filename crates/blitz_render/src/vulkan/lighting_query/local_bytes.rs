//! 局所光レコードをバイト列へ組み立てる工程。受け取るのはレコード内容、返すのは
//! shaders/lighting_query.slangのLocalLightRecordと同じ並びのバイト列である。
//!
//! 注意: 並びはシェーダー側の宣言順と完全に一致させること。ここを崩すと値化けとして現れる。

use super::local_content::{局所光の種別, 局所光レコード内容};
use crate::vulkan::uniform::write_bytes::{uvec4を書き込む, vec4を書き込む, ベクトル3をvec4化};

/// バイト長: float4が2個(32) + uint4が1個(16) = 48。
pub(crate) const バイト長: usize = 48;

/// 光種の生値。shaders/lighting_query.slangのlocalLightKindPointと同じ値である。
const 点光の種別: u32 = 0;

pub(crate) fn バイト列にする(内容: &局所光レコード内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    let mut 位置 = 0usize;
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.カメラ相対位置, 0.0));
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.色, 内容.強度));
    uvec4を書き込む(&mut バイト列, &mut 位置, [種別をuintにする(内容.種別), 0, 0, 0]);
    バイト列
}

fn 種別をuintにする(種別: 局所光の種別) -> u32 {
    match 種別 {
        局所光の種別::点光 => 点光の種別,
    }
}
