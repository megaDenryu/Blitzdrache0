//! 描画段階資源のうち空段階だけを触る操作。触れるフィールドは`空`の1つであり、他の段階の資源へは触れない。
//!
//! 不変条件: `空`が`Some`であることと、フレーム構成に空段階があることは常に一致する。
//! この一致は生成時に決まり、以降変わらない。ここが返す`None`は「この構成ではその段を積まない」ことだけを意味する。

use ash::vk;

use super::描画段階資源;
use crate::vulkan;
use crate::vulkan::sync::フレームスロット添字;

impl 描画段階資源 {
    /// 空段階を持つフレーム構成かどうか。フレーム描画入力の空の有無と突き合わせる判定に使う。
    pub(in crate::renderer) fn 空を描くか(&self) -> bool {
        self.空.is_some()
    }

    pub(in crate::renderer) fn 空描画入力を作る(
        &self,
        ディスクリプタセット: vk::DescriptorSet,
        フレーム添字: フレームスロット添字,
    ) -> Option<vulkan::frame::空描画入力> {
        self.空.as_ref().map(|空| 空.描画入力を作る(ディスクリプタセット, フレーム添字))
    }

    /// 空中遠近合成パスが束縛する資源。空段階が無い構成と合成を切った構成では`None`を返す。
    pub(in crate::renderer) fn 空中遠近合成描画入力を作る(
        &self,
        ディスクリプタセット: vk::DescriptorSet,
        フレーム添字: フレームスロット添字,
        最遠距離: f32,
    ) -> Option<vulkan::frame::空中遠近合成描画入力> {
        self.空
            .as_ref()
            .and_then(|空| 空.空中遠近合成描画入力を作る(ディスクリプタセット, フレーム添字, 最遠距離))
    }

    /// このフレームの深度画像を合成のディスクリプタへ結び直す。合成を持たない構成では何もしない。
    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(in crate::renderer) fn 合成の深度を結び直す(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        深度ビュー: vk::ImageView,
    ) {
        if let Some(空) = &self.空 {
            空.合成の深度を結び直す(device, フレーム添字, 深度ビュー);
        }
    }
}
