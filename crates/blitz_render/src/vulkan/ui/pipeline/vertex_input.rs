//! `UI頂点`のメモリレイアウトに対応するバインド記述・属性記述の組み立て。

use ash::vk;

use crate::ui_vertex::UI頂点;

pub(super) fn ui頂点のバインド記述と属性記述を組み立てる() -> (vk::VertexInputBindingDescription, [vk::VertexInputAttributeDescription; 3]) {
    let stride = u32::try_from(std::mem::size_of::<UI頂点>()).unwrap_or_else(|_| panic!("UI頂点のサイズがu32に収まらない"));
    let uv開始位置 = u32::try_from(std::mem::offset_of!(UI頂点, uv)).unwrap_or_else(|_| panic!("UI頂点のuv開始位置がu32に収まらない"));
    let 色開始位置 = u32::try_from(std::mem::offset_of!(UI頂点, 色rgba8)).unwrap_or_else(|_| panic!("UI頂点の色開始位置がu32に収まらない"));

    let バインド記述 = vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(stride)
        .input_rate(vk::VertexInputRate::VERTEX);
    let 属性記述一覧 = [
        vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0),
        vk::VertexInputAttributeDescription::default()
            .location(1)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(uv開始位置),
        vk::VertexInputAttributeDescription::default()
            .location(2)
            .binding(0)
            .format(vk::Format::R8G8B8A8_UNORM)
            .offset(色開始位置),
    ];
    (バインド記述, 属性記述一覧)
}
