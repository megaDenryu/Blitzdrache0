//! 実行時アセットのmtime監視。生成物が更新されたら同じ安定IDで再読込する。

use std::path::PathBuf;
use std::time::SystemTime;

use blitz_engine::{アセットID, カタログ, シーンデータ, 実行時シーンのファイル, 実行時シーン読込エラー};

use super::mtime;

pub(super) struct アセット監視状態 {
    カタログ: カタログ,
    id: アセットID,
    参照ファイル一覧: Vec<(PathBuf, SystemTime)>,
}

pub(super) fn アセット監視状態を構築する(
    カタログ: カタログ, id: アセットID, 参照ファイル一覧: &[PathBuf]
) -> アセット監視状態 {
    アセット監視状態 {
        カタログ,
        id,
        参照ファイル一覧: 時刻一覧を取得する(参照ファイル一覧),
    }
}

impl アセット監視状態 {
    pub(super) fn カタログ(&self) -> &カタログ {
        &self.カタログ
    }

    /// 参照ファイルのいずれかが変化していれば再読込を試み、結果を返す。
    /// 変化が無ければ`None`(呼び出し元は`変化なし`として扱う)。
    pub(super) fn 変化を確認して再読込する(&mut self) -> Option<Result<シーンデータ, 実行時シーン読込エラー>> {
        let 変化あり = self
            .参照ファイル一覧
            .iter()
            .any(|(パス, 記録時刻)| mtime::取得する(パス).map(|現在時刻| 現在時刻 > *記録時刻).unwrap_or(false));
        if !変化あり {
            return None;
        }

        let 結果 = 実行時シーンのファイル::カタログの安定idから作る(&self.カタログ, &self.id)
            .ok_or_else(|| 実行時シーン読込エラー::カタログ未登録(self.id.clone()))
            .and_then(|ファイル| ファイル.読み込む());
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
