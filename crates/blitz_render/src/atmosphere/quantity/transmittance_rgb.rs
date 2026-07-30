//! 経路を通り抜けた光が元の何割残るかを、赤・緑・青の3成分で表した割合。

use crate::atmosphere::大気数学エラー;

/// 経路の光学的深さの指数の負を取った値。1が完全に透明、0が完全に遮られた状態である。
/// 不変条件: 3成分とも有限かつ0以上1以下である。指数関数の像が正の実数であるため、
/// 光学的深さが有限かつ0以上である限りこの範囲を出ない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 透過率RGB {
    成分: [f32; 3],
}

impl 透過率RGB {
    pub fn 生成する(赤: f32, 緑: f32, 青: f32) -> Result<Self, 大気数学エラー> {
        let 成分 = [赤, 緑, 青];
        for 値 in 成分 {
            if !値.is_finite() || !(0.0..=1.0).contains(&値) {
                return Err(大気数学エラー::値域外("透過率RGB", 値));
            }
        }
        Ok(Self { 成分 })
    }

    /// 前提: 3成分とも0以上1以下である。光学的深さの指数からの導出だけがこの入口を使う。
    pub(in crate::atmosphere) fn 検証済みの成分から(成分: [f32; 3]) -> Self {
        Self { 成分 }
    }

    pub fn 成分(&self) -> [f32; 3] {
        self.成分
    }
}
