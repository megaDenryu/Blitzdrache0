//! 版付きカタログのソース依存一覧と更新時刻を監視状態へ変換する。

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use blitz_engine::実行時形式からカタログを読む;

pub(super) struct 監視状態 {
    一覧: Vec<(PathBuf, Option<SystemTime>)>,
}

impl 監視状態 {
    pub(super) fn 読み込む(カタログパス: &Path) -> Result<Self, String> {
        let バイト列 = std::fs::read(カタログパス).map_err(|誤り| format!("{}: {誤り}", カタログパス.display()))?;
        let カタログ = 実行時形式からカタログを読む(&バイト列).map_err(|誤り| 誤り.to_string())?;
        let mut 重複排除 = HashSet::new();
        for (_, 項目) in カタログ.全項目を走査する() {
            重複排除.extend(項目.ソース依存一覧().iter().cloned());
        }
        let mut パス一覧: Vec<_> = 重複排除.into_iter().collect();
        パス一覧.sort();
        let 一覧 = パス一覧
            .into_iter()
            .map(|パス| {
                let 時刻 = 更新時刻(&パス);
                (パス, 時刻)
            })
            .collect();
        Ok(Self { 一覧 })
    }

    pub(super) fn 変更されたか(&self) -> bool {
        self.一覧.iter().any(|(パス, 記録)| 更新時刻(パス) != *記録)
    }

    pub(super) fn 件数(&self) -> usize {
        self.一覧.len()
    }
}

fn 更新時刻(パス: &Path) -> Option<SystemTime> {
    std::fs::metadata(パス).and_then(|値| 値.modified()).ok()
}
