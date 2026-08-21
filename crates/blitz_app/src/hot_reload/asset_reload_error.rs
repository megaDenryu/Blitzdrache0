//! 公開済みの実行時アセット一式を読み直す境界で起こる失敗。

#[derive(Debug, thiserror::Error)]
pub(crate) enum 実行時アセット一式の再読込エラー {
    #[error("実行時カタログを読み直せなかった: {0}")]
    カタログ(#[from] blitz_engine::実行時カタログ読込エラー),
    #[error("実行時チャンク目録を読み直せなかった: {0}")]
    チャンク目録(#[from] blitz_engine::実行時チャンク目録読込エラー),
    #[error("起動時シーンを読み直せなかった: {0}")]
    起動時シーン(#[from] blitz_engine::実行時シーン読込エラー),
}
