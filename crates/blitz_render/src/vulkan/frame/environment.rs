//! フレームの記録の実行環境: フレームスロットのコマンドバッファ1本へ、そのフレームぶんの積み込みを開始する操作サービス。
//!
//! 論理デバイスを操作のたびに引数で受け取らず複製して保持するのは、記録を頼む側にデバイスとキューを運ばせないためである。
//! 依存を保持しないと、記録を頼む側から見える署名にまでデバイスとキューが漏れる。
//!
//! 参照: _doc/計画/ユビキタス言語.md「操作サービス」「セッション型」

use ash::vk;

use super::session::フレームのGPU命令を積むコマンドバッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::unsent_command_buffers::未送信のコマンドバッファ数;

pub(crate) struct フレームの記録の実行環境 {
    device: ash::Device,
    queue: vk::Queue,
    未送信のコマンドバッファ数: 未送信のコマンドバッファ数,
}

impl フレームの記録の実行環境 {
    pub(crate) fn 生成する(device: &ash::Device, queue: vk::Queue) -> Self {
        Self {
            device: device.clone(),
            queue,
            未送信のコマンドバッファ数: 未送信のコマンドバッファ数::零から数え始める(),
        }
    }

    /// そのフレームのスロットのコマンドバッファへ積み込みを開始する。計測が有効なフレームは同時にクエリプールをリセットする。
    /// 返る値は`記録を閉じて送信し提示する`で必ず閉じる。閉じないまま捨てると`未送信が1本も残っていないことを確かめる`が止める。
    pub(crate) fn フレームのgpu命令を積み始める(
        &self,
        command_buffer: vk::CommandBuffer,
        クエリプール: Option<vk::QueryPool>,
    ) -> Result<フレームのGPU命令を積むコマンドバッファ<'_>, レンダラーエラー> {
        フレームのGPU命令を積むコマンドバッファ::積み始める(self, command_buffer, クエリプール)
    }

    /// 前提: 呼び出し元はGPUの全作業の完了を待っている。レンダラーの破棄の1段として呼ぶ。
    pub(crate) fn 未送信が1本も残っていないことを確かめる(&self) {
        self.未送信のコマンドバッファ数.未送信が1本も残っていないことを確かめる();
    }

    pub(super) fn 論理デバイス(&self) -> &ash::Device {
        &self.device
    }

    pub(super) fn 送信先のキュー(&self) -> vk::Queue {
        self.queue
    }

    pub(super) fn 積み始めた1本を数える(&self) {
        self.未送信のコマンドバッファ数.積み始めた1本を加える();
    }

    pub(super) fn 送信し終えた1本を数える(&self) {
        self.未送信のコマンドバッファ数.送信し終えた1本を差し引く();
    }
}
