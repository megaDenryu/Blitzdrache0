//! 粒子のGPU表現(判断52・54): 1粒子32バイト(std430) = 位置vec4(w=逆質量) + 速度vec4(w=予備0.0)。
//! 布データは静止状態のみを保持するため、初期速度は常に0で埋める。

use crate::cloth::布データ;

const 粒子1件のバイト数: usize = 32;

pub fn 粒子バイト列にする(布: &布データ) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(布.粒子一覧.len() * 粒子1件のバイト数);
    for 粒子 in &布.粒子一覧 {
        バイト列.extend_from_slice(&粒子.位置[0].to_le_bytes());
        バイト列.extend_from_slice(&粒子.位置[1].to_le_bytes());
        バイト列.extend_from_slice(&粒子.位置[2].to_le_bytes());
        バイト列.extend_from_slice(&粒子.逆質量.to_le_bytes());
        バイト列.extend_from_slice(&0.0f32.to_le_bytes());
        バイト列.extend_from_slice(&0.0f32.to_le_bytes());
        バイト列.extend_from_slice(&0.0f32.to_le_bytes());
        バイト列.extend_from_slice(&0.0f32.to_le_bytes());
    }
    バイト列
}
