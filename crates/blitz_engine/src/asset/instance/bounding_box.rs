//! 軸に平行な直方体の包囲領域。インスタンス群の全個体をまとめて覆う保守的な境界として使う。
//! 保持するのは最小と最大の2点だけであり、最小が最大を上回らないことをこの型が保つ。

use super::error::{インスタンス群エラー, 軸名一覧};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 軸平行包囲領域 {
    最小: [f32; 3],
    最大: [f32; 3],
}

impl 軸平行包囲領域 {
    pub fn 生成する(最小: [f32; 3], 最大: [f32; 3]) -> Result<Self, インスタンス群エラー> {
        for (軸, (最小値, 最大値)) in 軸名一覧.into_iter().zip(最小.into_iter().zip(最大)) {
            if !最小値.is_finite() || !最大値.is_finite() {
                return Err(インスタンス群エラー::非有限成分 { 成分: "包囲領域" });
            }
            if 最小値 > 最大値 {
                return Err(インスタンス群エラー::包囲領域の順序違反 { 軸 });
            }
        }
        Ok(Self { 最小, 最大 })
    }

    pub fn 最小(&self) -> [f32; 3] {
        self.最小
    }

    pub fn 最大(&self) -> [f32; 3] {
        self.最大
    }
}
