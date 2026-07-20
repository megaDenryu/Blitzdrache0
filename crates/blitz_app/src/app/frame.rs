//! 1フレーム実行: ホットリロード確認 → 自己操作判定 →
//! 入力確定→世界更新→描画内容抽出→描画(フレーム内実行順序、コンポジションルートが明示)。

use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::cli::起動モード;
use crate::hot_reload::ホットリロード結果;
use crate::smoke::{self, スモークアクション};

impl アプリ {
    /// RedrawRequestedのたびに1フレーム分の描画を実行する。
    pub(super) fn 一フレーム実行する(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_none() {
            return;
        }
        self.ホットリロードを確認する();

        let アクション = match self.起動モード {
            起動モード::スモーク実行 { フレーム数 } => {
                smoke::判定する(self.現在フレーム, フレーム数, &self.シーン名)
            }
            起動モード::無期限実行 => スモークアクション::通常描画,
        };
        if let Some(window) = &self.window {
            smoke::window自己操作を適用する(window, アクション);
        }

        // フレーム内実行順序: 入力確定→世界更新→描画内容抽出→描画。
        let インテント = self.入力状態.インテントを確定する();
        self.カメラ.更新する(インテント);
        let 描画入力 = self.描画入力を作る();

        if let Err(誤り) = self.実行して判定する(アクション, 描画入力) {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }

        self.現在フレーム += 1;
        if let 起動モード::スモーク実行 { フレーム数 } = self.起動モード
            && self.現在フレーム >= フレーム数
        {
            event_loop.exit();
        }
    }

    fn 描画入力を作る(&self) -> blitz_render::フレーム描画入力 {
        let アスペクト比 = self
            .window
            .as_ref()
            .map(|window| super::aspect::計算する(window.inner_size()))
            .unwrap_or(1.0);
        blitz_render::フレーム描画入力 {
            クリア色: self.クリア色,
            ビュー射影: self.カメラ.ビュー射影変換を作る(アスペクト比),
            カメラ位置: self.カメラ.視点ワールド位置(),
            ライティング有効: self.ライティング有効,
        }
    }

    fn ホットリロードを確認する(&mut self) {
        let 結果 = self.ホットリローダー.確認する();
        let Some(レンダラー) = &mut self.レンダラー else {
            return;
        };
        match 結果 {
            ホットリロード結果::変化なし => {}
            ホットリロード結果::シェーダー再コンパイル成功 { シェーダー } => {
                if let Err(誤り) = レンダラー.シェーダーを差し替える(シェーダー) {
                    eprintln!("[hot-reload] パイプライン差し替えに失敗した: {誤り}");
                }
            }
            ホットリロード結果::シェーダー再コンパイル失敗 { メッセージ } => {
                eprintln!("[hot-reload] シェーダーの再コンパイルに失敗した:\n{メッセージ}");
            }
            ホットリロード結果::アセット再読込成功 { シーン } => {
                アセット再読込を反映する(レンダラー, &シーン);
            }
            ホットリロード結果::アセット再読込失敗 { メッセージ } => {
                eprintln!("[hot-reload] アセットの再読込に失敗した:\n{メッセージ}");
            }
        }
    }
}

fn アセット再読込を反映する(レンダラー: &mut blitz_render::レンダラー, シーン: &blitz_engine::シーンデータ) {
    match super::scene_load::シーンをレンダラー入力に変換する(シーン) {
        Ok((頂点一覧, インデックス一覧, マテリアル)) => {
            if let Err(誤り) = レンダラー.シーンを差し替える(&頂点一覧, &インデックス一覧, マテリアル) {
                eprintln!("[hot-reload] シーン差し替えに失敗した: {誤り}");
            }
        }
        Err(誤り) => eprintln!("[hot-reload] 再読込したシーンの変換に失敗した: {誤り}"),
    }
}
