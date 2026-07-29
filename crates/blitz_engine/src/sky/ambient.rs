//! 時刻から導く環境光の強さ。

use super::天空状態エラー;

/// 空全体から回り込む光の強さ。0以上1以下の係数であり、レンダラー境界の環境光係数へはライティング入力を組み立てる段で変換する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 環境光強度(f32);

impl 環境光強度 {
    pub fn 生成する(値: f32) -> Result<Self, 天空状態エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(天空状態エラー::値域外("環境光強度", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }

    /// 自身を係数0、相手を係数1として線形に混ぜる。両端が値域内であるため結果も値域内に収まる。
    pub fn 混ぜる(&self, 相手: &Self, 係数: f32) -> Result<Self, 天空状態エラー> {
        Self::生成する(self.0 + (相手.0 - self.0) * 係数)
    }
}
