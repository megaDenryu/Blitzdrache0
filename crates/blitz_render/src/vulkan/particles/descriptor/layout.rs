//! 粒子ディスクリプタセットレイアウト: binding0=uniform buffer(ビュー定数を共有、VERTEXのみ)、
//! binding3=storage buffer(粒子、COMPUTE|VERTEX)。
//! 番号0はshaders/view_uniform.slangの宣言に合わせる。粒子・表面流・SPHの頂点段はこの1つの宣言を取り込むため、
//! scene系のビューとパスのセットと同じ番号でなければ束縛できない。粒子のバッファをscene系が使わない番号3へ置くのは、
//! シミュレーションのコンピュートパイプラインへ空のセットを強制せずに、同じセットへ2つを同居させるためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX),
        vk::DescriptorSetLayoutBinding::default()
            .binding(3)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE | vk::ShaderStageFlags::VERTEX),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}
