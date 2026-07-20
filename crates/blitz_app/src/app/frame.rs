//! 1フレーム実行: ホットリロード確認 → 自己操作判定 →
//! 入力確定→世界更新→描画内容抽出→描画(フレーム内実行順序、コンポジションルートが明示)。

use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::cli::起動モード;
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
        // UI組み立てを描画入力の作成より先に行うのは、露出スライダーの変更を同じフレームの描画へ反映するため。
        let インテント = self.入力状態.インテントを確定する();
        self.カメラ.更新する(インテント);
        // アニメーション時刻は固定歩進(判断47: スモークの決定性を保つ。実時間追従はしない)。
        self.アニメ時刻秒 += 1.0 / 60.0;
        let 布入力 = self.布フレーム入力を作る();
        let ui描画 = match self.ui描画データを組み立てる() {
            Ok(データ) => データ,
            Err(誤り) => {
                self.起動時エラー = Some(誤り);
                event_loop.exit();
                return;
            }
        };
        let mut 描画入力 = self.描画入力を作る();
        描画入力.布 = 布入力;
        描画入力.ui描画 = ui描画;

        let 実行結果 = if self.ダンプ対象フレームか() {
            self.読み戻してダンプする(描画入力)
        } else {
            self.実行して判定する(アクション, 描画入力)
        };
        if let Err(誤り) = 実行結果 {
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
            露出: self.露出,
            スキン行列一覧: self
                .アニメーション
                .as_ref()
                .map(|再生| 再生.スキン行列を計算する(self.アニメ時刻秒, self.ブレンド)),
            布: None,
            ui描画: None,
        }
    }

}
