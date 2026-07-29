//! 一日の長さ。天空状態が巡る周期であり、時刻だけでは決まらないためシーン方針が持つ。

use blitz_math::秒;

use super::時刻エラー;

/// 一日が何秒で一巡するかを表す正の有限な長さ。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 一日の長さ {
    秒数: 秒,
}

/// 地球の一日と同じ24時間の秒数。世界ごとに変えられるため定数ではなく既定値として置く。
const 地球の一日の秒数: f32 = 86_400.0;

impl 一日の長さ {
    pub fn 生成する(秒数: 秒) -> Result<Self, 時刻エラー> {
        if !秒数.値().is_finite() || 秒数.値() <= 0.0 {
            return Err(時刻エラー::一日の長さが正でない);
        }
        Ok(Self { 秒数 })
    }

    /// 地球の一日と同じ24時間。
    pub fn 地球の一日() -> Self {
        Self {
            秒数: 秒::生成する(地球の一日の秒数),
        }
    }

    pub fn 秒数(&self) -> 秒 {
        self.秒数
    }
}
