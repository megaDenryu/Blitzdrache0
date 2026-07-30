//! スワップチェーンが取得を許した画像1枚を指す添字。スワップチェーン画像・画像ビュー・提示セマフォは、
//! いずれもこの添字で参照する要素数`スワップチェーン::画像数`の配列である。要素数は環境が決めるため2から4程度まで変わる。
//! `vkAcquireNextImageKHR`が返した値だけがこの型の発生源であり、値の範囲はスワップチェーンの契約が保証する。
//!
//! 注意: フレームスロット添字とは値の範囲が重なるため、両方を生の整数で扱うと入れ替えても添字が範囲内に収まり、
//! 別の画像とセマフォを参照したまま描画が成立する。別型にすることで入れ替えをコンパイルエラーにする。
//! 参照: crates/blitz_render/src/vulkan/sync/frame_slot_index.rs がもう一方の添字型を持つ。

#[cfg(test)]
mod image_index_tests;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct スワップチェーン画像添字 {
    値: u32,
}

impl スワップチェーン画像添字 {
    /// 前提: `vkAcquireNextImageKHR`が成功して返した添字であること。範囲の検証はスワップチェーンの契約に委ねる。
    pub(crate) const fn 取得結果から生成する(値: u32) -> Self {
        Self { 値 }
    }

    /// GPU境界: `vkQueuePresentKHR`へ渡す生の添字へ戻す。
    pub(crate) const fn gpu境界用u32(self) -> u32 {
        self.値
    }

    /// GPU境界: 画像数ぶんの配列を参照する生の添字へ戻す。
    /// 注意: `u32`が`usize`へ収まらない環境ではプログラムの前提が崩れているため落とす。
    pub(crate) fn 配列添字(self) -> usize {
        let 値 = self.値;
        usize::try_from(値).unwrap_or_else(|_| panic!("スワップチェーン画像添字がusizeに収まらない: {値}"))
    }
}
