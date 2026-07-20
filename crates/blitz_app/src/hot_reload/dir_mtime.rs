//! シェーダーディレクトリ内の.slangファイル群から最新の更新時刻を求める。
//! ホットリロードの監視対象を「エントリファイル単体」から「ディレクトリ内の
//! 全.slangファイル」へ拡張するための走査ロジック(importでpbr.slang等へ
//! 分割された関数群の変更も検知する必要があるため)。

use std::path::Path;
use std::time::SystemTime;

use super::mtime;

/// ディレクトリ直下の.slangファイルを走査し、最新の更新時刻を返す。
/// ディレクトリが読めない・対象ファイルが1つも無い場合は`None`。
pub(super) fn 最新更新時刻を取得する(ディレクトリ: &Path) -> Option<SystemTime> {
    let 読み取り結果 = std::fs::read_dir(ディレクトリ).ok()?;
    let 時刻一覧: Vec<SystemTime> = 読み取り結果
        .filter_map(|エントリ結果| エントリ結果.ok())
        .map(|エントリ| エントリ.path())
        .filter(|パス| パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("slang"))
        .filter_map(|パス| mtime::取得する(&パス).ok())
        .collect();
    最大値を求める(&時刻一覧)
}

/// 時刻一覧から最大値を求める純粋ロジック(単体テスト対象)。
pub(super) fn 最大値を求める(時刻一覧: &[SystemTime]) -> Option<SystemTime> {
    時刻一覧.iter().copied().max()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn 空なら未設定を返す() {
        assert!(最大値を求める(&[]).is_none());
    }

    #[test]
    fn 最も新しい時刻を返す() {
        let 基準 = SystemTime::UNIX_EPOCH;
        let 古い = 基準 + Duration::from_secs(10);
        let 新しい = 基準 + Duration::from_secs(20);
        let 結果 = 最大値を求める(&[古い, 新しい, 基準]).unwrap();
        assert_eq!(結果, 新しい);
    }

    #[test]
    fn 順序を入れ替えても最大値は変わらない() {
        let 基準 = SystemTime::UNIX_EPOCH;
        let a = 基準 + Duration::from_secs(5);
        let b = 基準 + Duration::from_secs(1);
        assert_eq!(最大値を求める(&[a, b]), 最大値を求める(&[b, a]));
    }
}
