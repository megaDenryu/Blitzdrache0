//! 表示距離の決定: カメラの遮蔽と復帰が1描画に1度答える、表示距離とその判定の対。

use blitz_math::メートル;

use crate::occlusion_verdict::遮蔽の判定;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 表示距離の決定 {
    表示距離: メートル,
    判定: 遮蔽の判定,
}

impl 表示距離の決定 {
    pub(crate) fn 生成する(表示距離: メートル, 判定: 遮蔽の判定) -> Self {
        Self { 表示距離, 判定 }
    }

    /// エンジンのカメラへ据える距離。理想距離を越えず、表示距離の下限を割らない。
    pub fn 表示距離(&self) -> メートル {
        self.表示距離
    }

    pub fn 判定(&self) -> 遮蔽の判定 {
        self.判定
    }
}
