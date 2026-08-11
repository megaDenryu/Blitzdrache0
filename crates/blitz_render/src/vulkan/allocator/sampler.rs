//! 画像を読むときの標本の取り方を決めるサンプラーの確保。エンジンが使う取り方は2つだけであり、
//! どちらも縁の外を縁の色で埋める(CLAMP_TO_EDGE)。

use ash::vk;

use super::GPU資源の確保係;
use crate::error::レンダラーエラー;

impl GPU資源の確保係<'_> {
    /// 隣り合うテクセルを混ぜて読むサンプラー。ポストプロセスの中間画像の全画面1対1の読み出しと、
    /// 光のにじみの縮小とぼかしのタップが使う。
    pub(crate) fn 線形サンプラーを作る(&self) -> Result<vk::Sampler, レンダラーエラー> {
        self.サンプラーを作る(vk::Filter::LINEAR)
    }

    /// 混ぜずにテクセルをそのまま読むサンプラー。
    ///
    /// 空中遠近合成が深度をこのサンプラーで参照するのは、隣の画素と深度を混ぜてはならないためである。混ぜると
    /// 面の輪郭で手前の深度と奥の深度の中間の値が出て、そこにだけ実在しない距離の霞が掛かる。
    /// 深度形式(D32_SFLOAT)の線形補間はVulkanの必須機能でもないため、機材の任意機能へも依らせない。
    pub(crate) fn 最近傍サンプラーを作る(&self) -> Result<vk::Sampler, レンダラーエラー> {
        self.サンプラーを作る(vk::Filter::NEAREST)
    }

    fn サンプラーを作る(&self, テクセルの混ぜ方: vk::Filter) -> Result<vk::Sampler, レンダラーエラー> {
        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(テクセルの混ぜ方)
            .min_filter(テクセルの混ぜ方)
            .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        // 安全性: deviceは生成済みで有効。
        Ok(unsafe { self.device.create_sampler(&create_info, None)? })
    }
}
