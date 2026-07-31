//! フレームスロットに連動する資源と、そのスロットの巡回状態の束。
//! コマンドバッファ・描画完了フェンス・取得セマフォの3つはすべて同じスロット番号で参照され、スロットが進むと参照先が揃って切り替わる。
//! 3つを別々に参照する手段を外へ出すと別スロットの資源が混ざるため、取り出しは`現在のスロット資源`の1つに限り、スロット番号を書き換える手段も外へ出さない。
//! 生成は`create`、スロットを進めるかどうかの規則は`slot_cycle`にある。

mod create;
mod slot_cycle;

use ash::vk;

use crate::vulkan;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::tracked_device::GPUデバイス;
use slot_cycle::フレームスロット巡回;

pub(super) use slot_cycle::フレーム結末;

pub(super) struct フレーム進行 {
    command_pool: vk::CommandPool,
    command_buffer一覧: [vk::CommandBuffer; 進行中フレーム数],
    フレーム同期: vulkan::sync::フレーム同期,
    巡回: フレームスロット巡回,
    /// 提示へ到達しなかったフレームの累計。提示停止に起因する異常(破棄待ちの滞留・フレームループの空転)を実行中に観測する計器である。
    見送り累計: u64,
}

/// 現在のスロットで参照した資源一式。3つを同時に取り出すため、別スロットの資源が混ざった組み合わせを作れない。
pub(super) struct フレームスロット資源 {
    pub(super) スロット: フレームスロット添字,
    pub(super) フェンス: vk::Fence,
    pub(super) 取得セマフォ: vk::Semaphore,
    pub(super) command_buffer: vk::CommandBuffer,
}

impl フレーム進行 {
    pub(super) fn 現在のスロット資源(&self) -> フレームスロット資源 {
        let スロット = self.巡回.現在のスロット();
        フレームスロット資源 {
            スロット,
            フェンス: self.フレーム同期.フェンス(スロット),
            取得セマフォ: self.フレーム同期.取得セマフォ(スロット),
            command_buffer: self.command_buffer一覧[スロット.配列添字()],
        }
    }

    /// 提示まで到達したフレームだけがスロットを進める。見送りと失敗では据え置き、次フレームが同じスロットの資源を使う。
    pub(super) fn 結末を反映する(&mut self, 結末: フレーム結末) {
        if let フレーム結末::見送った = 結末 {
            self.見送り累計 = self.見送り累計.saturating_add(1);
        }
        self.巡回.結末を反映する(結末);
    }

    pub(super) fn 見送りフレーム数(&self) -> u64 {
        self.見送り累計
    }

    /// 注意: コマンドバッファはコマンドプールの破棄で暗黙に解放されるため、個別のfree_command_buffersは不要。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.フレーム同期.破棄する(device);
        // 安全性: command_poolはSelfが唯一の所有者で、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_command_pool(self.command_pool, None) };
    }
}
