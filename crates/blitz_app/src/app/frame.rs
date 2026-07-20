//! 1フレーム実行: ホットリロード確認 → 自己操作判定 →
//! 入力確定→世界更新→描画内容抽出→描画(フレーム内実行順序、コンポジションルートが明示)。

use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::cli::起動モード;
use crate::dev_ui::stats::開発UI統計;
use crate::error::起動エラー;
use crate::smoke::{self, スモークアクション};

impl アプリ {
    /// RedrawRequestedのたびに1フレーム分の描画を実行する。
    pub(super) fn 一フレーム実行する(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_none() {
            return;
        }
        self.ホットリロードを確認する();

        let アクション = match self.起動モード {
            起動モード::スモーク実行 { フレーム数 } => smoke::判定する(
                self.現在フレーム,
                フレーム数,
                &self.シーン名,
                self.粒子有効,
                self.開発ui初期有効,
            ),
            起動モード::無期限実行 => スモークアクション::通常描画,
        };
        if let Some(window) = &self.window {
            smoke::window自己操作を適用する(window, アクション);
        }

        // フレーム内実行順序: 入力確定→世界更新→描画内容抽出→描画。
        let インテント = self.入力状態.インテントを確定する();
        self.カメラ.更新する(インテント);
        let mut 描画入力 = self.描画入力を作る();
        match self.ui描画データを組み立てる() {
            Ok(データ) => 描画入力.ui描画 = データ,
            Err(誤り) => {
                self.起動時エラー = Some(誤り);
                event_loop.exit();
                return;
            }
        }

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
            ui描画: None,
        }
    }

    /// 開発用UI(egui)の今フレームぶんの描画データを組み立てる。ウィンドウ・
    /// レンダラー・開発UIのいずれかが未生成なら`None`(起動直後の1フレーム目のみ
    /// 起こりうる)。
    fn ui描画データを組み立てる(&mut self) -> Result<Option<blitz_render::UI描画データ>, 起動エラー> {
        let Some(window) = &self.window else { return Ok(None) };
        let Some(開発ui) = &mut self.開発ui else { return Ok(None) };
        let Some(レンダラー) = &mut self.レンダラー else { return Ok(None) };
        let 統計 = 開発UI統計 {
            パス別gpu時間: レンダラー.パス別gpu時間を取得する(),
            フレーム時間ms: 開発ui.フレーム時間を記録する(),
            validation件数: レンダラー.検証カウンタを取得する().件数を読む(),
        };
        開発ui.描画データを作る(window, レンダラー, 統計)
    }
}
