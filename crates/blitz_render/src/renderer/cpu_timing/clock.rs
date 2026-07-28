//! 1フレームのCPU区間の境界時刻を刻む時計。フレームの始まりに生まれ、終わりに1つの区間時間へ畳まれて捨てられる。
//! 触れるのは自分が刻んだ時刻だけであり、標本の蓄積には関わらない。寿命が1フレームで閉じることが`store`との違いである。
//! 注意: 刻む順序は描画の実行順そのものである。順序が入れ替わると区間の差が負の経過を跨ぎ、`duration_since`が0へ丸める。

use std::time::Instant;

use super::CPU区間時間;

pub(in crate::renderer) struct CPU区間時計 {
    開始: Instant,
    フェンス待機終了: Instant,
    フレームデータ準備終了: Instant,
    画像取得終了: Instant,
    作業領域更新開始: Instant,
    作業領域更新終了: Instant,
    描画終了: Instant,
}

impl CPU区間時計 {
    pub(in crate::renderer) fn 開始する() -> Self {
        let 現在 = Instant::now();
        Self {
            開始: 現在,
            フェンス待機終了: 現在,
            フレームデータ準備終了: 現在,
            画像取得終了: 現在,
            作業領域更新開始: 現在,
            作業領域更新終了: 現在,
            描画終了: 現在,
        }
    }

    pub(in crate::renderer) fn フェンス待機を終了する(&mut self) {
        self.フェンス待機終了 = Instant::now();
    }

    pub(in crate::renderer) fn フレームデータ準備を終了する(&mut self) {
        self.フレームデータ準備終了 = Instant::now();
    }

    pub(in crate::renderer) fn 画像取得を終了する(&mut self) {
        self.画像取得終了 = Instant::now();
    }

    pub(in crate::renderer) fn 作業領域更新を開始する(&mut self) {
        self.作業領域更新開始 = Instant::now();
    }

    pub(in crate::renderer) fn 作業領域更新を終了する(&mut self) {
        self.作業領域更新終了 = Instant::now();
    }

    pub(in crate::renderer) fn 描画を終了する(&mut self) {
        self.描画終了 = Instant::now();
    }

    pub(in crate::renderer) fn 終了する(self) -> CPU区間時間 {
        let 終了 = Instant::now();
        CPU区間時間 {
            フェンス待機ms: 経過ms(self.開始, self.フェンス待機終了),
            フレームデータ準備ms: 経過ms(self.フェンス待機終了, self.フレームデータ準備終了),
            画像取得ms: 経過ms(self.フレームデータ準備終了, self.画像取得終了),
            記録送信提示ms: 経過ms(self.画像取得終了, 終了),
            作業領域更新ms: 経過ms(self.作業領域更新開始, self.作業領域更新終了),
            描画記録送信提示ms: 経過ms(self.作業領域更新終了, self.描画終了),
        }
    }
}

fn 経過ms(始まり: Instant, 終わり: Instant) -> f64 {
    終わり.duration_since(始まり).as_secs_f64() * 1000.0
}
