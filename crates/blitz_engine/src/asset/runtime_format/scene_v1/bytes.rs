//! 版1内容の範囲検査付きリトルエンディアン読み書き。

use super::super::アセット実行時形式エラー;

pub(super) struct 書込先(Vec<u8>);

impl 書込先 {
    pub(super) fn 新規() -> Self {
        Self(Vec::new())
    }
    pub(super) fn 完了する(self) -> Vec<u8> {
        self.0
    }
    pub(super) fn u8(&mut self, 値: u8) {
        self.0.push(値);
    }
    pub(super) fn u32(&mut self, 値: u32) {
        self.0.extend_from_slice(&値.to_le_bytes());
    }
    pub(super) fn u64(&mut self, 値: u64) {
        self.0.extend_from_slice(&値.to_le_bytes());
    }
    pub(super) fn f32(&mut self, 値: f32) -> Result<(), アセット実行時形式エラー> {
        if !値.is_finite() {
            return Err(アセット実行時形式エラー::非有限小数);
        }
        self.0.extend_from_slice(&値.to_le_bytes());
        Ok(())
    }
    pub(super) fn 件数(&mut self, 値: usize) -> Result<(), アセット実行時形式エラー> {
        let 値 = u32::try_from(値).map_err(|_| アセット実行時形式エラー::件数表現不能)?;
        self.u32(値);
        Ok(())
    }
    pub(super) fn バイト列(&mut self, 値: &[u8]) {
        self.0.extend_from_slice(値);
    }
}

pub(super) struct 読取位置<'a>(&'a [u8]);

impl<'a> 読取位置<'a> {
    pub(super) fn 新規(バイト列: &'a [u8]) -> Self {
        Self(バイト列)
    }
    pub(super) fn u8(&mut self) -> Result<u8, アセット実行時形式エラー> {
        Ok(self.固定::<1>()?[0])
    }
    pub(super) fn u32(&mut self) -> Result<u32, アセット実行時形式エラー> {
        Ok(u32::from_le_bytes(self.固定()?))
    }
    pub(super) fn u64(&mut self) -> Result<u64, アセット実行時形式エラー> {
        Ok(u64::from_le_bytes(self.固定()?))
    }
    pub(super) fn f32(&mut self) -> Result<f32, アセット実行時形式エラー> {
        let 値 = f32::from_le_bytes(self.固定()?);
        if 値.is_finite() {
            Ok(値)
        } else {
            Err(アセット実行時形式エラー::非有限小数)
        }
    }
    pub(super) fn 件数(&mut self) -> Result<usize, アセット実行時形式エラー> {
        let 宣言件数 = usize::try_from(self.u32()?).map_err(|_| アセット実行時形式エラー::長さ表現不能)?;
        if 宣言件数 > self.0.len() {
            return Err(アセット実行時形式エラー::件数過大 {
                宣言件数,
                残りバイト数: self.0.len(),
            });
        }
        Ok(宣言件数)
    }
    pub(super) fn バイト列(&mut self, 長さ: usize) -> Result<&'a [u8], アセット実行時形式エラー> {
        let (値, 残り) = self.0.split_at_checked(長さ).ok_or(アセット実行時形式エラー::内容終端)?;
        self.0 = 残り;
        Ok(値)
    }
    pub(super) fn 完了を検査する(self) -> Result<(), アセット実行時形式エラー> {
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
