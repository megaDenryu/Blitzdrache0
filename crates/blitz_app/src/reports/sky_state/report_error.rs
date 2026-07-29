//! 天空状態報告が途中で止まる原因。描画を伴わない報告のため起動エラーへは合流させず、この報告の中で閉じる。

use blitz_engine::sky::天空状態エラー;
use blitz_engine::time::時刻エラー;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum 天空状態報告エラー {
    #[error("報告する時刻を作れなかった: {0}")]
    時刻不正(#[from] 時刻エラー),

    #[error("天空状態を導けなかった: {0}")]
    天空状態導出失敗(#[from] 天空状態エラー),
}
