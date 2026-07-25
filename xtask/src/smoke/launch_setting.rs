//! 1ステージ分の起動条件。子プロセスへ渡すフレーム数・シェーダー・アセット・シーンと、各機能の有無を保持する。
//! 引数列への変換と子プロセス起動は`run_stage`が担う。

use std::path::Path;

pub(super) struct 起動設定<'a> {
    pub(super) フレーム数: &'a str,
    pub(super) シェーダーパス: &'a Path,
    pub(super) アセットルート: Option<&'a Path>,
    pub(super) シーン名: &'a str,
    pub(super) 照明なし: bool,
    pub(super) 粒子あり: bool,
    pub(super) 開発uiあり: bool,
    pub(super) ポストなし: bool,
    pub(super) 布あり: bool,
    pub(super) ウィンドウ再構築検証あり: bool,
}

impl<'a> 起動設定<'a> {
    pub(super) fn 生成する(
        フレーム数: &'a str, シェーダーパス: &'a Path, アセットルート: Option<&'a Path>, シーン名: &'a str
    ) -> Self {
        Self {
            フレーム数,
            シェーダーパス,
            アセットルート,
            シーン名,
            照明なし: false,
            粒子あり: false,
            開発uiあり: false,
            ポストなし: false,
            布あり: false,
            ウィンドウ再構築検証あり: false,
        }
    }

    pub(super) fn 照明なし(mut self) -> Self {
        self.照明なし = true;
        self
    }
    pub(super) fn 粒子あり(mut self) -> Self {
        self.粒子あり = true;
        self
    }
    pub(super) fn 開発uiあり(mut self) -> Self {
        self.開発uiあり = true;
        self
    }
    pub(super) fn ポストなし(mut self) -> Self {
        self.ポストなし = true;
        self
    }
    pub(super) fn 布あり(mut self) -> Self {
        self.布あり = true;
        self
    }
    /// ポスト処理を有効に保ったままリサイズ・最小化・復帰を踏み、ピクセル判定を行わない検証計画を選ばせる。
    pub(super) fn ウィンドウ再構築検証あり(mut self) -> Self {
        self.ウィンドウ再構築検証あり = true;
        self
    }
}
