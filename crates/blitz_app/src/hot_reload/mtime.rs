//! 監視対象ファイルの最終更新時刻(mtime)取得。

use std::path::Path;
use std::time::SystemTime;

pub(super) fn 取得する(パス: &Path) -> std::io::Result<SystemTime> {
    パス.metadata()?.modified()
}
