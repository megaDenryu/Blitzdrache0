//! チャンク台帳の入力重複と状態遷移違反。

use crate::チャンクID;

use super::chunk_state::チャンク状態;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum チャンク台帳エラー {
    #[error("必要集合でチャンクID {0:?}が重複した")]
    必要ID重複(チャンクID),
    #[error("チャンクID {0:?}は台帳に存在しない")]
    未登録(チャンクID),
    #[error("チャンクID {id:?}は状態{実際:?}から要求された遷移を行えない")]
    状態遷移不正 { id: チャンクID, 実際: チャンク状態 },
}
