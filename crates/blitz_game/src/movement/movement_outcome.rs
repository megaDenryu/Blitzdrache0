//! 一刻みの移動の結果: 胴体の移動が1刻みの終わりに答える、足元の大域位置・移動状態・移動の観測の束。

use blitz_math::大域ワールド位置;

use super::movement_observation::移動の観測;
use super::movement_state::移動状態;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 一刻みの移動の結果 {
    足元: 大域ワールド位置,
    移動状態: 移動状態,
    観測: 移動の観測,
}

impl 一刻みの移動の結果 {
    pub(crate) fn 生成する(足元: 大域ワールド位置, 移動状態: 移動状態, 観測: 移動の観測) -> Self {
        Self {
            足元, 移動状態, 観測
        }
    }

    pub fn 足元(&self) -> 大域ワールド位置 {
        self.足元
    }

    pub fn 移動状態(&self) -> 移動状態 {
        self.移動状態
    }

    pub fn 観測(&self) -> 移動の観測 {
        self.観測
    }
}
