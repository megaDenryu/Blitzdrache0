//! バッファを指すバインディングの書き込み。フレームシェーダー定数(binding3)・描画対象シェーダー定数(binding5)・
//! 個体変換(binding6)・可視ID列(binding7)の4つを担当し、画像バインディングには触れない。

use ash::vk;

/// binding3(フレームシェーダー定数バッファ)を書き込む。生成時にセットごとの固有バッファを結ぶ。
pub(super) fn フレームシェーダー定数を書き込む(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer) {
    シェーダー定数バッファを書き込む(device, set, 3, buffer);
}

/// binding5へ描画対象固有の変換とマテリアル係数を結ぶ。
pub(super) fn 描画対象シェーダー定数を書き込む(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer) {
    シェーダー定数バッファを書き込む(device, set, 5, buffer);
}

/// binding6へ個体変換のストレージバッファを結ぶ。個体が1体だけの対象は描画対象シェーダー定数のバッファを先頭112バイトの範囲で指す。
pub(super) fn 個体変換を書き込む(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer, 範囲: vk::DeviceSize) {
    ストレージバッファを書き込む(device, set, 6, buffer, 範囲);
}

/// binding7へ可視ID列のストレージバッファを結ぶ。セットはフレームスロットごとに別のため、そのスロットのバッファを結ぶ。
pub(super) fn 可視id列を書き込む(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer, 範囲: vk::DeviceSize) {
    ストレージバッファを書き込む(device, set, 7, buffer, 範囲);
}

fn シェーダー定数バッファを書き込む(device: &ash::Device, set: vk::DescriptorSet, binding: u32, buffer: vk::Buffer) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
    let 書き込み一覧 = [書き込みを作る(set, binding, vk::DescriptorType::UNIFORM_BUFFER, &buffer情報一覧)];
    // 安全性: setは割当済み、bufferは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn ストレージバッファを書き込む(
    device: &ash::Device, set: vk::DescriptorSet, binding: u32, buffer: vk::Buffer, 範囲: vk::DeviceSize
) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(範囲)];
    let 書き込み一覧 = [書き込みを作る(set, binding, vk::DescriptorType::STORAGE_BUFFER, &buffer情報一覧)];
    // 安全性: setは割当済み、bufferは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn 書き込みを作る<'a>(
    set: vk::DescriptorSet,
    binding: u32,
    種別: vk::DescriptorType,
    情報: &'a [vk::DescriptorBufferInfo],
) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(binding)
        .dst_array_element(0)
        .descriptor_type(種別)
        .buffer_info(情報)
}
