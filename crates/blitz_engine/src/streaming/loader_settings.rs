//! チャンク読込器の並列度と有界キューの容量を表す起動設定。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct チャンク読込設定 {
    ワーカー本数: usize,
    要求キュー容量: usize,
    完了キュー容量: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum チャンク読込設定エラー {
    #[error("チャンク読込ワーカーの本数は1以上でなければならない")]
    ワーカー本数が零,
    #[error("チャンク読込の要求キュー容量は1以上でなければならない")]
    要求キュー容量が零,
    #[error("チャンク読込の完了キュー容量は1以上でなければならない")]
    完了キュー容量が零,
}

impl チャンク読込設定 {
    pub const fn 生成する(
        ワーカー本数: usize, 要求キュー容量: usize, 完了キュー容量: usize
    ) -> Result<Self, チャンク読込設定エラー> {
        if ワーカー本数 == 0 {
            Err(チャンク読込設定エラー::ワーカー本数が零)
        } else if 要求キュー容量 == 0 {
            Err(チャンク読込設定エラー::要求キュー容量が零)
        } else if 完了キュー容量 == 0 {
            Err(チャンク読込設定エラー::完了キュー容量が零)
        } else {
            Ok(Self {
                ワーカー本数,
                要求キュー容量,
                完了キュー容量,
            })
        }
    }

    pub const fn 既定値() -> Self {
        Self {
            ワーカー本数: 1,
            要求キュー容量: 64,
            完了キュー容量: 4,
        }
    }

    pub(super) const fn ワーカー本数(self) -> usize {
        self.ワーカー本数
    }

    pub(super) const fn 要求キュー容量(self) -> usize {
        self.要求キュー容量
    }

    pub(super) const fn 完了キュー容量(self) -> usize {
        self.完了キュー容量
    }
}
