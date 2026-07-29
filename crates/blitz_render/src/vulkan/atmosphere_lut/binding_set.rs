//! 大気LUTのうち、生成パスが束縛する側の資源(2つのディスクリプタと2本のコンピュートパイプライン)の所有者。
//! 触れるのはこの4つだけであり、画像もメモリも持たない。生成の途中で失敗したらそれまでに作ったハンドルを逆順に片付ける。
//!
//! 注意: 破棄はパイプラインを先、ディスクリプタを後にする。パイプラインレイアウトがディスクリプタセットレイアウトを参照するためである。

use ash::vk;

use super::base_resources::大気LUT基盤資源;
use super::multiscatter_descriptor::{多重散乱の束縛先, 多重散乱ディスクリプタ};
use super::{pipeline, transmittance_descriptor};
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::sync::フレームスロット添字;

pub(super) struct 大気LUT束縛一式 {
    透過率ディスクリプタ: transmittance_descriptor::透過率ディスクリプタ,
    多重散乱ディスクリプタ: 多重散乱ディスクリプタ,
    pub(super) 透過率パイプライン: pipeline::生成パイプライン,
    pub(super) 多重散乱パイプライン: pipeline::生成パイプライン,
}

impl 大気LUT束縛一式 {
    pub(super) fn 生成する(
        device: &ash::Device,
        基盤: &大気LUT基盤資源,
        シェーダー: &大気LUTシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let ユニフォーム一覧 = 基盤.ユニフォーム一覧();
        let 透過率ディスクリプタ =
            transmittance_descriptor::透過率ディスクリプタ::生成する(device, ユニフォーム一覧, 基盤.透過率.画像ビュー)?;
        let 多重散乱ディスクリプタ = match 多重散乱ディスクリプタ::生成する(
            device,
            多重散乱の束縛先 {
                ユニフォーム一覧: &ユニフォーム一覧,
                透過率ビュー: 基盤.透過率.画像ビュー,
                多重散乱ビュー: 基盤.多重散乱.画像ビュー,
            },
        ) {
            Ok(値) => 値,
            Err(誤り) => {
                透過率ディスクリプタ.破棄する(device);
                return Err(誤り);
            }
        };
        let 透過率パイプライン =
            match pipeline::生成パイプライン::生成する(device, 透過率ディスクリプタ.layout, シェーダー.透過率.コード())
            {
                Ok(値) => 値,
                Err(誤り) => {
                    多重散乱ディスクリプタ.破棄する(device);
                    透過率ディスクリプタ.破棄する(device);
                    return Err(誤り);
                }
            };
        match pipeline::生成パイプライン::生成する(device, 多重散乱ディスクリプタ.layout, シェーダー.多重散乱.コード())
        {
            Ok(多重散乱パイプライン) => Ok(Self {
                透過率ディスクリプタ,
                多重散乱ディスクリプタ,
                透過率パイプライン,
                多重散乱パイプライン,
            }),
            Err(誤り) => {
                透過率パイプライン.破棄する(device);
                多重散乱ディスクリプタ.破棄する(device);
                透過率ディスクリプタ.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(super) fn 透過率セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.透過率ディスクリプタ.set(フレーム添字)
    }

    pub(super) fn 多重散乱セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.多重散乱ディスクリプタ.set(フレーム添字)
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.多重散乱パイプライン.破棄する(device);
        self.透過率パイプライン.破棄する(device);
        self.多重散乱ディスクリプタ.破棄する(device);
        self.透過率ディスクリプタ.破棄する(device);
    }
}
