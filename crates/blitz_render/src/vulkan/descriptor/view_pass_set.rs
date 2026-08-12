//! set0(ビューとパスのセット)のレイアウトと、そのセットへシェーダー定数3本を結ぶ操作。触れるのは
//! ビュー定数(binding0)・多段影定数(binding1)・空パス定数(binding2)の3つだけである。
//! 3本を1つのセットへ置くのは、どれも寿命がフレーム×ビューであり束縛頻度が同じためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 番号の正本は`shaders/view_uniform.slang`・`cascade_shadow_uniform.slang`・`sky_pass_uniform.slang`の宣言である。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(crate) const ビュー定数の束縛番号: u32 = 0;
pub(crate) const 多段影の定数の束縛番号: u32 = 1;
pub(crate) const 空パスの定数の束縛番号: u32 = 2;

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        定数バインド(ビュー定数の束縛番号),
        定数バインド(多段影の定数の束縛番号),
        定数バインド(空パスの定数の束縛番号),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// そのフレームスロットの定数3本を結ぶ。空段階を持たない構成でも空パス定数の束縛先は結ぶ(中身は書かれず誰も読まない)。
pub(super) fn 定数を結ぶ(
    device: &ash::Device, set: vk::DescriptorSet, 定数: &フレームシェーダー定数一式, フレーム添字: フレームスロット添字
) {
    let 対応 = [
        (ビュー定数の束縛番号, 定数.ビュー定数のbuffer(フレーム添字)),
        (多段影の定数の束縛番号, 定数.多段影のbuffer(フレーム添字)),
        (空パスの定数の束縛番号, 定数.空パスのbuffer(フレーム添字)),
    ];
    for (番号, buffer) in 対応 {
        let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(番号)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer情報一覧)];
        // 安全性: setは割当済み、bufferは生成済みで有効。
        unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
    }
}

/// ビュー射影行列を頂点段が、影の判定と空の視線復元を画素段が読むため、3本とも両ステージへ宣言する。
fn 定数バインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
}
