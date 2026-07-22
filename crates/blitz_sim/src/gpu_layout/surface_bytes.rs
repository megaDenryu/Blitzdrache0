//! 表面流セルのGPU表現: 1セル32バイト(std430) = 状態vec4 + 予備vec4。

use crate::surface_flow::表面流状態;

const セル1件のバイト数: usize = 32;

pub fn 表面流バイト列にする(状態: &表面流状態) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(状態.セル一覧().len() * セル1件のバイト数);
    for セル in 状態.セル一覧() {
        let 速度 = セル.接線速度();
        for 成分 in [セル.液膜厚さ(), 速度[0], 速度[1], 0.0, 0.0, 0.0, 0.0, 0.0] {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
    }
    バイト列
}
