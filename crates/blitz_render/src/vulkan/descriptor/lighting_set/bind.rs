//! 割り当て済みの照明問い合わせのセットへ現物を書き込む局面。触れるのは1つのセットだけであり、
//! バッファもシャドウマップも引数で受け取る。
//! 呼び出しタイミング: 資源束の生成時に、スロットごとに1回だけである(遠方環境の3つと局所可視度の画像は
//! 束縛先の画像が変わる局面があるため、それぞれの担当モジュールが別に結び直す)。

use ash::vk;

use super::{cluster_grid, point_light_shadow_map, 照明問い合わせのバッファ組};
use crate::vulkan::shadow_resources::影の資源の組;

/// そのスロットのバッファと、全スロットで共通の影の資源の組を結ぶ。
/// クラスタ格子の2本も同じ手順で結ぶのは、どちらも同じスロットが所有するバッファであり、結び直しの局面を持たないためである。
pub(crate) fn 資源を結ぶ(
    device: &ash::Device, set: vk::DescriptorSet, バッファ組: 照明問い合わせのバッファ組, 影の資源: &影の資源の組
) {
    シャドウマップを結ぶ(device, set, &影の資源.多段影);
    point_light_shadow_map::結ぶ(device, set, 影の資源.点光源の影.立方体配列ビュー, 影の資源.点光源の影.sampler);
    let 定数の対 = [(super::ヘッダのバインディング番号, vk::DescriptorType::UNIFORM_BUFFER, バッファ組.ヘッダ)];
    let 記憶の対 = [
        (super::方向光列のバインディング番号, バッファ組.方向光列),
        (super::局所光列のバインディング番号, バッファ組.局所光列),
    ];
    for (番号, 種別, buffer) in 定数の対 {
        バッファを結ぶ(device, set, 番号, 種別, buffer);
    }
    for (番号, buffer) in 記憶の対.into_iter().chain(cluster_grid::番号とバッファの対(バッファ組)) {
        バッファを結ぶ(device, set, 番号, vk::DescriptorType::STORAGE_BUFFER, buffer);
    }
}

fn バッファを結ぶ(device: &ash::Device, set: vk::DescriptorSet, 番号: u32, 種別: vk::DescriptorType, buffer: vk::Buffer) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(番号)
        .dst_array_element(0)
        .descriptor_type(種別)
        .buffer_info(&buffer情報一覧)];
    // 安全性: setは割当済み、bufferは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

/// 束縛するのは距離区分ごとの層でなく配列全体のビューであり、距離区分の選択はシェーダーが層の添字で行う。
/// シャドウマップはスワップチェーン再構築と独立の固定資源のため、生成時に一度だけ結べばよい。
fn シャドウマップを結ぶ(
    device: &ash::Device, set: vk::DescriptorSet, シャドウマップ: &crate::vulkan::shadow_map::シャドウマップ
) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(シャドウマップ.sampler)
        .image_view(シャドウマップ.配列ビュー)
        .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(super::シャドウマップのバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、シャドウマップの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
