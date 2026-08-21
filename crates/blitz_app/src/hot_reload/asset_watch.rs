//! 生成台帳を公開完了の印として監視し、実行時アセット一式を同じ世代から読み直す。

use std::time::SystemTime;

use blitz_engine::{アセットID, カタログ, 実行時シーンのファイル};

use super::asset_reload_error::実行時アセット一式の再読込エラー;
use super::reload_result::公開済みの実行時アセット一式;
use crate::runtime_assets::実行時アセットの置き場;

pub(super) struct アセット監視状態 {
    置き場: 実行時アセットの置き場,
    カタログ: カタログ,
    id: アセットID,
    確認済みの生成台帳更新時刻: Option<SystemTime>,
}

pub(super) fn アセット監視状態を構築する(
    置き場: 実行時アセットの置き場,
    カタログ: カタログ,
    id: アセットID,
) -> アセット監視状態 {
    let 確認済みの生成台帳更新時刻 = 置き場.生成台帳の更新時刻を取得する().ok();
    アセット監視状態 {
        置き場,
        カタログ,
        id,
        確認済みの生成台帳更新時刻,
    }
}

impl アセット監視状態 {
    pub(super) fn カタログ(&self) -> &カタログ {
        &self.カタログ
    }

    pub(super) fn カタログを採用する(&mut self, カタログ: カタログ) {
        self.カタログ = カタログ;
    }

    pub(super) fn 公開完了を確認して一式を再読込する(
        &mut self,
    ) -> Option<Result<公開済みの実行時アセット一式, 実行時アセット一式の再読込エラー>> {
        let 現在時刻 = self.置き場.生成台帳の更新時刻を取得する().ok()?;
        if !公開完了を検出したか(self.確認済みの生成台帳更新時刻, 現在時刻) {
            return None;
        }
        self.確認済みの生成台帳更新時刻 = Some(現在時刻);
        Some(self.公開済みの一式を読み込む())
    }

    fn 公開済みの一式を読み込む(
        &self,
    ) -> Result<公開済みの実行時アセット一式, 実行時アセット一式の再読込エラー> {
        let カタログ = self.置き場.カタログのファイル().読み込む()?;
        let チャンク目録 = self.置き場.チャンク目録のファイル().読み込む()?;
        let 起動時シーン = 実行時シーンのファイル::カタログの安定idから作る(&カタログ, &self.id)
            .ok_or_else(|| blitz_engine::実行時シーン読込エラー::カタログ未登録(self.id.clone()))?
            .読み込む()?;
        Ok(公開済みの実行時アセット一式 {
            カタログ,
            チャンク目録,
            起動時シーン,
        })
    }
}

fn 公開完了を検出したか(確認済み: Option<SystemTime>, 現在: SystemTime) -> bool {
    match 確認済み {
        None => true,
        Some(確認済み) => 現在 > 確認済み,
    }
}

#[cfg(test)]
mod tests {
    use super::公開完了を検出したか;
    use std::time::{Duration, SystemTime};

    #[test]
    fn 起動時に台帳が無く後から現れた場合は公開完了になる() {
        assert!(公開完了を検出したか(None, SystemTime::UNIX_EPOCH));
    }

    #[test]
    fn 台帳の更新時刻が進んだ場合だけ公開完了になる() {
        let 古い = SystemTime::UNIX_EPOCH;
        let 新しい = 古い + Duration::from_secs(1);
        assert!(公開完了を検出したか(Some(古い), 新しい));
        assert!(!公開完了を検出したか(Some(新しい), 新しい));
    }
}
