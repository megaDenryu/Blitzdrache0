//! `頂点`のメモリレイアウトに対応するバインド記述・属性記述の組み立て。

use ash::vk;

use crate::vertex::頂点;

pub(super) fn 記述する() -> (vk::VertexInputBindingDescription, [vk::VertexInputAttributeDescription; 4]) {
    let stride = u32::try_from(std::mem::size_of::<頂点>())
        .unwrap_or_else(|_| panic!("頂点のサイズがu32に収まらない"));
    let 法線オフセット = u32::try_from(std::mem::offset_of!(頂点, 法線))
        .unwrap_or_else(|_| panic!("頂点の法線オフセットがu32に収まらない"));
    let 接線オフセット = u32::try_from(std::mem::offset_of!(頂点, 接線))
        .unwrap_or_else(|_| panic!("頂点の接線オフセットがu32に収まらない"));
    let uvオフセット = u32::try_from(std::mem::offset_of!(頂点, uv))
        .unwrap_or_else(|_| panic!("頂点のuvオフセットがu32に収まらない"));

    let バインド記述 = vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(stride)
        .input_rate(vk::VertexInputRate::VERTEX);
    let 属性記述一覧 = [
        vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0),
        vk::VertexInputAttributeDescription::default()
            .location(1)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(法線オフセット),
        vk::VertexInputAttributeDescription::default()
            .location(2)
            .binding(0)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(接線オフセット),
        vk::VertexInputAttributeDescription::default()
            .location(3)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(uvオフセット),
    ];
    (バインド記述, 属性記述一覧)
}

/// 布描画用: 同じ48バイトレイアウトから位置・法線・uvの3属性だけを宣言する(判断54)。
/// 布シェーダーは接線を消費しないため、宣言すると未消費属性のvalidation警告になる。
pub(super) fn 布用記述する() -> (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>) {
    let (バインド記述, 全属性) = 記述する();
    let uv = 全属性[3].location(2);
    (バインド記述, vec![全属性[0], 全属性[1], uv])
}

/// 属性選択に応じたバインド・属性記述を返す(graphics_pipelineの行数分割のための集約)。
pub(super) fn 選択して記述する(選択: super::頂点属性選択) -> (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>) {
    match 選択 {
        super::頂点属性選択::全属性 => {
            let (バインド, 属性) = 記述する();
            (バインド, 属性.to_vec())
        }
        super::頂点属性選択::布用 => 布用記述する(),
    }
}
