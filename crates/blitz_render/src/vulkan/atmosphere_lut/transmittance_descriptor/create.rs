//! 透過率ディスクリプタの生成局面。レイアウト・プール・セットの割当と、束縛先を指す内容の書き込みを行う。
//! 呼ばれるのは大気のベイク済み画像一式の組み立て時の1回だけであり、以降のフレームはセットを参照するだけである。

use ash::vk;

use super::透過率ディスクリプタ;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};

pub(super) fn 生成する(
    device: &ash::Device,
    ユニフォーム一覧: [vk::Buffer; フレームインフライト数],
    書き込み先: vk::ImageView,
) -> Result<透過率ディスクリプタ, レンダラーエラー> {
    let layout = レイアウトを作る(device)?;
    let pool = match プールを作る(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, None);
            return Err(誤り);
        }
    };
    let set一覧 = match descriptor_common::セットを割り当てる(device, pool, layout) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, Some(pool));
            return Err(誤り);
        }
    };
    for 添字 in フレームスロット添字::全スロット() {
        書き込む(device, set一覧[添字.配列添字()], ユニフォーム一覧[添字.配列添字()], 書き込み先);
    }
    Ok(透過率ディスクリプタ { layout, pool, set一覧 })
}

fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(セット数),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(セット数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

/// 注意: ストレージ画像のレイアウトはGENERALである。レンダーグラフの画像用途「コンピュート書き」が同じレイアウトへ遷移させており、
/// ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。
fn 書き込む(device: &ash::Device, set: vk::DescriptorSet, ユニフォーム: vk::Buffer, 書き込み先: vk::ImageView) {
    let バッファ情報 = [vk::DescriptorBufferInfo::default().buffer(ユニフォーム).offset(0).range(vk::WHOLE_SIZE)];
    let 画像情報 = [vk::DescriptorImageInfo::default()
        .image_view(書き込み先)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み一覧 = [
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&バッファ情報),
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .image_info(&画像情報),
    ];
    // 安全性: setは割当済み、ユニフォームと画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
