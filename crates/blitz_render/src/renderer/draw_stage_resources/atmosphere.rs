//! 描画段階資源のうち大気LUTだけを触る操作。触れるフィールドは`大気LUT`の1つであり、他の段階の資源へは触れない。
//!
//! 不変条件: `大気LUT`が`Some`であることと、フレーム構成に空段階があることは常に一致する。
//! この一致は生成時に決まり、以降変わらない。ここが返す`None`は「この構成では大気LUTを作らない」ことだけを意味する。

use super::描画段階資源;
use crate::atmosphere::{スカイビュー観測条件, 大気散乱媒体};
use crate::atmosphere_lut_input::大気LUT生成指示;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::大気LUT描画入力;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

impl 描画段階資源 {
    /// 大気LUT資源を持つフレーム構成かどうか。フレーム描画入力の大気媒体の有無と突き合わせる判定に使う。
    pub(in crate::renderer) fn 大気lutを作るか(&self) -> bool {
        self.大気lut.is_some()
    }

    /// 大気LUTの生成パスが束縛する資源。LUT資源を持たない構成では`None`を返す。
    pub(in crate::renderer) fn 大気lut描画入力を作る(
        &self,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
        観測条件: スカイビュー観測条件,
        指示: 大気LUT生成指示,
    ) -> Option<大気LUT描画入力> {
        self.大気lut.as_ref().map(|一式| 一式.描画入力を作る(フレーム添字, 媒体, 観測条件, 指示))
    }

    /// そのフレームの大気媒体をLUT生成用のユニフォームへ書く。LUT資源を持たない構成では何もしない。
    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている。
    pub(in crate::renderer) fn 大気媒体を書き込む(
        &self,
        device: &GPUデバイス,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        match &self.大気lut {
            Some(一式) => 一式.媒体を書き込む(device, フレーム添字, 媒体),
            None => Ok(()),
        }
    }
}
