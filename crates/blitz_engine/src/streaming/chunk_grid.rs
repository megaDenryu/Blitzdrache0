//! 大域位置を格子へ写し、中心から外周の順で正方形の必要集合を作る。

use blitz_math::{大域メートル, 大域ワールド位置};

use super::{chunk_request::チャンク要求, error::チャンク格子エラー};
use crate::チャンク座標;

#[derive(Debug, Clone, Copy)]
pub struct チャンク格子 {
    一辺メートル: f64,
}

impl チャンク格子 {
    pub fn 生成する(一辺: 大域メートル) -> Result<Self, チャンク格子エラー> {
        let 値 = 一辺.値();
        if !値.is_finite() || 値 <= 0.0 {
            return Err(チャンク格子エラー::一辺不正);
        }
        Ok(Self { 一辺メートル: 値 })
    }

    pub fn 必要集合を計算する(
        self,
        プレイヤー位置: 大域ワールド位置,
        先読み半径: u8,
    ) -> Result<Vec<チャンク要求>, チャンク格子エラー> {
        let 中心 = self.位置を座標へ変換する(プレイヤー位置)?;
        範囲を検査する(中心, 先読み半径)?;
        let 一辺件数 = usize::from(先読み半径) * 2 + 1;
        let mut 結果 = Vec::with_capacity(一辺件数 * 一辺件数);
        結果.push(チャンク要求::生成する(中心, 0));
        for 距離 in 1..=先読み半径 {
            外周を追加する(&mut 結果, 中心, 距離);
        }
        Ok(結果)
    }

    fn 位置を座標へ変換する(self, 位置: 大域ワールド位置) -> Result<チャンク座標, チャンク格子エラー> {
        let x = 軸座標へ変換する(位置.x().値(), self.一辺メートル)?;
        let z = 軸座標へ変換する(位置.z().値(), self.一辺メートル)?;
        Ok(チャンク座標::生成する(x, z))
    }
}

fn 範囲を検査する(中心: チャンク座標, 半径: u8) -> Result<(), チャンク格子エラー> {
    let 半径 = i32::from(半径);
    for 値 in [中心.x(), 中心.z()] {
        if 値.checked_sub(半径).is_none() || 値.checked_add(半径).is_none() {
            return Err(チャンク格子エラー::座標範囲外);
        }
    }
    Ok(())
}

fn 外周を追加する(出力: &mut Vec<チャンク要求>, 中心: チャンク座標, 距離: u8) {
    let 半径 = i32::from(距離);
    for z差 in -半径..=半径 {
        for x差 in -半径..=半径 {
            if x差.abs().max(z差.abs()) == 半径 {
                出力.push(チャンク要求::生成する(
                    チャンク座標::生成する(中心.x() + x差, 中心.z() + z差),
                    距離,
                ));
            }
        }
    }
}

fn 軸座標へ変換する(位置: f64, 一辺: f64) -> Result<i32, チャンク格子エラー> {
    if !位置.is_finite() {
        return Err(チャンク格子エラー::位置非有限);
    }
    let 床値 = (位置 / 一辺).floor();
    if 床値 < f64::from(i32::MIN) || 床値 > f64::from(i32::MAX) {
        return Err(チャンク格子エラー::座標範囲外);
    }
    整数f64をi32へ変換する(床値).ok_or(チャンク格子エラー::座標範囲外)
}

fn 整数f64をi32へ変換する(値: f64) -> Option<i32> {
    let 負 = 値.is_sign_negative();
    let mut 残り = 値.abs();
    let mut 絶対値 = 0_u32;
    for ビット in (0_u32..32).rev() {
        let 重み = 1_u32 << ビット;
        if 残り >= f64::from(重み) {
            絶対値 |= 重み;
            残り -= f64::from(重み);
        }
    }
    if 負 && 絶対値 == 2_147_483_648 {
        Some(i32::MIN)
    } else {
        let 正値 = i32::try_from(絶対値).ok()?;
        Some(if 負 { -正値 } else { 正値 })
    }
}
