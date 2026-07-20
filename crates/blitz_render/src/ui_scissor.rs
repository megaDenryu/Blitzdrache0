//! 開発用UIメッシュのシザー矩形(物理ピクセル)。値オブジェクトだが範囲外の値は
//! 実行器がスワップチェーン寸法へクランプするため、ここでの検証は行わない。

/// UIメッシュ1つぶんのシザー矩形(物理ピクセル、左上原点)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UIシザー矩形px {
    x: u32,
    y: u32,
    幅: u32,
    高さ: u32,
}

impl UIシザー矩形px {
    pub fn 生成する(x: u32, y: u32, 幅: u32, 高さ: u32) -> Self {
        Self { x, y, 幅, 高さ }
    }

    pub(crate) fn x(&self) -> u32 {
        self.x
    }

    pub(crate) fn y(&self) -> u32 {
        self.y
    }

    pub(crate) fn 幅(&self) -> u32 {
        self.幅
    }

    pub(crate) fn 高さ(&self) -> u32 {
        self.高さ
    }
}
