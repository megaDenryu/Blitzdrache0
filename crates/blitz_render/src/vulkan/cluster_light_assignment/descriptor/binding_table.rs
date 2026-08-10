//! 生成側のセットが持つ束縛の並びと、そこへ現物を書き込む操作。触れるのは番号と種別の対応だけであり、
//! プールもパイプラインも知らない。
//!
//! 番号を並びの位置そのものにするのは、`shaders/cluster_light_assignment.slang`の宣言が0から連番だからである。
//! 並びはヘッダ・局所光レコード列・クラスタ格子・クラスタ光添字列であり、前の2つは照明問い合わせのセットが
//! 画素段へ結ぶのと同じバッファを生成側へも結んだものである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::照明問い合わせのバッファ組;

/// 束縛の種別の並び。位置がそのまま番号になる。
pub(super) const 束縛の種別一覧: [vk::DescriptorType; 4] = [
    vk::DescriptorType::UNIFORM_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
];

pub(super) fn セットレイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧: Vec<vk::DescriptorSetLayoutBinding<'_>> = 束縛の種別一覧
        .iter()
        .enumerate()
        .map(|(位置, &種別)| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(束縛番号にする(位置))
                .descriptor_type(種別)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        })
        .collect();
    let 生成情報 = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&生成情報, None)? })
}

/// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後)。
pub(super) fn 資源を結ぶ(device: &ash::Device, セット: vk::DescriptorSet, バッファ組: 照明問い合わせのバッファ組) {
    let 並び = [
        バッファ組.ヘッダ,
        バッファ組.局所光列,
        バッファ組.クラスタ格子,
        バッファ組.クラスタ光添字列,
    ];
    for (位置, buffer) in 並び.into_iter().enumerate() {
        let 情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(セット)
            .dst_binding(束縛番号にする(位置))
            .dst_array_element(0)
            .descriptor_type(束縛の種別一覧[位置])
            .buffer_info(&情報一覧)];
        // 安全性: セットは割当済み、bufferは生成済みで有効。
        unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
    }
}

fn 束縛番号にする(位置: usize) -> u32 {
    u32::try_from(位置).unwrap_or_else(|_| panic!("選別の束縛番号がu32に収まらない: {位置}"))
}
