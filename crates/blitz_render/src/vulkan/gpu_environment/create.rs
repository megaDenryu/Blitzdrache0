//! 生成局面: 起動時に1度だけ組み立てるGPU環境自身と、その保持ハンドルを材料にして作る資源
//! (実表示計測・スワップチェーン)の生成。実表示計測の対応判定は論理デバイス生成より前に行う。
//! 有効化していない拡張の機能構造体を連結するとデバイス生成が失敗するため、判定と有効化を離せない。

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use super::GPU環境;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::present_display::実表示計測要求;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;
use crate::vulkan::present_timing::実表示計測;
use crate::vulkan::swapchain::スワップチェーン;

impl GPU環境 {
    /// 前提: `表示ハンドル`と`ウィンドウハンドル`の指すウィンドウは、この環境より長生きすることを呼び出し元が保証する。
    pub(crate) fn 生成する(
        表示ハンドル: RawDisplayHandle,
        ウィンドウハンドル: RawWindowHandle,
        検証カウンタ: &検証カウンタ,
        実表示計測要求: 実表示計測要求,
    ) -> Result<Self, レンダラーエラー> {
        let デバッグ有効か = cfg!(debug_assertions);
        // 安全性: プロセス内で他にVulkanローダーを読み込んでいないことは
        // blitz_appがコンポジションルートとして唯一のレンダラーのみ生成することで保証する。
        let entry = unsafe { ash::Entry::load()? };

        let instance = vulkan::instance::生成する(&entry, 表示ハンドル, デバッグ有効か)?;
        let デバッグメッセンジャー = デバッグ有効か
            .then(|| vulkan::debug_messenger::デバッグメッセンジャー::生成する(&entry, &instance, 検証カウンタ))
            .transpose()?;

        let (surface_loader, surface) = vulkan::surface::生成する(&entry, &instance, 表示ハンドル, ウィンドウハンドル)?;
        let (physical_device, キューファミリ添字) =
            vulkan::physical_device::物理デバイスとキューファミリを選定する(&instance, &surface_loader, surface)?;
        // 索引の機能は選定が候補を絞る条件として見ており、ここへ来た時点で選ばれた1台は満たしている。
        // ここで読むのは、その1台の表容量に効く上限だけである。
        let ディスクリプタ索引上限 = vulkan::descriptor_indexing::上限を採取する(&instance, physical_device);
        let 大点描画対応 = vulkan::physical_device::大きな点描画に対応するか(&instance, physical_device);
        let 実表示計測状況 = vulkan::present_timing::調べる(&instance, physical_device, 実表示計測要求);
        let (device, queue) = vulkan::device::生成する(&instance, physical_device, キューファミリ添字, 大点描画対応, 実表示計測状況)?;
        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        Ok(Self {
            entry,
            instance,
            デバッグメッセンジャー,
            surface_loader,
            surface,
            physical_device,
            device,
            queue,
            キューファミリ添字,
            swapchain_loader,
            実表示計測状況,
            ディスクリプタ索引上限,
        })
    }

    /// 論理デバイスで実際に有効化した拡張と一致する実表示計測を作る。
    pub(crate) fn 実表示計測を作る(&self) -> 実表示計測 {
        実表示計測::生成する(&self.instance, &self.device, self.実表示計測状況)
    }

    /// 物理デバイス・サーフェス・両ローダーをスワップチェーン生成へ貸す。呼び出し側は寸法と旧スワップチェーンだけを決める。
    /// 旧スワップチェーンを渡すとVulkanのoldSwapchain経路で作り直す。破棄を先に済ませる呼び出し元は`None`を渡す。
    pub(crate) fn スワップチェーンを作る(
        &self,
        要求寸法: ウィンドウ寸法,
        旧スワップチェーン: Option<&スワップチェーン>,
    ) -> Result<スワップチェーン, レンダラーエラー> {
        スワップチェーン::生成する(
            self.physical_device,
            &self.device,
            &self.surface_loader,
            self.surface,
            &self.swapchain_loader,
            要求寸法,
            旧スワップチェーン.map_or(vk::SwapchainKHR::null(), |前の| 前の.handle),
        )
    }
}
