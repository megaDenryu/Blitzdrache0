//! 描画のクリアカラーを表す値オブジェクト。

use thiserror::Error;

/// クリアカラーの成分が正規化範囲(0.0〜1.0)を外れたときのエラー。
/// クリアカラー専用の軽量エラーであり、レンダラーエラーとは独立に扱う
/// （生成する 段階ではレンダラーがまだ存在しないため）。
#[derive(Debug, Error, Clone, Copy, PartialEq)]
#[error("クリアカラーの成分が0.0から1.0の範囲外だった: {0}")]
pub struct クリアカラーエラー(pub f32);

/// フレーム先頭でカラーアタッチメントをクリアする色。rgba各成分は0.0〜1.0。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct クリアカラー {
    赤: f32,
    緑: f32,
    青: f32,
    不透明度: f32,
}

impl クリアカラー {
    /// 各成分が0.0〜1.0であることを検証して生成する。
    pub fn 生成する(赤: f32, 緑: f32, 青: f32, 不透明度: f32) -> Result<Self, クリアカラーエラー> {
        for 成分 in [赤, 緑, 青, 不透明度] {
            if !(0.0..=1.0).contains(&成分) {
                return Err(クリアカラーエラー(成分));
            }
        }
        Ok(Self { 赤, 緑, 青, 不透明度 })
    }

    /// GPU境界向けの変換: VulkanのRGBA配列表現に変換する。
    pub(crate) fn rgba配列(&self) -> [f32; 4] {
        [self.赤, self.緑, self.青, self.不透明度]
    }
}
