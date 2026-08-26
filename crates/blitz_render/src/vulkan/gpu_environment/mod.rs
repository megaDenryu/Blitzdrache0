//! Vulkanの能力そのものを表す不変ハンドルの束: ローダー・インスタンス・validationメッセンジャー・
//! サーフェス・物理デバイス・論理デバイス・グラフィックスキュー・スワップチェーンローダー。
//! どのハンドルも生成後に変更されず、破棄はvkDestroyDevice→vkDestroySurfaceKHR→メッセンジャー→
//! vkDestroyInstanceという1つの順序制約を共有するため、その順序をこの型の内部に閉じる。
//! 生成と環境を材料にする資源の生成は`create`、破棄は`destroy`、物理デバイスへの問い合わせは`physical_query`が担う。
//!
//! `entry`はフィールドとして値が読まれることはないが、破棄まで保持し続けることに意味がある。ash::Entryを破棄すると
//! Vulkanローダー(vulkan-1.dll)がアンロードされ得るため、instance/deviceの関数ポインタが無効化される前に
//! entryを先に破棄してはならない。
//! `実表示計測状況`は論理デバイス生成時に実表示計測の2拡張を有効化したかの判定結果である。有効化していない拡張の関数を
//! 呼ばせないため、判定と実表示計測の生成をこの型で結び付ける。
//! `ディスクリプタ索引上限`は選定した物理デバイスから採取した、索引化した材質テクスチャ表の容量に効く上限である。
//! デバイスが変われば変わる値であり、デバイスと同じ寿命でここに保つ。
//!
//! 注意: 寸法変更で作り直す資源(スワップチェーン・深度バッファ・提示同期)はこの束に入れない。
//! 可変性の寿命が違い、混ぜるとこの束の破棄の途中にvkDestroyDeviceが挟まる。

mod create;
mod destroy;
mod physical_query;

use ash::vk;

use crate::descriptor_indexing_limits::ディスクリプタ索引上限;
use crate::error::レンダラーエラー;
use crate::present_display::実表示計測状況;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::debug_messenger::デバッグメッセンジャー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use physical_query::物理デバイス問い合わせ;

pub(crate) struct GPU環境 {
    #[allow(dead_code)]
    entry: ash::Entry, // 注意: instance/deviceより先に破棄してはならない
    instance: ash::Instance,
    デバッグメッセンジャー: Option<デバッグメッセンジャー>,
    surface_loader: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: GPUデバイス,
    queue: vk::Queue,
    キューファミリ添字: u32,
    swapchain_loader: ash::khr::swapchain::Device,
    実表示計測状況: 実表示計測状況,                 // 実表示計測の2拡張を有効化したかの判定結果
    ディスクリプタ索引上限: ディスクリプタ索引上限, // 材質テクスチャ表の容量に効く物理デバイスの上限
}

impl GPU環境 {
    pub(crate) fn device(&self) -> &GPUデバイス {
        &self.device
    }

    pub(crate) fn queue(&self) -> vk::Queue {
        self.queue
    }

    /// 選定した物理デバイスから採取した上限。印字の書式は上位層が決めるため、値だけを渡す。
    pub(crate) fn ディスクリプタ索引上限(&self) -> ディスクリプタ索引上限 {
        self.ディスクリプタ索引上限
    }

    /// 選定済みキューファミリの添字。コマンドプールと転送環境の生成に要る。
    pub(crate) fn キューファミリ添字(&self) -> u32 {
        self.キューファミリ添字
    }

    /// スワップチェーンの破棄と提示に貸す。生成は`スワップチェーンを作る`が内部で貸すため要らない。
    pub(crate) fn swapchain_loader(&self) -> &ash::khr::swapchain::Device {
        &self.swapchain_loader
    }

    pub(crate) fn 物理デバイス問い合わせ(&self) -> 物理デバイス問い合わせ<'_> {
        物理デバイス問い合わせ::生成する(&self.instance, self.physical_device)
    }

    /// GPU資源を確保する道具を貸す。論理デバイスとメモリ性質をこの環境が持ち続けるため、
    /// 確保を頼む側はこの2つを引数で運ばずに済む。
    pub(crate) fn 資源の確保係を貸す(&self) -> GPU資源の確保係<'_> {
        GPU資源の確保係::生成する(&self.device, self.物理デバイス問い合わせ().メモリプロパティを取得する())
    }

    /// GPU上の全作業の完了を待つ。使用中リソースの破棄によるvalidationエラーを避けるため、
    /// 資源の破棄・差し替えの前に呼ぶ。
    pub(crate) fn gpuの全作業完了を待つ(&self) -> Result<(), レンダラーエラー> {
        // 安全性: deviceはSelfが唯一の所有者で、待機の対象はこの環境へ投入した作業だけである。
        unsafe { self.device.device_wait_idle()? };
        Ok(())
    }
}
