//! 地表が入射光を跳ね返す割合を、赤・緑・青の3成分で表した反射率。

use crate::atmosphere::大気数学エラー;

/// 大気の下端で光を反射する地面の反射率。多重散乱の計算が地面からの照り返しを取り込むときの入力になる。
/// 不変条件: 3成分とも有限かつ0以上1以下である。1を超える反射率は入射より多くの光を返し、多重散乱の等比級数が発散する。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 地表アルベドRGB {
    成分: [f32; 3],
}

impl 地表アルベドRGB {
    pub fn 生成する(赤: f32, 緑: f32, 青: f32) -> Result<Self, 大気数学エラー> {
        let 成分 = [赤, 緑, 青];
        for 値 in 成分 {
            if !値.is_finite() || !(0.0..=1.0).contains(&値) {
                return Err(大気数学エラー::値域外("地表アルベドRGB", 値));
            }
        }
        Ok(Self { 成分 })
    }

    pub fn 成分(&self) -> [f32; 3] {
        self.成分
    }
}
