//! プリコンピュート大気LUTの資源一式: LUT画像・媒体ユニフォーム・ディスクリプタ・生成パイプライン。
//! 大気LUT方式の空を持つフレーム構成のときだけ`描画段階資源`が保持し、大気LUTの生成パスだけがこの一式を束縛する。
//!
//! LUTは起動時に1度だけ確保して使い回す。大気や太陽が変わったフレームだけ中身を焼き直し、資源そのものは作り直さない。
//! GPUメモリを占める側を`base_resources`、束縛する側を`binding_set`が持ち、この型は2つを束ねて寿命を揃える。
//! 毎フレームの束縛先の組み立ては`draw_input`が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」

mod base_resources;
mod binding_set;
mod descriptor_common;
mod draw_input;
mod image;
mod inputs;
mod medium_bytes;
mod medium_uniform;
mod multiscatter_descriptor;
pub(crate) mod pass;
mod pipeline;
mod probe;
mod readback_buffer;
mod skyview_descriptor;
mod transmittance_descriptor;

use ash::vk;

pub(crate) use inputs::{大気LUT描画入力, 大気LUT生成入力, 大気LUT画像入力, 生成する組, 生成の押し込み定数};
pub(crate) use probe::{大気lutをgpuで焼いて読み戻す, 焼く条件};

use crate::atmosphere::{大気LUT解像度, 大気散乱媒体};
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct 大気LUT一式 {
    解像度: 大気LUT解像度,
    基盤: base_resources::大気LUT基盤資源,
    束縛: binding_set::大気LUT束縛一式,
}

impl 大気LUT一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 大気LUT解像度,
        シェーダー: &大気LUTシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let 基盤 = base_resources::大気LUT基盤資源::生成する(device, メモリプロパティ, 解像度)?;
        match binding_set::大気LUT束縛一式::生成する(device, &基盤, シェーダー) {
            Ok(束縛) => Ok(Self { 解像度, 基盤, 束縛 }),
            Err(誤り) => {
                基盤.破棄する(device);
                Err(誤り)
            }
        }
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(crate) fn 媒体を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        self.基盤.媒体を書き込む(device, フレーム添字, 媒体)
    }

    /// 束縛する側を先に、GPUメモリを占める側を後に破棄する。生成の逆順で片付ける。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.束縛.破棄する(device);
        self.基盤.破棄する(device);
    }
}
