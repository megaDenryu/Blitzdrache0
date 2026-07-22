//! アセットのmtime監視。カタログ+選択idを保持し、シーンの参照ファイル一覧
//! (ローダが実際に読んだファイル群)のmtimeが1つでも進んでいたら再読込する。

use std::path::PathBuf;
use std::time::SystemTime;

use blitz_engine::{アセットID, アセットエラー, カタログ, シーンを読み込む, シーンデータ};

use super::mtime;

pub(super) struct アセット監視状態 {
    カタログ: カタログ,
    id: アセットID,
    参照ファイル一覧: Vec<(PathBuf, SystemTime)>,
}

pub(super) fn 構築する(カタログ: カタログ, id: アセットID, 参照ファイル一覧: &[PathBuf]) -> アセット監視状態 {
    アセット監視状態 {
        カタログ,
        id,
        参照ファイル一覧: 時刻一覧を取得する(参照ファイル一覧),
    }
}

impl アセット監視状態 {
    /// 参照ファイルのいずれかが変化していれば再読込を試み、結果を返す。
    /// 変化が無ければ`None`(呼び出し元は`変化なし`として扱う)。
    pub(super) fn 変化を確認して再読込する(&mut self) -> Option<Result<シーンデータ, アセットエラー>> {
        let 変化あり = self
            .参照ファイル一覧
            .iter()
            .any(|(パス, 記録時刻)| mtime::取得する(パス).map(|現在時刻| 現在時刻 > *記録時刻).unwrap_or(false));
        if !変化あり {
            return None;
        }

        let 結果 = シーンを読み込む(&self.カタログ, &self.id);
        if let Ok(シーン) = &結果 {
            self.参照ファイル一覧 = 時刻一覧を取得する(&シーン.参照ファイル一覧);
        }
        Some(結果)
    }
}

fn 時刻一覧を取得する(参照ファイル一覧: &[PathBuf]) -> Vec<(PathBuf, SystemTime)> {
    参照ファイル一覧
        .iter()
        .filter_map(|パス| mtime::取得する(パス).ok().map(|時刻| (パス.clone(), 時刻)))
        .collect()
}
