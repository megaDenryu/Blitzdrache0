//! アニメーションチャンネル: 昇順の時刻列とそれに対応する値列を補間種別つきで持つ。

use blitz_math::秒;

use super::interpolation_kind::補間種別;

/// `時刻列`と`値列`は同じ長さで、`時刻列`は昇順(glTFのサンプラー入力の規約)。
#[derive(Debug, Clone, PartialEq)]
pub struct チャンネル<値> {
    pub 時刻列: Vec<秒>,
    pub 値列: Vec<値>,
    pub 補間: 補間種別,
}
