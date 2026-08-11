//! 描画対象数: 計測用に生成する描画対象の件数。

use thiserror::Error;

/// 描画対象数が受け取れない値だったこと。
#[derive(Debug, Error)]
pub(crate) enum 描画対象数エラー {
    #[error("1以上でなければならない")]
    零件は受けない,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 描画対象数(u32);

impl 描画対象数 {
    pub(crate) fn 生成する(値: u32) -> Result<Self, 描画対象数エラー> {
        if 値 == 0 {
            Err(描画対象数エラー::零件は受けない)
        } else {
            Ok(Self(値))
        }
    }

    pub(crate) fn usize値(self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("描画対象数がusizeに収まらない: {}", self.0))
    }
}
