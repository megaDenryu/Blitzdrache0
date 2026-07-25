//! ポスト処理を組むかどうかの判断と、その判断が決まると同時に決まるシーンの描画先色形式(判断38・39)。
//! ポスト処理はシーンをHDR中間画像へ描いてからスワップチェーンへ出す構成のため、組むかどうかがシーンと粒子の描画先形式をそのまま決める。1つの判断から出る2つの答えを別々に持つと食い違うため、1つの型にまとめる。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::frame_composition::{フレーム構成, フレーム段階};
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::hdr_target::HDR形式;
use crate::vulkan::post_process::ポスト処理一式;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) enum 描画先構成 {
    HDR中間画像を経由する,
    スワップチェーンへ直接描く,
}

impl 描画先構成 {
    pub(super) fn 決める(フレーム構成: フレーム構成) -> Self {
        if フレーム構成.含む(フレーム段階::ブルームとトーンマップ) {
            Self::HDR中間画像を経由する
        } else {
            Self::スワップチェーンへ直接描く
        }
    }

    /// シーンと粒子が描き込む色アタッチメントの形式。
    pub(super) fn シーンカラー形式(&self, スワップチェーン画像形式: vk::Format) -> vk::Format {
        match self {
            Self::HDR中間画像を経由する => HDR形式,
            Self::スワップチェーンへ直接描く => スワップチェーン画像形式,
        }
    }

    pub(super) fn 組み立てる(
        &self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        swapchain: &vulkan::swapchain::スワップチェーン,
        シェーダー: &シェーダー束,
    ) -> Result<Option<ポスト処理一式>, レンダラーエラー> {
        match self {
            Self::HDR中間画像を経由する => Ok(Some(ポスト処理一式::生成する(
                device,
                メモリプロパティ,
                swapchain.画像形式,
                swapchain.寸法,
                シェーダー,
            )?)),
            Self::スワップチェーンへ直接描く => Ok(None),
        }
    }
}
