//! 放射輝度をRGBの3成分で表す色。

use super::天空状態エラー;

/// 光がどれだけの強さでその向きから来るかを表すRGBの3成分。
/// 不変条件: 3成分とも有限かつ0以上である。0以上に限るのは、負の放射輝度が物理的にも描画的にも意味を持たないためである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 放射輝度色 {
    成分: [f32; 3],
}

impl 放射輝度色 {
    pub fn 生成する(赤: f32, 緑: f32, 青: f32) -> Result<Self, 天空状態エラー> {
        let 成分 = [赤, 緑, 青];
        for 値 in 成分 {
            if !値.is_finite() || 値 < 0.0 {
                return Err(天空状態エラー::値域外("放射輝度色", 値));
            }
        }
        Ok(Self { 成分 })
    }

    pub fn 成分(&self) -> [f32; 3] {
        self.成分
    }
}
