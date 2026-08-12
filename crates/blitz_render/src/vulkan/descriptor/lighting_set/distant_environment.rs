//! 照明問い合わせのセットのうち、遠方環境の枝だけが持つ3つの束縛先。触れるのは拡散照度の立方体画像(binding4)・
//! 鏡面畳込みの立方体画像(binding5)・反射率積分表(binding6)であり、直接光と影の束縛先には触れない。
//!
//! 不変条件: レイアウトがこの3つを宣言したセットは、使う前に必ず3つとも結ばれている。定数近似の枝はこの3つを
//! 1つも宣言しないため、未使用のダミー束縛を作らない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」)。
//!
//! 注意: ディスクリプタのレイアウトはGENERALである。3つの画像は焼き直さないフレームも中身を保ち、
//! 休むレイアウトをGENERALに固定する不変条件を大気のベイク済み画像と共有するためである
//! (参照: `crates/blitz_render/src/vulkan/graph/initial_state/atmosphere_lut.rs`)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) const 拡散照度の束縛番号: u32 = 4;
pub(crate) const 鏡面畳込みの束縛番号: u32 = 5;
pub(crate) const 反射率積分表の束縛番号: u32 = 6;

/// 3つの画像ビュー。どれも消費側が向きと粗さで参照する全段を含むビューである。
#[derive(Clone, Copy)]
pub(crate) struct 遠方環境の束縛先 {
    pub(crate) 拡散照度: vk::ImageView,
    pub(crate) 鏡面畳込み: vk::ImageView,
    pub(crate) 反射率積分表: vk::ImageView,
}

/// 3つの画像を読む1つの固定サンプラー。段の間を補間するのは、粗さが段の間の値を取るためである。
/// 縁を伸ばすのは、反射率積分表の端(粗さ0と1、視線が面と平行)で表の外を読ませないためである。
pub(crate) fn サンプラーを生成する(確保係: &GPU資源の確保係<'_>) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .min_lod(0.0)
        .max_lod(vk::LOD_CLAMP_NONE)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false);
    確保係.標本の取り方からサンプラーを確保する(&create_info)
}

pub(crate) fn サンプラーを破棄する(device: &ash::Device, サンプラー: vk::Sampler) {
    // 安全性: サンプラーは呼び出し元が唯一の所有者であり、破棄時点でGPU側の使用完了を保証する。
    unsafe { device.destroy_sampler(サンプラー, None) };
}

pub(super) fn バインド一覧() -> [vk::DescriptorSetLayoutBinding<'static>; 3] {
    [
        画素段の画像バインド(拡散照度の束縛番号),
        画素段の画像バインド(鏡面畳込みの束縛番号),
        画素段の画像バインド(反射率積分表の束縛番号),
    ]
}

/// 3つの画像は焼き直しても束縛先のビューが変わらない固定資源のため、生成時に一度だけ結べばよい。
pub(crate) fn 結ぶ(device: &ash::Device, set: vk::DescriptorSet, 束縛先: 遠方環境の束縛先, サンプラー: vk::Sampler) {
    let 対応 = [
        (拡散照度の束縛番号, 束縛先.拡散照度),
        (鏡面畳込みの束縛番号, 束縛先.鏡面畳込み),
        (反射率積分表の束縛番号, 束縛先.反射率積分表),
    ];
    for (番号, ビュー) in 対応 {
        let 画像情報一覧 = [vk::DescriptorImageInfo::default()
            .sampler(サンプラー)
            .image_view(ビュー)
            .image_layout(vk::ImageLayout::GENERAL)];
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(番号)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&画像情報一覧)];
        // 安全性: setは割当済み、画像ビューとサンプラーは生成済みで有効。
        unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
    }
}

fn 画素段の画像バインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
