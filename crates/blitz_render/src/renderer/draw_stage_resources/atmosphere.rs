//! 描画段階資源のうち大気LUTだけを触る操作。触れるフィールドは`大気LUT`の1つであり、他の段階の資源へは触れない。
//!
//! 不変条件: `大気LUT`が`Some`であることと、フレーム構成に空段階があることは常に一致する。
//! この一致は生成時に決まり、以降変わらない。ここが返す`None`は「この構成では大気LUTを作らない」ことだけを意味する。

use super::描画段階資源;
use crate::atmosphere::{スカイビュー観測条件, 大気散乱媒体, 空中遠近観測条件};
use crate::atmosphere_lut_input::大気LUT生成指示;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::atmosphere_lut::{大気LUT描画入力, 描画入力の材料};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

impl 描画段階資源 {
    /// 大気LUT資源を持つフレーム構成かどうか。フレーム描画入力の大気媒体の有無と突き合わせる判定に使う。
    pub(in crate::renderer) fn 大気lutを作るか(&self) -> bool {
        self.大気lut.is_some()
    }

    /// 大気LUTの生成パスが束縛する資源。LUT資源を持たない構成では`None`を返す。
    /// 空中遠近を焼くかどうかは空段階の方式が決めるため、その判定をこの型が持つ空段階資源へ問う。
    pub(in crate::renderer) fn 大気lut描画入力を作る(
        &self,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
        条件: (スカイビュー観測条件, 空中遠近観測条件),
        指示: 大気LUT生成指示,
    ) -> Option<大気LUT描画入力> {
        let (スカイビュー条件, 空中遠近条件) = 条件;
        let 材料 = 描画入力の材料 {
            フレーム添字,
            媒体,
            スカイビュー条件,
            空中遠近条件,
            指示,
            空中遠近を引くか: self.空中遠近を引くか(),
        };
        self.大気lut.as_ref().map(|一式| 一式.描画入力を作る(&材料))
    }

    /// 空パスが空中遠近ボリュームを引くか。大気LUT腕だけが引く。
    /// Hosek腕で焼かないのは、誰も引かないボリュームを焼く時間がそのまま無駄になるためである。
    pub(in crate::renderer) fn 空中遠近を引くか(&self) -> bool {
        self.空.as_ref().is_some_and(vulkan::sky_stage::空段階資源::大気lut腕か)
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
