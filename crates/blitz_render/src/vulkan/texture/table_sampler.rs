//! 材質テクスチャ表の全画像を読む1つの固定サンプラー。担当するのは、標本化の設定をテクスチャの枚数から切り離すことである。
//!
//! 表の要素ごとにサンプラーを複製しないのは、標本化の設定が画像の中身によらず同じであり、複製するとサンプラーの
//! 上限を枚数に比例して消費するためである(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」)。
//! 注意: 縮小段の上限を打ち切らない。表には縮小段数の違う画像が並ぶため、1枚ぶんの段数で切ると別の画像の粗い段が読めない。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 生成する(device: &ash::Device) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .min_lod(0.0)
        .max_lod(vk::LOD_CLAMP_NONE)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}

/// 注意: このサンプラーを固定サンプラーとして宣言したセットレイアウトをすべて破棄した後に呼ぶ。
pub(crate) fn 破棄する(device: &ash::Device, サンプラー: vk::Sampler) {
    // 安全性: サンプラーは呼び出し元が唯一の所有者であり、破棄時点でGPU側の使用完了を保証する。
    unsafe { device.destroy_sampler(サンプラー, None) };
}
