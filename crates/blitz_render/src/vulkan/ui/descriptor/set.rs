//! UIテクスチャ1枚ぶんのディスクリプタセットの割り当てと書き込みと個別解放。
//! 呼ばれるのは台帳への登録と削除のたびであり、レイアウトとプールを確保する局面とは呼び出し頻度が違う。

use ash::vk;

use super::{UIテクスチャのディスクリプタ資源, 束縛の宣言};
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::結ぶ現物;
use crate::vulkan::ui::texture::UIテクスチャ;

impl UIテクスチャのディスクリプタ資源 {
    pub(crate) fn テクスチャのセットを割り当てて書き込む(
        &self,
        device: &ash::Device,
        テクスチャ: &UIテクスチャ,
    ) -> Result<vk::DescriptorSet, レンダラーエラー> {
        let layout一覧 = [self.レイアウトのハンドル()];
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.プールのハンドル())
            .set_layouts(&layout一覧);
        // 安全性: pool・layoutは生成済みで有効。
        let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
        let Some(&set) = set一覧.first() else {
            panic!("allocate_descriptor_setsが1個でなく{}個のセットを返した", set一覧.len());
        };

        束縛の宣言.書き込み先(device, set).並びの位置ごとに結ぶ([結ぶ現物::サンプラー付きの画像 {
            ビュー: テクスチャ.image_view,
            サンプラー: テクスチャ.sampler,
            レイアウト: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        }]);
        Ok(set)
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元(device_wait_idle)が保証する。
    pub(crate) fn セットを解放する(&self, device: &ash::Device, set: vk::DescriptorSet) {
        // 安全性: setはこのプールから割当済みで、プールはFREE_DESCRIPTOR_SETフラグ付きで生成済みである。
        unsafe {
            let _ = device.free_descriptor_sets(self.プールのハンドル(), &[set]);
        }
    }
}
