//! 粒子ディスクリプタセットレイアウト: binding0=storage buffer(粒子、COMPUTE|VERTEX)、
//! binding3=uniform buffer(ビュー・シーンパス定数を共有、VERTEXのみ)。
//! 番号3はshaders/view_pass_uniform.slangの宣言に合わせる。粒子・表面流・SPHの頂点段は
//! この1つの宣言を取り込むため、シーンと同じ番号でなければ束縛できない。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE | vk::ShaderStageFlags::VERTEX),
        vk::DescriptorSetLayoutBinding::default()
            .binding(3)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}
