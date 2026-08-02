//! set1(ジオメトリと可視のセット)のレイアウトと、そのセットへ2本のストレージバッファを結ぶ操作。触れるのは
//! 個体レコード(binding0)と可視ID列(binding1)の2つだけである。
//! 2本を同じセットへ置くのは、どちらも頂点段が描画対象ごとに読み、束縛の単位が描画対象で一致するためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 番号の正本は`shaders/instance_transform.slang`・`visible_id.slang`の宣言である。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) const 個体レコードのバインディング番号: u32 = 0;
pub(crate) const 可視ID列のバインディング番号: u32 = 1;

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        頂点段のストレージバッファバインド(個体レコードのバインディング番号),
        頂点段のストレージバッファバインド(可視ID列のバインディング番号),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// 個体が1体だけの対象も1要素ぶんの範囲を持つ専用のバッファを結び、可視ID列はそのフレームスロットのバッファを結ぶ。
pub(super) fn 資源を結ぶ(
    device: &ash::Device,
    set: vk::DescriptorSet,
    個体レコード: (vk::Buffer, vk::DeviceSize),
    可視id列: (vk::Buffer, vk::DeviceSize),
) {
    let 対応 = [(個体レコードのバインディング番号, 個体レコード), (可視ID列のバインディング番号, 可視id列)];
    for (番号, (buffer, 範囲)) in 対応 {
        let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(範囲)];
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(番号)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffer情報一覧)];
        // 安全性: setは割当済み、bufferは生成済みで有効。
        unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
    }
}

fn 頂点段のストレージバッファバインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
}
