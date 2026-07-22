//! シャドウパイプライン専用の頂点入力記述: 位置のみ(location0)。
//! シェーダーは位置しか使わないが、バインド自体は`頂点`構造体全体のstride/offsetを
//! 使う(シーン描画と同じ頂点/インデックスバッファをそのまま束縛できるようにするため)。

use ash::vk;

use crate::vertex::頂点;

pub(super) fn 記述する() -> (vk::VertexInputBindingDescription, [vk::VertexInputAttributeDescription; 1]) {
    let stride = u32::try_from(std::mem::size_of::<頂点>()).unwrap_or_else(|_| panic!("頂点のサイズがu32に収まらない"));
    let 位置オフセット = u32::try_from(std::mem::offset_of!(頂点, 位置)).unwrap_or_else(|_| panic!("頂点の位置オフセットがu32に収まらない"));

    let バインド記述 = vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(stride)
        .input_rate(vk::VertexInputRate::VERTEX);
    let 属性記述一覧 = [vk::VertexInputAttributeDescription::default()
        .location(0)
        .binding(0)
        .format(vk::Format::R32G32B32_SFLOAT)
        .offset(位置オフセット)];
    (バインド記述, 属性記述一覧)
}
