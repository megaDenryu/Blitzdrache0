//! チャンク目録への登録で返す型付きエラー。

use crate::チャンク座標;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum チャンク目録エラー {
    #[error("チャンク目録に座標(x={}, z={})が二重に登録された", .0.x(), .0.z())]
    座標重複(チャンク座標),
}
