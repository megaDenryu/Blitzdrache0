//! 1フレーム実行: ホットリロード確認 → 自己操作判定 →
//! 入力確定→世界更新→描画内容抽出→描画(フレーム内実行順序、コンポジションルートが明示)。

mod action;
mod draw_input;
#[cfg(test)]
mod draw_input_tests;
mod finish;

use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::smoke;

pub(super) use draw_input::{フレーム視点, 組み立てる as 描画入力を組み立てる};

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
        if let Some(探査) = &mut self.個体詳細段探査 {
            探査.適用する(&mut self.カメラ, self.現在フレーム);
        }
        // 必要チャンク集合の中心は確定した視点位置から決めるため、カメラ更新の直後に置く。
        if let Err(誤り) = self.ストリーミングを進める() {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }
        // アニメーション時刻は固定歩進(判断47: スモークの決定性を保つ。実時間追従はしない)。
        self.アニメ時刻秒 += 1.0 / 60.0;
        // 時刻はこのフレームの可視判定より先に確定させる。ライト視錐台の導出が方向光の向きを読むためである。
        self.天空.進める();
        let 布入力 = self.布フレーム入力を作る();
        let ui描画 = match self.ui描画データを組み立てる() {
            Ok(データ) => データ,
            Err(誤り) => {
                self.起動時エラー = Some(誤り);
                event_loop.exit();
                return;
            }
        };
        // 視点は可視判定と描画入力の両方が使うため、1フレームに1度だけ作って配る。
        let 視点情報 = draw_input::視点を求める(self);
        // 計測が無効な実行では時刻を1度も読まない。読むと既存の性能時系列の条件が指定なしで変わる。
        let 群選択開始 = self.群選択計測.as_ref().map(|_| std::time::Instant::now());
        let 群選択結果 = self
            .可視判定
            .判定する(視点情報.ビュー射影, self.天空.ライティング(), 視点情報.カメラ大域位置);
        if let (Some(開始), Some(計測)) = (群選択開始, &mut self.群選択計測) {
            計測.記録する(開始.elapsed());
        }
        if let Err(誤り) = 群選択結果 {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }
        // 描画入力は詳細段の選択と可視個体選択を借用で持つ。描画は`&mut self`を要るため、借用元の受け皿をいったん`self`の外へ預かる。
        // 確保した容量を次のフレームへ持ち越すため、描画の成否によらず同じフレームのうちに返す。
        let 段選択受け皿 = self.ストリーミング.as_mut().map_or_else(Vec::new, |配線| 配線.地形詳細段選択を預かる());
        let 可視受け皿 = self.可視判定.受け皿を預ける();
        let 実行結果 = self.描画まで進める(super::draw_dispatch::フレーム材料 {
            アクション,
            視点情報: &視点情報,
            地形詳細段選択一覧: &段選択受け皿,
            可視受け皿: &可視受け皿,
            布入力,
            ui描画,
        });
        self.可視判定.受け皿を戻す(可視受け皿);
        if let Some(配線) = &mut self.ストリーミング {
            配線.地形詳細段選択を返す(段選択受け皿);
        }
        if let Err(誤り) = 実行結果 {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }

        finish::進めて必要なら終了する(self, event_loop);
    }
}
