//! 方向光シャドウに使うワールドから光源クリップへの変換。

use blitz_math::{ワールド, 光源クリップ, 変換};

use super::ライティング入力エラー;

#[derive(Debug, Clone, Copy)]
pub struct 影入力(変換<ワールド, 光源クリップ>);

impl 影入力 {
    pub fn 生成する(ライトビュー射影: 変換<ワールド, 光源クリップ>) -> Result<Self, ライティング入力エラー> {
        let 行列 = ライトビュー射影.gpu境界用列優先配列();
        if 行列.iter().flatten().any(|成分| !成分.is_finite()) {
            return Err(ライティング入力エラー::影変換不正);
        }
        Ok(Self(ライトビュー射影))
    }

    pub(crate) fn ライトビュー射影(self) -> 変換<ワールド, 光源クリップ> {
        self.0
    }
}
