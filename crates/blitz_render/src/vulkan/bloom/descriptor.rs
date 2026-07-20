//! ブルーム用ディスクリプタ: binding0のcombined image sampler 1個のlayoutと、
//! そこから割り当てる3セット(抽出=HDR読み・横ぼかし=a読み・縦ぼかし=b読み)。

use ash::vk;

use crate::error::レンダラーエラー;

const セット数: u32 = 3;

pub(super) struct ブルームディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pub(super) pool: vk::DescriptorPool,
    pub(super) set一覧: [vk::DescriptorSet; 3],
}

impl ブルームディスクリプタ {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはこの構造体が唯一の所有者。poolの破棄がsetの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

pub(super) fn 生成する(device: &ash::Device) -> Result<ブルームディスクリプタ, レンダラーエラー> {
    let binding = vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);
    let binding一覧 = [binding];
    let layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&binding一覧);
    // 安全性: deviceは生成済みで有効。
    let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None)? };

    let pool_size =
        vk::DescriptorPoolSize::default().ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER).descriptor_count(セット数);
    let pool_size一覧 = [pool_size];
    let pool_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&pool_size一覧);
    // 安全性: deviceは生成済みで有効。失敗時はlayoutを片付ける。
    let pool = match unsafe { device.create_descriptor_pool(&pool_info, None) } {
        Ok(pool) => pool,
        Err(誤り) => {
            // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_set_layout(layout, None) };
            return Err(誤り.into());
        }
    };

    let layout一覧 = [layout; 3];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは直前に生成済み。
    match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
        Ok(一覧) => match <[vk::DescriptorSet; 3]>::try_from(一覧) {
            Ok(set一覧) => Ok(ブルームディスクリプタ { layout, pool, set一覧 }),
            Err(_) => panic!("allocate_descriptor_setsが成功したのにセットが3個でなかった(Vulkan実装の契約違反)"),
        },
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
