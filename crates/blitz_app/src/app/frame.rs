//! 1フレーム実行: ホットリロード確認 → 自己操作判定 →
//! 入力確定→世界更新→描画内容抽出→描画(フレーム内実行順序、コンポジションルートが明示)。

mod action;
mod draw_input;
mod finish;

use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::smoke;

impl アプリ {
    /// RedrawRequestedのたびに1フレーム分の描画を実行する。
    pub(super) fn 一フレーム実行する(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_none() {
            return;
        }
        if let Some(計測) = &mut self.フレーム間隔計測 {
            計測.記録する();
        }
        self.ホットリロードを確認する();

        let アクション = action::選ぶ(self);
        if let Some(window) = &self.window {
            smoke::window自己操作を適用する(window, アクション);
        }

        // フレーム内実行順序: 入力確定→世界更新→描画内容抽出→描画。
        // UI組み立てを描画入力の作成より先に行うのは、露出スライダーの変更を同じフレームの描画へ反映するため。
        let インテント = self.入力状態.インテントを確定する();
        self.カメラ.更新する(インテント);
        // 必要チャンク集合の中心は確定した視点位置から決めるため、カメラ更新の直後に置く。
        if let Err(誤り) = self.ストリーミングを進める() {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }
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
        let mut 描画入力 = draw_input::組み立てる(self);
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

        finish::進めて必要なら終了する(self, event_loop);
    }
}
