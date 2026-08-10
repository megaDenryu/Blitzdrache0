//! 視野の半角の正接という量。触れる状態を持たず、担うのは横と縦がともに有限かつ正であることの保証である。
//!
//! 値の意味は、カメラから奥行き1メートルの面で視錐台が横と縦へ広がる幅の半分である。透視投影の行列でなく
//! この2つの数で持つのは、セルの錐台の導出が行列の一般の逆を必要とせず、2つの数なら不変条件(視野が開いている)を
//! 型が守れるためである(`crates/blitz_render/src/local_visibility/projection.rs`が同じ理由で4つの量を持つ)。

use super::input_error::クラスタ多光源入力エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 視野の半角の正接 {
    横: f32,
    縦: f32,
}

impl 視野の半角の正接 {
    pub fn 生成する(横: f32, 縦: f32) -> Result<Self, クラスタ多光源入力エラー> {
        if !横.is_finite() || 横 <= 0.0 || !縦.is_finite() || 縦 <= 0.0 {
            return Err(クラスタ多光源入力エラー::視野の半角の正接不正 { 横, 縦 });
        }
        Ok(Self { 横, 縦 })
    }

    pub fn 横(self) -> f32 {
        self.横
    }

    pub fn 縦(self) -> f32 {
        self.縦
    }
}
