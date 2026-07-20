//! 布シミュ用ディスクリプタ(判断54): b0=定数UBO、b1〜b9=ストレージ9本の統一レイアウトと、
//! UBO・介入だけが異なるフレームインフライト2セット。バインディング表はcloth_step.slang冒頭の仕様。

use ash::vk;

use super::buffers::布バッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームインフライト数;

const ストレージ本数: u32 = 9;

pub(super) struct 布ディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub(super) set一覧: [vk::DescriptorSet; フレームインフライト数],
}

impl 布ディスクリプタ {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはこの構造体が唯一の所有者。poolの破棄がsetの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

pub(super) fn 生成する(
    device: &ash::Device,
    バッファ: &布バッファ,
    スキン済み頂点buffer: Option<vk::Buffer>,
) -> Result<布ディスクリプタ, レンダラーエラー> {
    let mut binding一覧 = vec![
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE),
    ];
    for 添字 in 1..=ストレージ本数 {
        binding一覧.push(
            vk::DescriptorSetLayoutBinding::default()
                .binding(添字)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE),
        );
    }
    let layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&binding一覧);
    // 安全性: deviceは生成済みで有効。
    let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None)? };

    let セット数 = u32::try_from(フレームインフライト数).unwrap_or_else(|_| panic!("フレームインフライト数がu32に収まらない"));
    let pool_size一覧 = [
        vk::DescriptorPoolSize::default().ty(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(セット数),
        vk::DescriptorPoolSize::default().ty(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(ストレージ本数 * セット数),
    ];
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

    let layout一覧 = [layout; フレームインフライト数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは直前に生成済み。失敗時は両方片付ける。
    let set一覧 = match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
        Ok(一覧) => match <[vk::DescriptorSet; フレームインフライト数]>::try_from(一覧) {
            Ok(set一覧) => set一覧,
            Err(_) => panic!("allocate_descriptor_setsが成功したのにセット数が要求と違った(Vulkan実装の契約違反)"),
        },
        Err(誤り) => {
            // 安全性: pool・layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_descriptor_pool(pool, None);
                device.destroy_descriptor_set_layout(layout, None);
            }
            return Err(誤り.into());
        }
    };

    let 一式 = 布ディスクリプタ { layout, pool, set一覧 };
    for フレーム添字 in 0..フレームインフライト数 {
        super::write::書く(device, 一式.set一覧[フレーム添字], バッファ, スキン済み頂点buffer, フレーム添字);
    }
    Ok(一式)
}
