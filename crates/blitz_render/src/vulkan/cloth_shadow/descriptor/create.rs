//! 布シャドウディスクリプタの生成。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に破棄する。部分的に生成された器は呼び出し元から見えない。

use ash::vk;

use super::{フレームユニフォームのバインディング番号, 布シャドウディスクリプタ};
use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(in crate::vulkan::cloth_shadow) fn 生成する(
    device: &ash::Device,
    ユニフォーム: &フレームユニフォーム一式,
) -> Result<布シャドウディスクリプタ, レンダラーエラー> {
    let layout = レイアウトを生成する(device)?;
    let pool = match プールを生成する(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_set_layout(layout, None) };
            return Err(誤り);
        }
    };
    let set一覧 = match セットを割り当てる(device, pool, layout) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            // 安全性: pool・layoutはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_descriptor_pool(pool, None);
                device.destroy_descriptor_set_layout(layout, None);
            }
            return Err(誤り);
        }
    };
    for フレーム添字 in フレームスロット添字::全スロット() {
        フレームユニフォームを結ぶ(device, set一覧[フレーム添字.配列添字()], ユニフォーム.buffer(フレーム添字));
    }
    Ok(布シャドウディスクリプタ { layout, pool, set一覧 })
}

fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [vk::DescriptorSetLayoutBinding::default()
        .binding(フレームユニフォームのバインディング番号)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn プールを生成する(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = u32::try_from(フレームインフライト数).unwrap_or_else(|_| panic!("フレームインフライト数がu32に収まらない"));
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(セット数)];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn セットを割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<[vk::DescriptorSet; フレームインフライト数], レンダラーエラー> {
    let layout一覧 = [layout; フレームインフライト数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは直前に生成済みで有効。
    let 一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    match <[vk::DescriptorSet; フレームインフライト数]>::try_from(一覧) {
        Ok(set一覧) => Ok(set一覧),
        Err(_) => panic!("allocate_descriptor_setsが成功したのにセット数が要求と違った(Vulkan実装の契約違反)"),
    }
}

fn フレームユニフォームを結ぶ(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(フレームユニフォームのバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .buffer_info(&buffer情報一覧)];
    // 安全性: setは割当済み、bufferは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
