//! 点光源の影の比較サンプラー(PCF用)の確保。

use ash::vk;

use crate::error::レンダラーエラー;

/// PCF用の比較サンプラー。深度比較をハードウェアで行い、比較の向きは順Zに合わせて`LESS_OR_EQUAL`にする
/// (記録された遮蔽より手前か等しければ照らされる。多段影と同じ読み方である)。
///
/// 端の扱いをCLAMP_TO_EDGEにするのは、立方体として標本する限り面の境界がハードウェアの側で
/// 隣の面へつながり、範囲の外という状態がそもそも現れないためである。多段影が使う「範囲外は影なし」の
/// 縁の色は、正射影の外側を持つあちら固有の必要である(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断o」)。
pub(super) fn 比較サンプラーを作る(device: &ash::Device) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .compare_enable(true)
        .compare_op(vk::CompareOp::LESS_OR_EQUAL)
        .min_lod(0.0)
        .max_lod(0.0)
        .unnormalized_coordinates(false);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}
