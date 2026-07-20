//! 頂点バッファ用のGPU境界型。フレーム型・単位型を外した生の数値のみを持つ。

/// glTFメッシュ1頂点ぶんの位置・法線・UV。`repr(C)`でVulkanの頂点入力レイアウトと
/// 一致させる(判断19: 頂点色は廃止し、位置+法線+UVへ拡張)。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 頂点 {
    pub 位置: [f32; 3],
    pub 法線: [f32; 3],
    pub uv: [f32; 2],
}
