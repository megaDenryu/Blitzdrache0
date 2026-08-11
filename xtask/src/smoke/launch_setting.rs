//! 1ステージ分の起動条件。子プロセスへ渡すフレーム数・シェーダー・アセット・シーンと、各機能の有無を保持し、
//! 起動指定へ写す。実行環境の用意と起動は`run_stage`が担う。

use std::path::Path;

use crate::acceptance::{アプリの起動指定, 描画フレーム数, 検収シーン名};

pub(super) struct 起動設定<'a> {
    pub(super) フレーム数: 描画フレーム数,
    pub(super) シェーダーパス: &'a Path,
    pub(super) アセットルート: Option<&'a Path>,
    pub(super) シーン名: 検収シーン名,
    照明なし: bool,
    粒子あり: bool,
    開発uiあり: bool,
    ポストなし: bool,
    布あり: bool,
    ウィンドウ再構築検証あり: bool,
}

impl<'a> 起動設定<'a> {
    pub(super) fn 生成する(
        フレーム数: 描画フレーム数,
        シェーダーパス: &'a Path,
        アセットルート: Option<&'a Path>,
        シーン名: 検収シーン名,
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

    /// 各機能の有無を起動指定へ写す。
    ///
    /// 厳密ピクセル判定を使うステージはポストを外し、期待値を明るさの圧縮導入前のまま保つ(判断39)。
    pub(super) fn 起動指定へ写す(&self) -> アプリの起動指定 {
        let 有無と綴り = [
            (self.照明なし, "--unlit"),
            (self.粒子あり, "--particles"),
            (self.開発uiあり, "--dev-ui"),
            (self.ポストなし, "--no-post"),
            (self.布あり, "--cloth"),
            (self.ウィンドウ再構築検証あり, "--window-rebuild"),
        ];
        有無と綴り
            .into_iter()
            .filter(|(有るか, _)| *有るか)
            .fold(基本の指定(self), |指定, (_, 綴り)| 指定.選択肢を足す(綴り))
    }
}

fn 基本の指定(設定: &起動設定<'_>) -> アプリの起動指定 {
    アプリの起動指定::シーンと枚数を決める(設定.シーン名, 設定.フレーム数)
        .パスを値に持つ選択肢を足す("--shader-source", 設定.シェーダーパス)
}
