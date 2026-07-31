//! プリコンピュート大気のベイク済み画像の資源一式: ベイク済み画像・媒体シェーダー定数・ディスクリプタ・生成パイプライン。
//! 大気のベイク済み画像方式の空を持つフレーム構成のときだけ`描画段階資源`が保持し、大気のベイク済み画像の生成パスだけがこの一式を束縛する。
//!
//! ベイク済み画像は起動時に1度だけ確保して使い回す。大気や太陽が変わったフレームだけ中身を焼き直し、資源そのものは作り直さない。
//! GPUメモリを占める側を`base_resources`、束縛する側を`binding_set`が持ち、この型は2つを束ねて寿命を揃える。
//! 毎フレームの束縛先の組み立ては`draw_input`が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」
mod base_resources;
mod binding_set;
mod composite_descriptor;
mod descriptor_common;
mod draw_input;
mod image;
mod inputs;
mod lut_extent;
mod march_descriptor;
mod medium_bytes;
mod medium_uniform;
mod multiscatter_descriptor;
pub(crate) mod pass;
mod pipeline;
mod probe;
mod readback_buffer;
mod sample_descriptor;
mod transmittance_descriptor;
use crate::atmosphere::{大気のベイク済み画像の解像度, 大気散乱媒体};
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気のベイク済み画像のシェーダー一式;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use ash::vk;
pub(crate) use composite_descriptor::{空中遠近合成の束縛先, 空中遠近合成ディスクリプタ};
pub(crate) use draw_input::描画入力の材料;
pub(crate) use inputs::{
    大気のベイク済み画像の描画入力, 大気のベイク済み画像の生成入力, 大気のベイク済み画像の画像入力, 生成する組, 生成の即時定数
};
pub(crate) use lut_extent::大気のベイク済み画像の形;
pub(crate) use probe::{大気のベイク済み画像をgpuで焼いて読み戻す, 焼く条件};
pub(crate) use sample_descriptor::{大気のベイク済み画像標本の束縛先, 大気のベイク済み画像標本ディスクリプタ};
/// 解像度をこの型が覚えないのは、ベイク済み画像1枚ごとの形が`基盤`の画像そのものに付いており、計算の発行の大きさも
/// 即時定数もその形から導くためである。同じ値を2箇所で持つと、画像を作った寸法と焼く寸法が食い違いうる。
pub(crate) struct 大気のベイク済み画像一式 {
    基盤: base_resources::大気のベイク済み画像の基盤資源,
    束縛: binding_set::大気のベイク済み画像の束縛一式,
}
impl 大気のベイク済み画像一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 大気のベイク済み画像の解像度,
        シェーダー: &大気のベイク済み画像のシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let 基盤 = base_resources::大気のベイク済み画像の基盤資源::生成する(device, メモリプロパティ, 解像度)?;
        match binding_set::大気のベイク済み画像の束縛一式::生成する(device, &基盤, シェーダー) {
            Ok(束縛) => Ok(Self { 基盤, 束縛 }),
            Err(誤り) => {
                基盤.破棄する(device);
                Err(誤り)
            }
        }
    }
    /// 空パスの大気のベイク済み画像方式が参照する先。媒体のシェーダー定数と2枚のベイク済み画像のビューを、標本ディスクリプタの生成へ渡す。
    /// 多重散乱のベイク済み画像を含めないのは、参照する側が経路を刻まずスカイビューの結果と透過率だけを使うためである。
    pub(crate) fn 標本の束縛先(&self) -> 大気のベイク済み画像標本の束縛先 {
        大気のベイク済み画像標本の束縛先 {
            シェーダー定数一覧: self.基盤.シェーダー定数一覧(),
            透過率ビュー: self.基盤.透過率.画像ビュー,
            スカイビュービュー: self.基盤.スカイビュー.画像ビュー,
        }
    }
    /// 空中遠近合成が参照する先。ボリュームだけを渡すのは、合成が写像に惑星半径を要らず、深度を毎フレーム結び直すためである。
    pub(crate) fn 合成の束縛先(&self) -> 空中遠近合成の束縛先 {
        空中遠近合成の束縛先 {
            空中遠近ビュー: self.基盤.空中遠近.画像ビュー,
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
