//! 高さ場が覆うチャンクの矩形を求める工程。受け取るのはチャンク座標の一覧、返すのは最小と最大の座標である。
//!
//! 矩形であることを先に確かめるのは、貼り合わせた格子が1つの長方形の標本の並びになるためである。
//! 隙間があるチャンク集合を許すと、標本の並びに穴を無言で埋める既定値が要る。

use std::collections::HashSet;

use blitz_engine::チャンク座標;

use super::error::高さ場コンパイルエラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct チャンクの矩形 {
    pub(super) 最小: チャンク座標,
    pub(super) 最大: チャンク座標,
}

impl チャンクの矩形 {
    /// 座標一覧が隙間も重複もない矩形をなすことを確かめて、その矩形を返す。
    pub(super) fn 座標一覧から求める(座標一覧: &[チャンク座標]) -> Result<Self, 高さ場コンパイルエラー> {
        let (先頭, 残り) = 座標一覧.split_first().ok_or(高さ場コンパイルエラー::高さ格子が1件も無い)?;
        let mut 最小 = *先頭;
        let mut 最大 = *先頭;
        let mut 既出 = HashSet::new();
        既出.insert((先頭.x(), 先頭.z()));
        for 座標 in 残り {
            if !既出.insert((座標.x(), 座標.z())) {
                return Err(高さ場コンパイルエラー::チャンク座標が重複している {
                    東: 座標.x(), 南: 座標.z()
                });
            }
            最小 = チャンク座標::生成する(最小.x().min(座標.x()), 最小.z().min(座標.z()));
            最大 = チャンク座標::生成する(最大.x().max(座標.x()), 最大.z().max(座標.z()));
        }
        let 矩形 = Self { 最小, 最大 };
        矩形.欠けが無いことを確かめる(&既出)?;
        Ok(矩形)
    }

    /// 東西と南北それぞれのチャンク数。矩形の検査を通った後は1以上である。
    pub(super) fn 東方向のチャンク数(&self) -> Result<u32, 高さ場コンパイルエラー> {
        方向のチャンク数(self.最小.x(), self.最大.x())
    }

    pub(super) fn 南方向のチャンク数(&self) -> Result<u32, 高さ場コンパイルエラー> {
        方向のチャンク数(self.最小.z(), self.最大.z())
    }

    fn 欠けが無いことを確かめる(&self, 既出: &HashSet<(i32, i32)>) -> Result<(), 高さ場コンパイルエラー> {
        for 南 in self.最小.z()..=self.最大.z() {
            for 東 in self.最小.x()..=self.最大.x() {
                if !既出.contains(&(東, 南)) {
                    return Err(高さ場コンパイルエラー::チャンクの矩形に欠けがある { 東, 南 });
                }
            }
        }
        Ok(())
    }
}

fn 方向のチャンク数(最小: i32, 最大: i32) -> Result<u32, 高さ場コンパイルエラー> {
    最大
        .checked_sub(最小)
        .and_then(|差| u32::try_from(差).ok())
        .and_then(|差| 差.checked_add(1))
        .ok_or(高さ場コンパイルエラー::格子点数が表現できない)
}
