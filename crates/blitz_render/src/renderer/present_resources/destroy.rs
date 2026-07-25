//! 破棄局面: 終了時に1度と、寸法変更のたびに1度呼ぶ3資源の破棄。
//! 3つはどれも論理デバイス水準のオブジェクトで、提示セマフォ・深度画像・スワップチェーン画像ビューのあいだに
//! Vulkanの破棄順序の制約は無いため、この3行の順序に意味は無い。意味があるのは3つが1回の呼び出しで揃って落ちること、
//! すなわち片方だけが残った束を作れないことである。
//!
//! 前提: GPU上の全作業の完了待ちは呼び出し元が済ませる。
//! 前提: 深度バッファは専用メモリを持つため、呼び出し元はこの破棄を`device.全メモリ解放を確認する()`より前に呼ぶ。
//! 参照: crates/blitz_render/src/renderer/destroy.rs がレンダラー全体の破棄順を持つ。

use super::提示資源;
use crate::vulkan::gpu_environment::GPU環境;

impl 提示資源 {
    pub(in crate::renderer) fn 破棄する(&self, 環境: &GPU環境) {
        let device = 環境.device();
        self.提示同期.破棄する(device);
        self.深度バッファ.破棄する(device);
        self.swapchain.破棄する(device, 環境.swapchain_loader());
    }
}
