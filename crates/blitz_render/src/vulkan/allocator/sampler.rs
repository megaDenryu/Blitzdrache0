//! 画像を読むときの標本の取り方を決めるサンプラーの確保。ここが持つ取り方は2つであり、どちらも縁の外を縁の色で埋める(CLAMP_TO_EDGE)。
//!
//! 取り方そのものは、深度の比較や材質テクスチャ表のように読む側の都合で決まるため、読む側の階層が組んで
//! `標本の取り方からサンプラーを確保する`へ渡す。確保の口をここ1つに閉じるのは、論理デバイスを読む側へ運ばせないためである。

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

    /// 読む側が組んだ標本の取り方でサンプラーを確保する。比較サンプラーや材質テクスチャ表のサンプラーのように、
    /// 取り方の理由を読む側の階層が持つものがこの口を通る。
    pub(crate) fn 標本の取り方からサンプラーを確保する(
        &self,
        取り方: &vk::SamplerCreateInfo<'_>,
    ) -> Result<vk::Sampler, レンダラーエラー> {
        // 安全性: deviceは生成済みで有効。取り方は呼び出し元がこの呼び出しの間だけ生存する値として組み立てる。
        Ok(unsafe { self.device.create_sampler(取り方, None)? })
    }

    fn サンプラーを作る(&self, テクセルの混ぜ方: vk::Filter) -> Result<vk::Sampler, レンダラーエラー> {
        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(テクセルの混ぜ方)
            .min_filter(テクセルの混ぜ方)
            .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        self.標本の取り方からサンプラーを確保する(&create_info)
    }
}
