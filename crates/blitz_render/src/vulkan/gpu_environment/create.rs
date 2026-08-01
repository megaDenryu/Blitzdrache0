//! 生成局面: 起動時に1度だけ組み立てるGPU環境自身と、その保持ハンドルを材料にして作る資源
//! (実表示計測・スワップチェーン)の生成。実表示計測の対応判定は論理デバイス生成より前に行う。
//! 有効化していない拡張の機能構造体を連結するとデバイス生成が失敗するため、判定と有効化を離せない。

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use super::GPU環境;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::present_display_request::実表示計測要求;
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
        let (physical_device, キューファミリ添字) = vulkan::physical_device::選定する(&instance, &surface_loader, surface)?;
        // 選定の条件は変えず、選ばれたデバイスに対して索引化の最低機能要件を照合する。選定の枝で落とすと
        // どの機能が欠けたのかを報告できず、機材を直す手がかりが残らない。
        let (ディスクリプタ索引機能, ディスクリプタ索引上限) = vulkan::descriptor_indexing::採取する(&instance, physical_device);
        ディスクリプタ索引機能.最低要件を照合する()?;
        let 大点描画対応 = vulkan::physical_device::大きな点描画に対応するか(&instance, physical_device);
        let 実表示計測状況 = vulkan::present_timing::調べる(&instance, physical_device, 実表示計測要求);
        let (device, queue) = vulkan::device::生成する(&instance, physical_device, キューファミリ添字, 大点描画対応, 実表示計測状況)?;
        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        let 環境 = Self {
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
        };
        環境.ディスクリプタ索引上限を報告する();
        Ok(環境)
    }

    /// 採取した上限を起動時に1行で残す。合否を判定しないため、判定しないことも同じ行に書いて
    /// 「通ったから足りている」と読まれないようにする。
    fn ディスクリプタ索引上限を報告する(&self) {
        println!(
            "ディスクリプタ索引上限(合否は判定しない): シェーダー段あたりの画像ディスクリプタ{}個 / セットあたりの画像ディスクリプタ{}個",
            self.ディスクリプタ索引上限.シェーダー段あたりの画像ディスクリプタ数(),
            self.ディスクリプタ索引上限.セットあたりの画像ディスクリプタ数()
        );
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
