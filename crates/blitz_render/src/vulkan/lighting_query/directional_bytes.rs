//! 方向光レコードをバイト列へ組み立てる工程。受け取るのはレコード内容、返すのは
//! shaders/lighting_query.slangのDirectionalLightRecordと同じ並びのバイト列である。
//!
//! 注意: 並びはシェーダー側の宣言順と完全に一致させること。ここを崩すと値化けとして現れる。

use super::directional_content::方向光レコード内容;
use crate::frame_lighting::直接光の可視性;
use crate::vulkan::uniform::write_bytes::{uvec4を書き込む, vec4を書き込む, ベクトル3をvec4化};

/// バイト長: float4が2個(32) + uint4が1個(16) = 48。
pub(crate) const バイト長: usize = 48;

/// 可視性の種別の生値。shaders/lighting_query.slangのdirectionalVisibilityNoneと同じ値である。
const 影なしの種別: u32 = 0;
/// 可視性の種別の生値。shaders/lighting_query.slangのdirectionalVisibilityCascadedShadowと同じ値である。
const 多段影の種別: u32 = 1;

pub(crate) fn 方向光レコード内容をバイト列にする(内容: &方向光レコード内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    let mut 位置 = 0usize;
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.光へ向かう方向, 内容.強度));
    vec4を書き込む(&mut バイト列, &mut 位置, ベクトル3をvec4化(内容.色, 0.0));
    uvec4を書き込む(&mut バイト列, &mut 位置, 可視性をuvec4にする(内容.可視性));
    バイト列
}

/// 「影なし」は添字の番兵でなく種別そのもので表す。影なしの枝では資源添字を書かない。
fn 可視性をuvec4にする(可視性: 直接光の可視性) -> [u32; 4] {
    match 可視性 {
        直接光の可視性::影なし => [影なしの種別, 0, 0, 0],
        直接光の可視性::多段影 { 資源添字 } => [多段影の種別, 資源添字.値(), 0, 0],
    }
}
