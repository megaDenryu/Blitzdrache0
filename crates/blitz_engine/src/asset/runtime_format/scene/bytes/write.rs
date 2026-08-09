//! 版1内容の小数検査と整数のリトルエンディアン書き込み。

use super::super::super::アセット実行時形式エラー;

pub(crate) struct 書込先(Vec<u8>);

impl 書込先 {
    pub(crate) fn 新規() -> Self {
        Self(Vec::new())
    }
    pub(crate) fn 完了する(self) -> Vec<u8> {
        self.0
    }
    pub(crate) fn u8(&mut self, 値: u8) {
        self.0.push(値);
    }
    pub(crate) fn u16(&mut self, 値: u16) {
        self.0.extend_from_slice(&値.to_le_bytes());
    }
    pub(crate) fn u32(&mut self, 値: u32) {
        self.0.extend_from_slice(&値.to_le_bytes());
    }
    pub(crate) fn u64(&mut self, 値: u64) {
        self.0.extend_from_slice(&値.to_le_bytes());
    }
    pub(crate) fn f32(&mut self, 値: f32) -> Result<(), アセット実行時形式エラー> {
        if !値.is_finite() {
            return Err(アセット実行時形式エラー::非有限小数);
        }
        self.0.extend_from_slice(&値.to_le_bytes());
        Ok(())
    }
    pub(crate) fn f64(&mut self, 値: f64) -> Result<(), アセット実行時形式エラー> {
        if !値.is_finite() {
            return Err(アセット実行時形式エラー::非有限小数);
        }
        self.0.extend_from_slice(&値.to_le_bytes());
        Ok(())
    }
    pub(crate) fn 件数(&mut self, 値: usize) -> Result<(), アセット実行時形式エラー> {
        let 値 = u32::try_from(値).map_err(|_| アセット実行時形式エラー::件数表現不能)?;
        self.u32(値);
        Ok(())
    }
    pub(crate) fn バイト列(&mut self, 値: &[u8]) {
        self.0.extend_from_slice(値);
    }
}
