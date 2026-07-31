//! 明るさの圧縮用ディスクリプタ: binding0=HDR画像・binding1=光のにじみ結果の
//! combined image sampler 2個のlayout・pool・set。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) struct 明るさの圧縮ディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pub(super) pool: vk::DescriptorPool,
    pub(super) set: vk::DescriptorSet,
}

impl 明るさの圧縮ディスクリプタ {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはこの構造体が唯一の所有者。poolの破棄がsetの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

pub(super) fn 生成する(device: &ash::Device) -> Result<明るさの圧縮ディスクリプタ, レンダラーエラー> {
    let binding一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
    ];
    let layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&binding一覧);
    // 安全性: deviceは生成済みで有効。
    let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None)? };

    let pool_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(2);
    let pool_size一覧 = [pool_size];
    let pool_info = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&pool_size一覧);
    // 安全性: deviceは生成済みで有効。失敗時はlayoutを片付ける。
    let pool = match unsafe { device.create_descriptor_pool(&pool_info, None) } {
        Ok(pool) => pool,
        Err(誤り) => {
            // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_set_layout(layout, None) };
            return Err(誤り.into());
        }
    };

    let layout一覧 = [layout];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは直前に生成済み。
    match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
        Ok(一覧) => {
            let Some(&set) = 一覧.first() else {
                panic!("allocate_descriptor_setsが成功したのにセットが0個だった(Vulkan実装の契約違反)");
            };
            Ok(明るさの圧縮ディスクリプタ { layout, pool, set })
        }
        Err(誤り) => {
            // 安全性: pool・layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_descriptor_pool(pool, None);
                device.destroy_descriptor_set_layout(layout, None);
            }
            Err(誤り.into())
        }
    }
}
