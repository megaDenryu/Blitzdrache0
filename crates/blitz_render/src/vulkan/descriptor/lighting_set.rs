//! set3(照明問い合わせのセット)のレイアウトと、そのセットへ資源を結ぶ操作。触れるのはシャドウマップの比較サンプラー(binding0)・
//! ヘッダの定数バッファ(binding1)・方向光レコード列(binding2)・局所光レコード列(binding3)の4つと、両方の契約が持つ
//! 局所可視度の画像(binding7、`local_visibility`が担う)、遠方環境の枝だけが持つ3つ(binding4から6、`distant_environment`が担う)である。
//! 4つを同じセットへ置くのは、どれも寿命がフレーム×ビューであり、直接光と直接影が同じ問い合わせ契約に属するためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「照明問い合わせ資源のGPU境界」)。番号の正本は`shaders/lighting_query.slang`の宣言である。

mod buffer_group;
pub(crate) mod distant_environment;
pub(crate) mod local_visibility;
mod pool;

use ash::vk;

pub(crate) use buffer_group::照明問い合わせのバッファ組;

use crate::error::レンダラーエラー;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;
use crate::vulkan::shadow_map::シャドウマップ;

pub(crate) const シャドウマップのバインディング番号: u32 = 0;
pub(crate) const ヘッダのバインディング番号: u32 = 1;
pub(crate) const 方向光列のバインディング番号: u32 = 2;
pub(crate) const 局所光列のバインディング番号: u32 = 3;

/// 束縛レイアウトの枝ごとに宣言するバインドの並びを変える。定数近似の枝へ未使用のダミー束縛を強制しない。
pub(super) fn レイアウトを生成する(
    device: &ash::Device,
    束縛レイアウト: 照明束縛レイアウト,
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let mut バインド一覧 = vec![
        画素段のバインド(シャドウマップのバインディング番号, vk::DescriptorType::COMBINED_IMAGE_SAMPLER),
        画素段のバインド(ヘッダのバインディング番号, vk::DescriptorType::UNIFORM_BUFFER),
        画素段のバインド(方向光列のバインディング番号, vk::DescriptorType::STORAGE_BUFFER),
        画素段のバインド(局所光列のバインディング番号, vk::DescriptorType::STORAGE_BUFFER),
    ];
    if 束縛レイアウト.遠方環境の画像を結ぶか() {
        バインド一覧.extend(distant_environment::バインド一覧());
    }
    バインド一覧.push(local_visibility::バインド());
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// 進行中フレームスロットの数だけセットを取り出せるプール。件数の決め方は`pool`が持つ。
pub(crate) fn プールを生成する(
    device: &ash::Device,
    スロット数: usize,
    束縛レイアウト: 照明束縛レイアウト,
) -> Result<vk::DescriptorPool, レンダラーエラー> {
    pool::生成する(device, スロット数, 束縛レイアウト)
}

/// そのスロットの3本のバッファと、全スロットで共通のシャドウマップを結ぶ。
pub(crate) fn 資源を結ぶ(
    device: &ash::Device, set: vk::DescriptorSet, バッファ組: 照明問い合わせのバッファ組, シャドウマップ: &シャドウマップ
) {
    シャドウマップを結ぶ(device, set, シャドウマップ);
    let 対応 = [
        (ヘッダのバインディング番号, vk::DescriptorType::UNIFORM_BUFFER, バッファ組.ヘッダ),
        (方向光列のバインディング番号, vk::DescriptorType::STORAGE_BUFFER, バッファ組.方向光列),
        (局所光列のバインディング番号, vk::DescriptorType::STORAGE_BUFFER, バッファ組.局所光列),
    ];
    for (番号, 種別, buffer) in 対応 {
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
}

/// 束縛するのは距離区分ごとの層でなく配列全体のビューであり、距離区分の選択はシェーダーが層の添字で行う。
/// シャドウマップはスワップチェーン再構築と独立の固定資源のため、生成時に一度だけ結べばよい。
fn シャドウマップを結ぶ(device: &ash::Device, set: vk::DescriptorSet, シャドウマップ: &シャドウマップ) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(シャドウマップ.sampler)
        .image_view(シャドウマップ.配列ビュー)
        .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(シャドウマップのバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、シャドウマップの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn 画素段のバインド(binding: u32, 種別: vk::DescriptorType) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(種別)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
