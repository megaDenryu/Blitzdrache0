//! 描画に使ってよいと観測できた提示資源への通行証と、その通行証で読み出した画像1枚ぶんのハンドル束。
//! 提示資源のハンドルを読む唯一の入口をここに置くため、状態の判別を経ずにスワップチェーン・深度画像・
//! 提示セマフォへ触るコードは書けない。

use ash::vk;

use crate::renderer::present_resources::{提示画像組, 提示資源};
use crate::vulkan::swapchain::スワップチェーン画像添字;

/// 提示状態が`描画可能`のときだけ得られる提示資源への借用。
///
/// 注意: 通行証を持つことは「取得した時点で描画可能と観測できた」以上を意味しない。スワップチェーンが
/// 実際に有効であることは保証せず、判別と`vkAcquireNextImageKHR`のあいだに陳腐化しうる。
/// 実際の有効性は取得と提示の戻り値だけが答える。
///
/// 借用であるため、`&mut レンダラー`を要する工程を挟むと持ち回れない。工程をまたいで運ぶ値は`取得済み提示`にする。
pub(in crate::renderer) struct 描画許可<'資源> {
    資源: &'資源 提示資源,
}

impl<'資源> 描画許可<'資源> {
    pub(super) fn 束ねる(資源: &'資源 提示資源) -> Self {
        Self { 資源 }
    }

    /// 画像取得・実表示待機に渡すスワップチェーンのハンドル。
    pub(in crate::renderer) fn スワップチェーンハンドル(&self) -> vk::SwapchainKHR {
        self.資源.スワップチェーンハンドル()
    }

    /// 取得できた画像添字に対応するハンドルを、通行証を持っているこの場でまとめて読み出す。
    pub(in crate::renderer) fn 取得した画像を束ねる(&self, 画像添字: スワップチェーン画像添字) -> 取得済み提示 {
        取得済み提示 {
            画像添字,
            swapchain: self.資源.スワップチェーンハンドル(),
            提示セマフォ: self.資源.提示セマフォ(画像添字),
            画像組: self.資源.画像組を引く(画像添字),
        }
    }
}

/// 取得できた画像1枚ぶんの提示側ハンドル。`描画許可`のもとで読み出した値だけを持つため、
/// `&mut レンダラー`を要する送信・提示の工程へそのまま持ち込める。
pub(in crate::renderer) struct 取得済み提示 {
    画像添字: スワップチェーン画像添字,
    swapchain: vk::SwapchainKHR,
    提示セマフォ: vk::Semaphore,
    画像組: 提示画像組,
}

impl 取得済み提示 {
    pub(in crate::renderer) fn 画像添字(&self) -> スワップチェーン画像添字 {
        self.画像添字
    }

    pub(in crate::renderer) fn スワップチェーン(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    pub(in crate::renderer) fn 提示セマフォ(&self) -> vk::Semaphore {
        self.提示セマフォ
    }

    pub(in crate::renderer) fn 画像組(&self) -> &提示画像組 {
        &self.画像組
    }
}
