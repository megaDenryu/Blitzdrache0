//! 実表示時刻の計測。提示に単調増加のIDを付け、VK_KHR_present_waitで表示完了を観測する。
//!
//! CPU側フレーム開始間隔はFIFO提示待ちを含むため、実表示時刻の更新脱落と同一ではない。
//! 参照: `_doc/計測/OW2_2026-07-25.md`「実表示時刻の計測(VK_KHR_present_id + VK_KHR_present_wait)」。

mod record;
mod support;
mod wait;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::present_display::実表示観測;
use crate::present_display::実表示計測状況;
use record::表示時刻記録;

pub(in crate::vulkan) use support::{有効化する拡張名一覧, 調べる};

/// 実表示計測の使用可否。非対応の理由まで型で保持し、記録は使用可能なときだけ持つ。
pub(crate) enum 実表示計測 {
    要求していない,
    拡張が無い,
    機能が無効,
    使用可能 {
        待機: ash::khr::present_wait::Device,
        記録: Option<表示時刻記録>,
    },
}

impl 実表示計測 {
    pub(crate) fn 生成する(instance: &ash::Instance, device: &ash::Device, 状況: 実表示計測状況) -> Self {
        match 状況 {
            実表示計測状況::要求していない => Self::要求していない,
            実表示計測状況::拡張が無い => Self::拡張が無い,
            実表示計測状況::機能が無効 => Self::機能が無効,
            実表示計測状況::計測できる => Self::使用可能 {
                待機: ash::khr::present_wait::Device::new(instance, device),
                記録: None,
            },
        }
    }

    pub(crate) fn 状況(&self) -> 実表示計測状況 {
        match self {
            Self::要求していない => 実表示計測状況::要求していない,
            Self::拡張が無い => 実表示計測状況::拡張が無い,
            Self::機能が無効 => 実表示計測状況::機能が無効,
            Self::使用可能 { .. } => 実表示計測状況::計測できる,
        }
    }

    /// 記録を開始する。非対応環境では何も始めず、対応状況を返して呼び出し元に判定させる。
    pub(crate) fn 記録を開始する(&mut self, ウォームアップ観測数: u32, 標本容量: usize) -> 実表示計測状況 {
        if let Self::使用可能 { 記録, .. } = self {
            *記録 = Some(表示時刻記録::生成する(ウォームアップ観測数, 標本容量));
        }
        self.状況()
    }

    /// 提示に付けるIDを発番する。記録していないときは`None`を返し、提示情報にIDを連結させない。
    pub(crate) fn 提示idを発番する(&mut self) -> Option<u64> {
        match self {
            Self::要求していない | Self::拡張が無い | Self::機能が無効 => None,
            Self::使用可能 { 記録, .. } => 記録.as_mut().map(表示時刻記録::発番する),
        }
    }

    pub(crate) fn 表示を待って記録する(&mut self, swapchain: vk::SwapchainKHR) -> Result<(), レンダラーエラー> {
        let Self::使用可能 {
            待機, 記録: Some(記録)
        } = self
        else {
            return Ok(());
        };
        wait::表示を待って記録する(待機, 記録, swapchain)
    }

    /// 提示IDはスワップチェーンごとに単調増加する必要があるため、作り直したら追跡を捨てる。
    pub(crate) fn スワップチェーンを作り直した(&mut self) {
        if let Self::使用可能 { 記録: Some(記録), .. } = self {
            記録.提示の追跡をやり直す();
        }
    }

    pub(crate) fn 観測一覧(&self) -> &[実表示観測] {
        match self {
            Self::要求していない | Self::拡張が無い | Self::機能が無効 => &[],
            Self::使用可能 { 記録, .. } => 記録.as_ref().map_or(&[], 表示時刻記録::観測一覧),
        }
    }
}
