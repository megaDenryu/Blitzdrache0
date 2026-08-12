//! UIテクスチャを読むディスクリプタセットのレイアウトと、そのセットを配るプールを所有する資源型。
//! レイアウトの宣言はbinding0=combined image sampler(FRAGMENT)の1本だけである。
//!
//! 2つを1つの型にするのは、プールがこのレイアウトのセットだけを配るためであり、寿命も所有者も同じだからである。
//! セット1枚ぶんの割り当てと書き込みと解放は`set`にある。呼ばれるのが台帳への登録と削除のたびであり、
//! レイアウトとプールを確保する局面(生成時の1回)と呼び出し頻度が違うためである。
//!
//! 注意: プールの容量上限は開発用UI専用の実用的な仮定(フォントアトラス+少数のアイコン)であり、
//! 超過時は割り当てがVulkanのエラーとして表面化する(型付きエラーで伝播、無言の破綻はしない)。
//!
//! 注意: `Drop`を持たない。破棄の順番はUIテクスチャ台帳が決める。

mod set;

use ash::vk;

use crate::error::レンダラーエラー;

/// 同時に保持できるUIテクスチャ数の上限。
pub(crate) const 最大テクスチャ数: u32 = 32;

pub(crate) struct UIテクスチャのディスクリプタ資源 {
    layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
}

impl UIテクスチャのディスクリプタ資源 {
    pub(crate) fn 確保する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let layout = レイアウトを作る(device)?;
        match プールを作る(device) {
            Ok(pool) => Ok(Self { layout, pool }),
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(layout, None) };
                Err(誤り)
            }
        }
    }

    /// UIパイプラインの宣言へ渡す境界。
    pub(crate) const fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout
    }

    /// セットの割り当てと解放へ渡す境界。プールのハンドルはこの型の外へ出さない。
    const fn プールのハンドル(&self) -> vk::DescriptorPool {
        self.pool
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    /// 注意: プールの破棄が残存セットの解放を暗黙に行う。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: layoutとpoolはSelfが唯一の所有者である。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(最大テクスチャ数)];
    let create_info = vk::DescriptorPoolCreateInfo::default()
        .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
        .max_sets(最大テクスチャ数)
        .pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
