//! `--project <ルート>`の引数からプロジェクトルートを解く工程。受け取るのは`editor_server`へ渡す追加引数と
//! 既定ルート、返すのは開くプロジェクトのルートである。
//!
//! `editor_server`と同じ規則をここでも解くのは、カタログの書き出しが編集サーバーと同じ格子の置き場を読むためである。
//! 引数を素通しするだけにすると、使い捨てのプロジェクトを開いた実行がリポジトリ側の格子でカタログを組み、
//! 画面に出る建物と保存先の建物が食い違う。
//! 参照: `crates/editor_server/src/project_root.rs`

use std::path::{Path, PathBuf};

pub(super) fn プロジェクトルートを引数から解く(引数一覧: &[String], 既定ルート: &Path) -> PathBuf {
    引数一覧
        .iter()
        .position(|引数| 引数 == "--project")
        .and_then(|添字| 引数一覧.get(添字 + 1))
        .map_or_else(|| 既定ルート.to_path_buf(), PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectの指定が無ければ既定ルートを使う() {
        assert_eq!(プロジェクトルートを引数から解く(&[], Path::new("/repo")), PathBuf::from("/repo"));
    }

    #[test]
    fn projectの次の引数をプロジェクトルートにする() {
        let 引数一覧 = vec!["--project".to_string(), "/game".to_string()];
        assert_eq!(プロジェクトルートを引数から解く(&引数一覧, Path::new("/repo")), PathBuf::from("/game"));
    }
}
