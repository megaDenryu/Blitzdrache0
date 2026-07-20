//! 頂点バッファ用のGPU境界型。フレーム型・単位型を外した生の数値のみを持つ。

/// 立方体メッシュ1頂点ぶんの位置・色。`repr(C)`でVulkanの頂点入力レイアウトと一致させる。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 頂点 {
    pub 位置: [f32; 3],
    pub 色: [f32; 3],
}
