//! フレーム1回ぶんのGPU命令を積んでいる最中のコマンドバッファ。担当する局面は「スロットのコマンドバッファへ積み込みを
//! 開始してから、記録を閉じて送信し提示するまで」であり、この型はその間だけ存在する。
//! 積む局面は`record`が、閉じて送信し提示する局面は`submit_present`が、同じ型のメソッドとして持つ。
//!
//! 括りをクロージャで挟まずこの型で表すのは、何を積むかを書く側が制御を持ったまま直列の文で書けるようにするためである。

use ash::vk;

use super::environment::フレームの記録の実行環境;
use crate::error::レンダラーエラー;
use crate::vulkan::gpu_timing;

/// 注意: `記録を閉じて送信し提示する`を呼ばずに捨てると勘定が戻らない。
/// 残りはレンダラーの破棄が`未送信が1本も残っていないことを確かめる`で見つけてpanicする。
/// `クエリプール`は積み始めでリセットし、パス前後の書き込みはグラフの実行が行う。
#[must_use]
pub(crate) struct フレームのGPU命令を積むコマンドバッファ<'環境> {
    環境: &'環境 フレームの記録の実行環境,
    command_buffer: vk::CommandBuffer,
    クエリプール: Option<vk::QueryPool>, // パス別GPU計測が有効なフレームだけ`Some`
}

impl<'環境> フレームのGPU命令を積むコマンドバッファ<'環境> {
    pub(super) fn 積み始める(
        環境: &'環境 フレームの記録の実行環境,
        command_buffer: vk::CommandBuffer,
        クエリプール: Option<vk::QueryPool>,
    ) -> Result<Self, レンダラーエラー> {
        let begin_info = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // 安全性: command_bufferはRESET_COMMAND_BUFFERフラグ付きプール由来で、
        // ここでの開始が暗黙的に前回の記録をリセットする。
        unsafe { 環境.論理デバイス().begin_command_buffer(command_buffer, &begin_info)? };
        if let Some(pool) = クエリプール {
            // 安全性: command_bufferは記録開始済みで、poolはこのスロット専用に生成済み。
            // このフレームで書くクエリより前に必ずリセットする(未リセットのクエリへの
            // 書き込みはVulkanの契約違反になる)。
            unsafe {
                環境
                    .論理デバイス()
                    .cmd_reset_query_pool(command_buffer, pool, 0, gpu_timing::パス数上限 * 2)
            };
        }
        環境.積み始めた1本を数える();
        Ok(Self {
            環境,
            command_buffer,
            クエリプール,
        })
    }

    /// 積み先を組む材料。`command_sink`が積み先を作るためだけに開ける口であり、返る参照はこの値を借りる。
    /// 記録を閉じて送信した後の積み先が残らないよう、`'環境`でなくこの値の借用へ縛る。
    pub(in crate::vulkan) fn 論理デバイス(&self) -> &ash::Device {
        self.環境.論理デバイス()
    }

    pub(in crate::vulkan) fn 積む先のコマンドバッファ(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    pub(super) fn 環境(&self) -> &'環境 フレームの記録の実行環境 {
        self.環境
    }

    pub(super) fn 計測のクエリプール(&self) -> Option<vk::QueryPool> {
        self.クエリプール
    }
}
