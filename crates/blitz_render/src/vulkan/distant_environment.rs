//! 遠方環境の資源一式: 立方体画像・ディスクリプタ・生成パイプライン。
//! 大気のベイク済み画像方式の空を持つフレーム構成で、かつ遠方環境を焼く構成のときだけ`描画段階資源`が保持する。
//!
//! 大気のベイク済み画像一式と別の型にするのは、遠方環境が大気の表ではなく、その上に置いた供給から作る間接照明の
//! 表現だからである。焼き直しの条件も別であり(遠方環境キーは太陽の放射照度と夜空放射輝度も見る)、
//! 1つの型へ混ぜると2つの更新判断が同じ資源の中で絡む。
//!
//! 参照するスカイビューのベイク済み画像と媒体のシェーダー定数は大気のベイク済み画像一式が所有し、この型はビューを借りるだけである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「遠方環境の形式と生成(3-Ia)」

mod create;
mod descriptor;
pub(in crate::vulkan) mod image;
mod inputs;
pub(crate) mod pass;
mod probe;

use ash::vk;

use crate::atmosphere::遠方環境の解像度;
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::pipeline::生成パイプライン;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan) use image::遠方環境の立方体画像;
pub(crate) use inputs::{
    計算の班数を求める, 遠方環境が借りる束縛先, 遠方環境の即時定数, 遠方環境の生成入力, 遠方環境の画像入力
};
pub(crate) use probe::{遠方環境をgpuで焼いて読み戻す, 遠方環境を焼く条件};

pub(crate) struct 遠方環境一式 {
    画像: image::遠方環境の立方体画像,
    ディスクリプタ: descriptor::遠方環境ディスクリプタ,
    パイプライン: 生成パイプライン,
}

impl 遠方環境一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 遠方環境の解像度,
        借りる束縛先: 遠方環境が借りる束縛先<'_>,
        シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 解像度, 借りる束縛先, シェーダー)
    }

    pub(crate) fn 画像入力を作る(&self) -> 遠方環境の画像入力 {
        遠方環境の画像入力 {
            画像: self.画像.画像,
            ビュー: self.画像.配列ビュー,
            範囲: self.画像.範囲(),
            層数: self.画像.層数(),
            グラフへ渡す寸法: self.画像.グラフへ渡す寸法(),
        }
    }

    /// そのフレームの生成パスが束縛する資源。焼くかどうかの判断は呼び出し元が済ませてから呼ぶ。
    pub(crate) fn 生成入力を作る(
        &self, フレーム添字: フレームスロット添字, 即時定数: 遠方環境の即時定数
    ) -> 遠方環境の生成入力 {
        遠方環境の生成入力 {
            pipeline: self.パイプライン.handle,
            layout: self.パイプライン.layout,
            ディスクリプタセット: self.ディスクリプタ.set(フレーム添字),
            計算の班数: 計算の班数を求める(self.画像.面の一辺, self.画像.層数()),
            即時定数,
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.画像.破棄する(device);
    }
}
