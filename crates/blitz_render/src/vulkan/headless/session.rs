//! ウィンドウなし実行GPU環境でGPU命令を積んでいる最中の一時コマンドバッファ。担当する局面は
//! 「1本確保して積み込みを開始してから、送信してフェンスで完了を待つまで」であり、この型はその間だけ存在する。
//!
//! 検査だけがこの経路を使う。進行中フレームも提示も持たないため、送信ごとに待つことが許される。

use ash::vk;

use super::ウィンドウなし実行GPU環境;
use crate::error::レンダラーエラー;

/// 注意: `送信して完了を待つ`を呼ばずに捨てると、コマンドバッファが解放されず勘定も戻らない。
/// 残りはウィンドウなし実行GPU環境の`破棄する`が見つけてpanicする。
#[must_use]
pub(crate) struct GPU命令を積む一時コマンドバッファ<'環境> {
    環境: &'環境 ウィンドウなし実行GPU環境,
    command_buffer: vk::CommandBuffer,
}

impl<'環境> GPU命令を積む一時コマンドバッファ<'環境> {
    pub(super) fn 積み始める(環境: &'環境 ウィンドウなし実行GPU環境) -> Result<Self, レンダラーエラー> {
        let command_buffer = 一時コマンドバッファを1本確保する(環境)?;
        let 開始情報 = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // 安全性: command_bufferは直前に確保したもので、まだ積み込みを始めていない。
        if let Err(誤り) = unsafe { 環境.device.begin_command_buffer(command_buffer, &開始情報) } {
            // 安全性: command_bufferはこのスコープの唯一の所有者で、積み込みを始めていないため以降使用しない。
            unsafe { 環境.device.free_command_buffers(環境.command_pool, &[command_buffer]) };
            return Err(誤り.into());
        }
        環境.未送信のコマンドバッファ数.積み始めた1本を加える();
        Ok(Self { 環境, command_buffer })
    }

    /// 積み先を組む材料。`command_sink`が積み先を作るためだけに開ける口であり、返る参照はこの値を借りる。
    /// この値を送信で消費した後の積み先が残らないよう、`'環境`でなくこの値の借用へ縛る。
    pub(in crate::vulkan) fn 論理デバイス(&self) -> &ash::Device {
        &self.環境.device
    }

    pub(in crate::vulkan) fn 積む先のコマンドバッファ(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    /// 積み込みを閉じてコンピュートキューへ送信し、フェンスで完了を待つ。戻った時点でGPUの作業は終わっている。
    pub(crate) fn 送信して完了を待つ(self) -> Result<(), レンダラーエラー> {
        let 送信結果 = self.積み込みを閉じて送信し待つ();
        // 安全性: command_bufferはこの値が唯一の所有者で、完了待ちの後か送信前の失敗のいずれでも以降使用しない。
        unsafe { self.環境.device.free_command_buffers(self.環境.command_pool, &[self.command_buffer]) };
        self.環境.未送信のコマンドバッファ数.送信し終えた1本を差し引く();
        送信結果
    }

    fn 積み込みを閉じて送信し待つ(&self) -> Result<(), レンダラーエラー> {
        let device = &self.環境.device;
        // 安全性: command_bufferは積み込み開始済みで、これが対応するend呼び出しである。
        unsafe { device.end_command_buffer(self.command_buffer)? };
        // 安全性: deviceは生成済みで有効。
        let フェンス = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None)? };
        let 結果 = self.フェンスを添えて送信し待つ(フェンス);
        // 安全性: フェンスはこの関数が唯一の所有者であり、待機の完了後に破棄する。
        unsafe { device.destroy_fence(フェンス, None) };
        結果
    }

    fn フェンスを添えて送信し待つ(&self, フェンス: vk::Fence) -> Result<(), レンダラーエラー> {
        let device = &self.環境.device;
        let コマンドバッファ情報 = [vk::CommandBufferSubmitInfo::default().command_buffer(self.command_buffer)];
        let 送信情報 = [vk::SubmitInfo2::default().command_buffer_infos(&コマンドバッファ情報)];
        // 安全性: command_bufferは積み終えており、キューとフェンスは生成済みで有効である。
        unsafe { device.queue_submit2(self.環境.queue, &送信情報, フェンス)? };
        // 安全性: フェンスは直前の送信に結び付けた唯一のものである。
        unsafe { device.wait_for_fences(&[フェンス], true, u64::MAX)? };
        Ok(())
    }
}

fn 一時コマンドバッファを1本確保する(
    環境: &ウィンドウなし実行GPU環境
) -> Result<vk::CommandBuffer, レンダラーエラー> {
    let 割当情報 = vk::CommandBufferAllocateInfo::default()
        .command_pool(環境.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    // 安全性: command_poolは生成済みで有効。
    let 一覧 = unsafe { 環境.device.allocate_command_buffers(&割当情報)? };
    let Some(&command_buffer) = 一覧.first() else {
        panic!("allocate_command_buffersが成功したのにコマンドバッファが0本だった(Vulkan実装の契約違反)");
    };
    Ok(command_buffer)
}
