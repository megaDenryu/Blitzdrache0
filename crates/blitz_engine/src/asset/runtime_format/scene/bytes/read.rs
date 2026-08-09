//! 版1内容を境界外へ進めず、宣言件数の最小バイト数を確保前に検査する。

use super::super::super::アセット実行時形式エラー;

pub(crate) struct 読取位置<'a>(&'a [u8]);

impl<'a> 読取位置<'a> {
    pub(crate) fn 新規(バイト列: &'a [u8]) -> Self {
        Self(バイト列)
    }
    pub(crate) fn u8(&mut self) -> Result<u8, アセット実行時形式エラー> {
        Ok(self.固定::<1>()?[0])
    }
    pub(crate) fn u16(&mut self) -> Result<u16, アセット実行時形式エラー> {
        Ok(u16::from_le_bytes(self.固定()?))
    }
    pub(crate) fn u32(&mut self) -> Result<u32, アセット実行時形式エラー> {
        Ok(u32::from_le_bytes(self.固定()?))
    }
    pub(crate) fn u64(&mut self) -> Result<u64, アセット実行時形式エラー> {
        Ok(u64::from_le_bytes(self.固定()?))
    }
    pub(crate) fn f32(&mut self) -> Result<f32, アセット実行時形式エラー> {
        let 値 = f32::from_le_bytes(self.固定()?);
        if 値.is_finite() {
            Ok(値)
        } else {
            Err(アセット実行時形式エラー::非有限小数)
        }
    }
    pub(crate) fn f64(&mut self) -> Result<f64, アセット実行時形式エラー> {
        let 値 = f64::from_le_bytes(self.固定()?);
        if 値.is_finite() {
            Ok(値)
        } else {
            Err(アセット実行時形式エラー::非有限小数)
        }
    }
    pub(crate) fn 件数(&mut self, 最小要素長: usize) -> Result<usize, アセット実行時形式エラー> {
        let 宣言件数 = usize::try_from(self.u32()?).map_err(|_| アセット実行時形式エラー::長さ表現不能)?;
        self.宣言件数を確保前に検査する(宣言件数, 最小要素長)?;
        Ok(宣言件数)
    }
    /// 件数を1つのu32で持たない形式のための確保前の検査。高さ場のように件数を2つの数の積で持つ形式が、
    /// 積を求めてからこれを呼ぶ。
    pub(crate) fn 宣言件数を確保前に検査する(
        &self,
        宣言件数: usize,
        最小要素長: usize,
    ) -> Result<(), アセット実行時形式エラー> {
        let 最小長 = 宣言件数.checked_mul(最小要素長).ok_or(アセット実行時形式エラー::長さ表現不能)?;
        if 最小長 > self.0.len() {
            return Err(アセット実行時形式エラー::件数過大 {
                宣言件数,
                残りバイト数: self.0.len(),
            });
        }
        Ok(())
    }
    pub(crate) fn usize(&mut self) -> Result<usize, アセット実行時形式エラー> {
        usize::try_from(self.u32()?).map_err(|_| アセット実行時形式エラー::長さ表現不能)
    }
    pub(crate) fn バイト列(&mut self, 長さ: usize) -> Result<&'a [u8], アセット実行時形式エラー> {
        let (値, 残り) = self.0.split_at_checked(長さ).ok_or(アセット実行時形式エラー::内容終端)?;
        self.0 = 残り;
        Ok(値)
    }
    pub(crate) fn 完了を検査する(self) -> Result<(), アセット実行時形式エラー> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(アセット実行時形式エラー::未読末尾(self.0.len()))
        }
    }
    fn 固定<const 長さ: usize>(&mut self) -> Result<[u8; 長さ], アセット実行時形式エラー> {
        let 値 = self.バイト列(長さ)?;
        let mut 結果 = [0; 長さ];
        結果.copy_from_slice(値);
        Ok(結果)
    }
}
