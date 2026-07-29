//! 地表の反射率。空の放射輝度が地面からの照り返しを受ける量を決める。

use super::天空状態エラー;

/// 解析近似が受け取る地表の反射率。0が完全に吸収する地面、1が完全に反射する地面である。
/// 時刻では決まらない値のため、シーンの方針が持つ。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 地表アルベド(f32);

impl 地表アルベド {
    pub fn 生成する(値: f32) -> Result<Self, 天空状態エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(天空状態エラー::値域外("地表アルベド", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }
}
