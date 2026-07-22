//! UIテクスチャの登録/更新/削除を管理する台帳(判断33)。IDごとにテクスチャとディスクリプタセットを対にして保持する。
//!
//! 注意: 登録・削除は`device_wait_idle`を伴う。開発用UIのテクスチャ変化はフォントアトラス初期化時のごく少数回のみのため、この単純さを優先する設計判断。

mod create;

use std::collections::HashMap;

use ash::vk;

use super::descriptor;
use super::texture::UIテクスチャ;
use crate::error::レンダラーエラー;
use crate::ui_texture_id::UIテクスチャID;
use crate::ui_texture_material::UIテクスチャ素材;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct UIテクスチャレジストリ {
    layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    表: HashMap<UIテクスチャID, (UIテクスチャ, vk::DescriptorSet)>,
}

impl UIテクスチャレジストリ {
    pub(crate) fn layout(&self) -> vk::DescriptorSetLayout {
        self.layout
    }

    /// テクスチャを新規登録、または既存IDを新しい内容で置き換える。
    pub(crate) fn 反映する(
        &mut self,
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        id: UIテクスチャID,
        素材: &UIテクスチャ素材,
    ) -> Result<(), レンダラーエラー> {
        let 新テクスチャ = UIテクスチャ::生成する(device, メモリプロパティ, 転送環境, 素材)?;
        let 新set = match descriptor::割り当てて書き込む(device, self.pool, self.layout, &新テクスチャ) {
            Ok(set) => set,
            Err(誤り) => {
                新テクスチャ.破棄する(device);
                return Err(誤り);
            }
        };

        if let Some((旧テクスチャ, 旧set)) = self.表.remove(&id) {
            // 安全性: 旧テクスチャ・旧セットの破棄前にGPU使用完了を待つ。
            let _ = unsafe { device.device_wait_idle() };
            旧テクスチャ.破棄する(device);
            descriptor::解放する(device, self.pool, 旧set);
        }
        self.表.insert(id, (新テクスチャ, 新set));
        Ok(())
    }

    pub(crate) fn 削除する(&mut self, device: &ash::Device, id: UIテクスチャID) {
        let Some((テクスチャ, set)) = self.表.remove(&id) else {
            return;
        };
        // 安全性: 破棄前にGPU使用完了を待つ。
        let _ = unsafe { device.device_wait_idle() };
        テクスチャ.破棄する(device);
        descriptor::解放する(device, self.pool, set);
    }

    /// `id`に対応するディスクリプタセットを返す。未登録IDの解決は呼び出し側
    /// (blitz_app)がテクスチャデルタを描画データより先に反映する契約に反しており、
    /// プログラムのバグとしてpanicする。
    pub(crate) fn setを取得する(&self, id: UIテクスチャID) -> vk::DescriptorSet {
        self.表
            .get(&id)
            .map(|(_, set)| *set)
            .unwrap_or_else(|| panic!("UIメッシュが未登録のUIテクスチャIDを参照した(テクスチャデルタの反映漏れ)"))
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        for (テクスチャ, _) in self.表.values() {
            テクスチャ.破棄する(device);
        }
        // 安全性: poolの破棄が残存setの解放を暗黙に行う。layout・poolはSelfが唯一の
        // 所有者であり、破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
