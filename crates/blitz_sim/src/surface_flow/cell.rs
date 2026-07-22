//! 表面流の1セルが保持する液膜厚さとUV接線速度。

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct 表面セル {
    液膜厚さ: f32,
    接線速度: [f32; 2],
}

impl 表面セル {
    pub(crate) fn 生成する(液膜厚さ: f32, 接線速度: [f32; 2]) -> Self {
        Self { 液膜厚さ, 接線速度 }
    }

    pub fn 液膜厚さ(&self) -> f32 {
        self.液膜厚さ
    }

    pub fn 接線速度(&self) -> [f32; 2] {
        self.接線速度
    }
}
