//! ウィンドウ寸法に連動して揃って作り直す提示側の資源の束: スワップチェーン・深度バッファ・提示同期。
//! 深度画像はスワップチェーンが実際に決めた寸法で確保し、提示セマフォはスワップチェーン画像1枚につき1本用意するため、
//! 3つの寸法と本数はスワップチェーンが決めた1つの値に従う。3つを別々に持つと寸法変更で一部だけを作り直せてしまい、
//! 深度画像がスワップチェーン画像と違う寸法になる状態と、提示セマフォ数が画像数とずれる状態を作れる。
//! その不整合を作れなくするため、3つの生成と破棄をこの型に閉じる。
//! 生成と作り直しは`create`、破棄は`destroy`にある。
//!
//! 注意: サーフェスと論理デバイスはこの束に入れない。それらはvkDestroyDeviceを挟む破棄順序の制約を持つ
//! `vulkan::gpu_environment::GPU環境`の担当であり、混ぜるとこの束の破棄の途中にvkDestroyDeviceが挟まる。

mod create;
mod destroy;

use ash::vk;

use crate::vulkan;
use crate::vulkan::swapchain::スワップチェーン画像添字;

pub(super) struct 提示資源 {
    swapchain: vulkan::swapchain::スワップチェーン,
    深度バッファ: vulkan::depth::深度バッファ,
    提示同期: vulkan::sync::提示同期,
}

/// 1つの画像添字で参照した提示先の画像とビューに、全画像が共有する深度画像とビューを添えた組。
/// 画像とビューを同時に取り出すため、別添字の画像とビューが混ざった組み合わせを作れない。
pub(super) struct 提示画像組 {
    pub(super) スワップチェーン画像: vk::Image,
    pub(super) スワップチェーンビュー: vk::ImageView,
    pub(super) 深度画像: vk::Image,
    pub(super) 深度ビュー: vk::ImageView,
}

impl 提示資源 {
    /// 画像取得・提示・実表示待機に渡すスワップチェーンのハンドル。
    pub(super) fn スワップチェーンハンドル(&self) -> vk::SwapchainKHR {
        self.swapchain.handle
    }

    /// スワップチェーンが実際に決めた寸法。深度画像とポスト処理の中間画像はこの寸法に従う。
    pub(super) fn 寸法(&self) -> vk::Extent2D {
        self.swapchain.寸法
    }

    /// スワップチェーン画像の色形式。シーンの描画先形式とパイプラインの色アタッチメント形式を決める。
    pub(super) fn 画像形式(&self) -> vk::Format {
        self.swapchain.画像形式
    }

    /// 提示済み画像をホスト可視バッファへ転送する使い方をスワップチェーンが許すか。
    pub(super) fn 読み戻し対応(&self) -> bool {
        self.swapchain.読み戻し対応
    }

    pub(super) fn 画像組を参照する(&self, 画像添字: スワップチェーン画像添字) -> 提示画像組 {
        let 添字 = 画像添字.配列添字();
        提示画像組 {
            スワップチェーン画像: self.swapchain.画像一覧[添字],
            スワップチェーンビュー: self.swapchain.画像ビュー一覧[添字],
            深度画像: self.深度バッファ.画像,
            深度ビュー: self.深度バッファ.画像ビュー,
        }
    }

    /// 画像添字に対応する提示セマフォ。本数は画像数と一致するため、取得できた添字はそのまま使える。
    pub(super) fn 提示セマフォ(&self, 画像添字: スワップチェーン画像添字) -> vk::Semaphore {
        self.提示同期.提示セマフォ(画像添字)
    }
}
