//! 距離拘束のGPU表現(判断52・54): 1拘束16バイト(std430) =
//! a添字u32 + b添字u32 + 静止長f32 + パディングf32(0.0)。

use crate::cloth::布データ;

const 拘束1件のバイト数: usize = 16;

pub fn 拘束バイト列にする(布: &布データ) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(布.距離拘束一覧.len() * 拘束1件のバイト数);
    for 拘束 in &布.距離拘束一覧 {
        バイト列.extend_from_slice(&拘束.粒子a添字.to_le_bytes());
        バイト列.extend_from_slice(&拘束.粒子b添字.to_le_bytes());
        バイト列.extend_from_slice(&拘束.静止長.to_le_bytes());
        バイト列.extend_from_slice(&0.0f32.to_le_bytes());
    }
    バイト列
}
