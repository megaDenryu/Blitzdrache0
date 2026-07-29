//! プリコンピュート大気LUTの資源一式: LUT画像・媒体ユニフォーム・ディスクリプタ・生成パイプライン。
//! 空段階を持つフレーム構成のときだけ`描画段階資源`が保持し、大気LUTの生成パスだけがこの一式を束縛する。
//!
//! LUTは起動時に1度だけ確保して使い回す。大気が変わったフレームだけ中身を焼き直し、資源そのものは作り直さない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」

mod create;
mod descriptor_common;
mod image;
mod inputs;
mod medium_bytes;
mod medium_uniform;
mod pipeline;
mod transmittance_descriptor;

use ash::vk;

pub(crate) use inputs::{大気LUT描画入力, 大気LUT生成入力};

use crate::atmosphere::{大気LUT解像度, 大気散乱媒体};
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

/// 1ワークグループが受け持つテクセルの一辺。`shaders/atmosphere_transmittance.slang`の`numthreads`と一致させる。
const ワークグループの一辺: u32 = 8;

pub(crate) struct 大気LUT一式 {
    解像度: 大気LUT解像度,
    透過率: image::大気LUT画像,
    媒体ユニフォーム: medium_uniform::媒体ユニフォーム一式,
    透過率ディスクリプタ: transmittance_descriptor::透過率ディスクリプタ,
    透過率パイプライン: pipeline::生成パイプライン,
}

impl 大気LUT一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 大気LUT解像度,
        シェーダー: &大気LUTシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 解像度, シェーダー)
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(crate) fn 媒体を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        self.媒体ユニフォーム.書き込む(device, フレーム添字, 媒体)
    }

    pub(crate) fn 描画入力を作る(&self, フレーム添字: フレームスロット添字) -> 大気LUT描画入力 {
        大気LUT描画入力 {
            透過率: 大気LUT生成入力 {
                pipeline: self.透過率パイプライン.handle,
                layout: self.透過率パイプライン.layout,
                ディスクリプタセット: self.透過率ディスクリプタ.set(フレーム添字),
                ワークグループ数: ワークグループ数を求める(self.解像度.透過率の幅(), self.解像度.透過率の高さ()),
            },
            透過率画像: self.透過率.画像,
            透過率ビュー: self.透過率.画像ビュー,
            透過率寸法: self.透過率.寸法,
        }
    }

    /// パイプライン・ディスクリプタ・ユニフォーム・画像の順に破棄する。生成の逆順で片付ける。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.透過率パイプライン.破棄する(device);
        self.透過率ディスクリプタ.破棄する(device);
        self.媒体ユニフォーム.破棄する(device);
        self.透過率.破棄する(device);
    }
}

fn ワークグループ数を求める(幅: u32, 高さ: u32) -> [u32; 2] {
    [幅.div_ceil(ワークグループの一辺), 高さ.div_ceil(ワークグループの一辺)]
}
