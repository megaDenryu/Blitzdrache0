//! 大気LUTのうち、生成パスが束縛する側の資源(4つのディスクリプタと4本のコンピュートパイプライン)の所有者。
//! 触れるのはこの8つだけであり、画像もメモリも持たない。生成の途中で失敗したらそれまでに作ったハンドルを逆順に片付ける。
//!
//! 注意: 破棄はパイプラインを先、ディスクリプタを後にする。パイプラインレイアウトがディスクリプタセットレイアウトを参照するためである。

mod create;

use ash::vk;

use super::base_resources::大気LUT基盤資源;
use super::march_descriptor::経路生成ディスクリプタ;
use super::multiscatter_descriptor::多重散乱ディスクリプタ;
use super::{pipeline, transmittance_descriptor};
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::sync::フレームスロット添字;

pub(super) struct 大気LUT束縛一式 {
    透過率ディスクリプタ: transmittance_descriptor::透過率ディスクリプタ,
    多重散乱ディスクリプタ: 多重散乱ディスクリプタ,
    スカイビューディスクリプタ: 経路生成ディスクリプタ,
    空中遠近ディスクリプタ: 経路生成ディスクリプタ,
    pub(super) 透過率パイプライン: pipeline::生成パイプライン,
    pub(super) 多重散乱パイプライン: pipeline::生成パイプライン,
    pub(super) スカイビューパイプライン: pipeline::生成パイプライン,
    pub(super) 空中遠近パイプライン: pipeline::生成パイプライン,
}

impl 大気LUT束縛一式 {
    pub(super) fn 生成する(
        device: &ash::Device,
        基盤: &大気LUT基盤資源,
        シェーダー: &大気LUTシェーダー一式,
    ) -> Result<Self, crate::error::レンダラーエラー> {
        create::生成する(device, 基盤, シェーダー)
    }

    pub(super) fn 透過率セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.透過率ディスクリプタ.set(フレーム添字)
    }

    pub(super) fn 多重散乱セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.多重散乱ディスクリプタ.set(フレーム添字)
    }

    pub(super) fn スカイビューセット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.スカイビューディスクリプタ.set(フレーム添字)
    }

    pub(super) fn 空中遠近セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.空中遠近ディスクリプタ.set(フレーム添字)
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.空中遠近パイプライン.破棄する(device);
        self.スカイビューパイプライン.破棄する(device);
        self.多重散乱パイプライン.破棄する(device);
        self.透過率パイプライン.破棄する(device);
        self.空中遠近ディスクリプタ.破棄する(device);
        self.スカイビューディスクリプタ.破棄する(device);
        self.多重散乱ディスクリプタ.破棄する(device);
        self.透過率ディスクリプタ.破棄する(device);
    }
}
