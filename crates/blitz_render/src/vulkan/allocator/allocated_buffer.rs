//! バッファと、そのバッファのためだけに確保した専用メモリの組。担当するのは、この2つを常に組で生き死にさせ、
//! 自分のメモリの読み書きを引き受けることである。
//!
//! 資源型の様式に従い論理デバイスを保持せず、破棄でも読み書きでも引数として受け取る。読み書きも同じ形にするのは、
//! それが毎フレームの描画や検収の途中で起こり、そこに立っているのは確保係ではなく論理デバイスだけだからである。
//! メモリのハンドルを外へ出さないのは、写像と解放という対の操作をこの型の中だけに閉じるためである。
//! 参照: _doc/計画/ユビキタス言語.md「資源型」

mod host_read;
mod host_write;

use ash::vk;

use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct 専用メモリ付きバッファ {
    buffer: vk::Buffer,
    pub(super) memory: vk::DeviceMemory, // 注意: このハンドルを確保係の外へ出さない
    確保バイト数: u64,                   // メモリ要件が求めた確保量。要求値以上になりうるため別に保つ
}

impl 専用メモリ付きバッファ {
    pub(super) fn 組にする(buffer: vk::Buffer, memory: vk::DeviceMemory, 確保バイト数: u64) -> Self {
        Self {
            buffer,
            memory,
            確保バイト数,
        }
    }

    pub(crate) fn バッファのハンドル(&self) -> vk::Buffer {
        self.buffer
    }

    pub(crate) fn 確保バイト数(&self) -> u64 {
        self.確保バイト数
    }

    /// 写像して読み書きする長さが確保量に収まることを確かめる。破れたまま`from_raw_parts`へ渡すと確保の外を触るため、
    /// 呼び出し元の「前提」で済ませずここで止める。到達したら呼び出し元の長さの計算がバグである。
    pub(super) fn 写像で触る長さが確保量に収まることを確かめる(&self, 長さ: usize) {
        let 長さのバイト数 = u64::try_from(長さ).unwrap_or_else(|_| panic!("写像で触る長さがu64に収まらない: {長さ}"));
        if 長さのバイト数 > self.確保バイト数 {
            panic!("写像で触る長さ{長さのバイト数}バイトが確保量{}バイトを超えている", self.確保バイト数);
        }
    }

    /// 巻き戻しの台帳が持つ控えを返す。控えと元は同じGPU資源を指すため、破棄するのはどちらか一方だけである。
    pub(super) fn 控えを返す(&self) -> Self {
        Self {
            buffer: self.buffer,
            memory: self.memory,
            確保バイト数: self.確保バイト数,
        }
    }

    /// 前提: 呼び出し元はGPU側の使用完了を保証している。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}
