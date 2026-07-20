//! レンダラーの生成手順。各Vulkanオブジェクトを依存順に組み立てる。
//! スワップチェーン生成後の資源組み立ては`generate_resources`に委ねる。

mod debug_setup;
mod generate_resources;

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use debug_setup::デバッグメッセンジャーを作る;

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::shader_set::シェーダー一式;
use crate::validation_counter::検証カウンタ;
use crate::vertex::頂点;
use crate::vulkan;

impl レンダラー {
    /// Vulkanインスタンス・物理/論理デバイス・スワップチェーン・深度バッファ・
    /// 頂点/インデックスバッファ・コマンドバッファ・同期プリミティブ・
    /// グラフィックスパイプラインを構築する。
    ///
    /// 前提: `表示ハンドル` と `ウィンドウハンドル` の指すウィンドウは、
    /// 戻り値のレンダラーより長生きすること（呼び出し元のフィールド宣言順で担保する）。
    #[allow(clippy::too_many_arguments)]
    pub fn 生成する(
        表示ハンドル: RawDisplayHandle,
        ウィンドウハンドル: RawWindowHandle,
        寸法: ウィンドウ寸法,
        シェーダー: シェーダー一式,
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let デバッグ有効か = cfg!(debug_assertions);
        // 安全性: プロセス内で他にVulkanローダーを読み込んでいないことは
        // blitz_appがコンポジションルートとして唯一のレンダラーのみ生成することで保証する。
        let entry = unsafe { ash::Entry::load()? };

        let instance = vulkan::instance::生成する(&entry, 表示ハンドル, デバッグ有効か)?;
        let 検証カウンタ = 検証カウンタ::生成する();
        let デバッグメッセンジャー =
            デバッグメッセンジャーを作る(&entry, &instance, &検証カウンタ, デバッグ有効か)?;

        let (surface_loader, surface) =
            vulkan::surface::生成する(&entry, &instance, 表示ハンドル, ウィンドウハンドル)?;
        let (physical_device, queue_family_index) =
            vulkan::physical_device::選定する(&instance, &surface_loader, surface)?;
        let (device, queue) = vulkan::device::生成する(&instance, physical_device, queue_family_index)?;
        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        let swapchain = vulkan::swapchain::スワップチェーン::生成する(
            physical_device,
            &device,
            &surface_loader,
            surface,
            &swapchain_loader,
            寸法,
            vk::SwapchainKHR::null(),
        )?;

        let 資源 = generate_resources::組み立てる(
            &instance,
            physical_device,
            &device,
            queue_family_index,
            &swapchain,
            &シェーダー,
            頂点一覧,
            インデックス一覧,
        )?;

        Ok(Self {
            entry,
            instance,
            デバッグメッセンジャー,
            surface_loader,
            surface,
            physical_device,
            device,
            queue,
            swapchain_loader,
            swapchain,
            深度バッファ: 資源.深度バッファ,
            ジオメトリ: 資源.ジオメトリ,
            command_pool: 資源.command_pool,
            command_buffer一覧: 資源.command_buffer一覧,
            フレーム同期: 資源.フレーム同期,
            提示同期: 資源.提示同期,
            現在フレーム添字: 0,
            pipeline: 資源.pipeline,
            読み戻しバッファ: None,
            検証カウンタ,
            現在の寸法: 寸法,
            再構築が必要: false,
        })
    }
}
