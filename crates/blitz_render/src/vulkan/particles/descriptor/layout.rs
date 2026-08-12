//! 粒子ディスクリプタセットレイアウト: binding0=uniform buffer(ビュー定数を共有、VERTEXのみ)、
//! binding3=storage buffer(粒子、COMPUTE|VERTEX)。
//! 番号0はshaders/view_uniform.slangの宣言に合わせる。粒子・表面流・SPHの頂点段はこの1つの宣言を取り込むため、
//! scene系のビューとパスのセットと同じ番号でなければ束縛できない。粒子のバッファをscene系が使わない番号3へ置くのは、
//! シミュレーションのコンピュートパイプラインへ空のセットを強制せずに、同じセットへ2つを同居させるためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言した束縛の並び, 束縛番号};

/// 束縛の並び。ビュー定数と粒子ストレージバッファの順である。番号が連番でないのは、粒子のバッファを
/// scene系が使わない番号3へ置いているからである。
pub(super) const 束縛の宣言: 宣言した束縛の並び<2> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::UNIFORM_BUFFER, vk::ShaderStageFlags::VERTEX),
    (
        束縛番号::生成する(3),
        vk::DescriptorType::STORAGE_BUFFER,
        vk::ShaderStageFlags::from_raw(vk::ShaderStageFlags::COMPUTE.as_raw() | vk::ShaderStageFlags::VERTEX.as_raw()),
    ),
]);

pub(super) fn 生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = 束縛の宣言.セットレイアウトの宣言();
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}
