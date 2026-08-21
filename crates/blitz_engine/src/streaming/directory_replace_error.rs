//! 実行中のストリーミング調停へ新しいチャンク目録を差し替えるときの失敗。

use super::チャンク一辺;

#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum チャンク目録差し替えエラー {
    #[error("実行中のチャンク一辺は変更できない(旧={旧}, 新={新})")]
    チャンク一辺変更 { 旧: チャンク一辺, 新: チャンク一辺 },
}
