//! メッシュ頂点属性: エンジン中立のCPU側頂点データ。GPU境界型ではないため
//! reprは不要（レンダラー側で改めてGPU境界型に変換する）。

/// 位置・法線・UVを持つ1頂点分の属性。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct メッシュ頂点属性 {
    pub 位置: [f32; 3],
    pub 法線: [f32; 3],
    pub uv: [f32; 2],
}
