//! 頂点バッファ用のGPU境界型。フレーム型・単位型を外した生の数値のみを持つ。

/// glTFメッシュ1頂点ぶんの位置・法線・接線・UV。`repr(C)`でVulkanの頂点入力
/// レイアウトと一致させる(判断25: 法線マップ用に接線[f32;4]を追加)。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 頂点 {
    pub 位置: [f32; 3],
    pub 法線: [f32; 3],
    pub 接線: [f32; 4], // UVのU方向に沿った表面ベクトル。w成分は従法線の向き(±1)
    pub uv: [f32; 2],
}
