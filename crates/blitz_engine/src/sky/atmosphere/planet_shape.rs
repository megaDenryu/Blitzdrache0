//! 大気を載せた球の形。視線と大気上端・地表の交差はこの2つの半径だけで決まる。

use blitz_math::メートル;

use crate::sky::天空状態エラー;

/// 惑星中心から地表までの距離が6360km、大気上端までが6460kmという地球の値。
/// 参照: Sebastien Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020)の地球パラメータ。
const 地球の下端半径のメートル: f32 = 6_360_000.0;
const 地球の上端半径のメートル: f32 = 6_460_000.0;

/// 惑星中心から測った地表と大気上端の半径。
/// 不変条件: どちらも有限かつ正であり、下端半径は上端半径より小さい。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 惑星形状 {
    下端半径: メートル,
    上端半径: メートル,
}

impl 惑星形状 {
    pub fn 生成する(下端半径: メートル, 上端半径: メートル) -> Result<Self, 天空状態エラー> {
        for (項目, 半径) in [("惑星下端半径", 下端半径), ("大気上端半径", 上端半径)] {
            if !半径.値().is_finite() || 半径.値() <= 0.0 {
                return Err(天空状態エラー::値域外(項目, 半径.値()));
            }
        }
        if 下端半径 >= 上端半径 {
            return Err(天空状態エラー::値域外("大気上端半径", 上端半径.値()));
        }
        Ok(Self { 下端半径, 上端半径 })
    }

    pub fn 地球標準() -> Self {
        Self {
            下端半径: メートル::生成する(地球の下端半径のメートル),
            上端半径: メートル::生成する(地球の上端半径のメートル),
        }
    }

    pub fn 下端半径(&self) -> メートル {
        self.下端半径
    }

    pub fn 上端半径(&self) -> メートル {
        self.上端半径
    }

    pub fn 大気の厚み(&self) -> メートル {
        self.上端半径 - self.下端半径
    }
}
