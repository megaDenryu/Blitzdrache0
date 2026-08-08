//! 転送コマンドを積んでいる最中の一時コマンドバッファ。担当する局面は「1本確保して積み込みを開始してから、
//! 送信してfence待ちで完了させるまで」であり、この型はその間だけ存在する。
//!
//! 括りをクロージャで挟まずこの型で表すのは、何を積むかを書く側が制御を持ったまま直列の文で書けるようにするためである。

use ash::vk;

use super::転送実行環境;
use crate::error::レンダラーエラー;

/// 注意: `送信して完了を待つ`を呼ばずに捨てると、コマンドバッファが解放されず勘定も戻らない。
/// 残りは転送実行環境の`破棄する`が見つけてpanicする。
#[must_use]
pub(crate) struct 転送コマンドを積む一時コマンドバッファ<'環境> {
    環境: &'環境 転送実行環境,
    command_buffer: vk::CommandBuffer,
}

impl<'環境> 転送コマンドを積む一時コマンドバッファ<'環境> {
    pub(super) fn 積み始める(環境: &'環境 転送実行環境) -> Result<Self, レンダラーエラー> {
        let command_buffer = 一時コマンドバッファを1本確保する(環境)?;
        let 開始情報 = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // 安全性: command_bufferは直前に確保したもので、まだ積み込みを始めていない。
        if let Err(誤り) = unsafe { 環境.device.begin_command_buffer(command_buffer, &開始情報) } {
            // 安全性: command_bufferはこのスコープの唯一の所有者で、積み込みを始めていないため以降使用しない。
            unsafe { 環境.device.free_command_buffers(環境.command_pool, &[command_buffer]) };
            return Err(誤り.into());
        }
        環境.未送信の一時コマンドバッファ数.積み始めた1本を加える();
        Ok(Self { 環境, command_buffer })
    }

    /// 返る参照が借りるのは転送実行環境であり、この値ではない。積んだ後にこの値を送信で消費できる。
    pub(crate) fn 論理デバイス(&self) -> &'環境 ash::Device {
        &self.環境.device
    }

    pub(crate) fn 積む先のコマンドバッファ(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    /// 積み込みを閉じてグラフィックスキューへ送信し、fence待ちで完了を保証する。
    pub(crate) fn 送信して完了を待つ(self) -> Result<(), レンダラーエラー> {
        let 送信結果 = self.積み込みを閉じて送信し待つ();
        // 安全性: command_bufferはこの値が唯一の所有者で、fence待ちの後か送信前の失敗のいずれでも以降使用しない。
        unsafe { self.環境.device.free_command_buffers(self.環境.command_pool, &[self.command_buffer]) };
        self.環境.未送信の一時コマンドバッファ数.送信し終えた1本を差し引く();
        送信結果
    }

    fn 積み込みを閉じて送信し待つ(&self) -> Result<(), レンダラーエラー> {
        let device = &self.環境.device;
        // 安全性: command_bufferは積み込み開始済みで、これが対応するend呼び出しである。
        unsafe { device.end_command_buffer(self.command_buffer)? };
        // 安全性: deviceは生成済みで有効。
        let fence = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None)? };
        let command_buffer一覧 = [self.command_buffer];
        let 送信情報 = vk::SubmitInfo::default().command_buffers(&command_buffer一覧);
        // 安全性: command_bufferは積み終えており、fenceは直前に生成した非シグナル状態である。
        let 送信結果 = unsafe { device.queue_submit(self.環境.queue, &[送信情報], fence) };
        let 待機結果 = 送信結果.and_then(|()| {
            // 安全性: fenceはこの送信の完了を示す唯一の待機対象である。
            unsafe { device.wait_for_fences(&[fence], true, u64::MAX) }
        });
        // 安全性: fenceはこのスコープの唯一の所有者で、待機の成否によらず以降使用しない。
        unsafe { device.destroy_fence(fence, None) };
        待機結果.map_err(Into::into)
    }
}

fn 一時コマンドバッファを1本確保する(環境: &転送実行環境) -> Result<vk::CommandBuffer, レンダラーエラー> {
    let 割当情報 = vk::CommandBufferAllocateInfo::default()
        .command_pool(環境.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    // 安全性: command_poolは生成済みで有効。
    let 一覧 = unsafe { 環境.device.allocate_command_buffers(&割当情報)? };
    let Some(&command_buffer) = 一覧.first() else {
        // command_buffer_count(1)を要求してVulkanが成功を返したのに0本なのは、Vulkan実装が契約を破っている状態であり回復不能。
        panic!("allocate_command_buffersが1本のコマンドバッファを返さなかった");
    };
    Ok(command_buffer)
}
