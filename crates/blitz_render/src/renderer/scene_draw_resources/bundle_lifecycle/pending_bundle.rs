//! 解除予約された束と、破棄までに待つ残りフレーム数を1つの値として所有する。
//! 触れる状態はこの2つだけであり、残りを減らす操作と破棄してよいかの判定をここに閉じる。
//!
//! 束と残りを別々に持ち回らないのは、残りが0でない束を破棄する経路と、残りだけが進んで束が置き去りになる経路を作らないためである。

use super::super::chunk_draw_resources::チャンク描画資源;
use crate::draw_bundle_id::描画束ID;
use crate::vulkan::sync::進行中フレーム数;

pub(in crate::renderer) struct 破棄待ち束 {
    束: チャンク描画資源,
    残りフレーム: usize,
}

impl 破棄待ち束 {
    /// 進行中フレーム数ぶんの待ちを付けて予約する。この回数を待てば、その束を使う発行済みの描画は必ず完了している。
    pub(super) fn 解除予約から作る(束: チャンク描画資源) -> Self {
        Self {
            束,
            残りフレーム: 進行中フレーム数,
        }
    }

    pub(in crate::renderer) fn 束(&self) -> &チャンク描画資源 {
        &self.束
    }

    /// 待ちを終えた束をGPUから解放し、会計へ載せる束IDを返す。
    /// 前提: 呼び出し元がGPU側の使用完了を保証している。
    pub(super) fn 破棄してその束の識別子を返す(self, device: &crate::vulkan::tracked_device::GPUデバイス) -> 描画束ID {
        self.束.破棄する(device);
        self.束.id()
    }

    /// 残りを1減らし、破棄してよくなったかを返す。
    pub(super) fn 一フレーム進める(&mut self) -> bool {
        self.残りフレーム = self.残りフレーム.saturating_sub(1);
        self.残りフレーム == 0
    }
}
