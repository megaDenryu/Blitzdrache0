//! 空中遠近合成パスの束縛先の対: 合成パイプラインと合成ディスクリプタ。
//! 2つを1つの型へまとめるのは、片方だけがある状態を作れなくするためである(パイプラインレイアウトが
//! ディスクリプタセットレイアウトを参照するため、対でしか意味を持たない)。
//!
//! 触れるのはこの2つだけであり、空パスの資源へは触れない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::{空中遠近合成の束縛先, 空中遠近合成ディスクリプタ};
use crate::vulkan::frame::空中遠近合成描画入力;
use crate::vulkan::pipeline::空中遠近合成パイプライン;
use crate::vulkan::sync::フレームスロット添字;

pub(crate) struct 空中遠近合成資源 {
    パイプライン: 空中遠近合成パイプライン,
    ディスクリプタ: 空中遠近合成ディスクリプタ,
}

impl 空中遠近合成資源 {
    /// ディスクリプタを先に作る。パイプラインレイアウトがそのディスクリプタセットレイアウトを要るためである。
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        ビューとパスlayout: vk::DescriptorSetLayout,
        束縛先: &空中遠近合成の束縛先,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let ディスクリプタ = 空中遠近合成ディスクリプタ::生成する(確保係, 束縛先)?;
        let layout一覧 = [ビューとパスlayout, ディスクリプタ.layout];
        match 空中遠近合成パイプライン::生成する(確保係, カラー形式, &layout一覧, シェーダー) {
            Ok(パイプライン) => Ok(Self {
                パイプライン,
                ディスクリプタ,
            }),
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(crate) fn 描画入力を作る(
        &self,
        ディスクリプタセット: vk::DescriptorSet,
        フレーム添字: フレームスロット添字,
        最遠距離: f32,
    ) -> 空中遠近合成描画入力 {
        空中遠近合成描画入力 {
            pipeline: self.パイプライン.handle,
            layout: self.パイプライン.layout,
            シーンセット: ディスクリプタセット,
            合成セット: self.ディスクリプタ.set(フレーム添字),
            最遠距離,
        }
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている。
    pub(crate) fn 深度を結び直す(
        &self, device: &ash::Device, フレーム添字: フレームスロット添字, 深度ビュー: vk::ImageView
    ) {
        self.ディスクリプタ.深度を結び直す(device, フレーム添字, 深度ビュー);
    }

    /// 注意: パイプラインをディスクリプタより先に破棄する(パイプラインレイアウトがディスクリプタセットレイアウトを参照するため)。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
    }
}
