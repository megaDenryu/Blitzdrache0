//! 受光面の1つの矩形と、その中の影の数え上げ。受け取るのは矩形と比較結果、返すのはその矩形だけの集計である。
//!
//! 全画面の合計では「遠方キャスターの影が消えて近距離対照の影が残った」と「近距離対照の影が消えて別の影が残った」を
//! 区別できない。どちらも「一部が消え一部が残った」になるためである。落ちる場所で分けて数えるのがこの工程の役目であり、
//! どの矩形をどう解釈するかは`range_world`が持つ。

use super::compare::比較結果;

/// 受光面の画素矩形。終端は含まない。
pub(super) struct 受光面の領域 {
    x開始: usize,
    x終端: usize,
    y開始: usize,
    y終端: usize,
}

pub(super) struct 領域の集計 {
    pub(super) 基準の影画素数: u64,
    pub(super) 候補の影画素数: u64,
    pub(super) 欠落画素数: u64,
}

impl 受光面の領域 {
    pub(super) const fn 生成する(x開始: usize, x終端: usize, y開始: usize, y終端: usize) -> Self {
        Self {
            x開始, x終端, y開始, y終端
        }
    }

    pub(super) fn 数える(&self, 比較: &比較結果) -> Result<領域の集計, String> {
        let mut 集計 = 領域の集計 {
            基準の影画素数: 0,
            候補の影画素数: 0,
            欠落画素数: 0,
        };
        for y in self.y開始..self.y終端 {
            for x in self.x開始..self.x終端 {
                let 添字 = y * 比較.幅 + x;
                let (Some(基準の影), Some(候補の影)) = (比較.基準の影の印.get(添字), 比較.候補の影の印.get(添字)) else {
                    return Err(format!("受光面の領域が読み戻し画像({}×{})の外にある", 比較.幅, 比較.高さ));
                };
                集計.基準の影画素数 += u64::from(*基準の影);
                集計.候補の影画素数 += u64::from(*候補の影);
                集計.欠落画素数 += u64::from(*基準の影 && !*候補の影);
            }
        }
        Ok(集計)
    }
}
