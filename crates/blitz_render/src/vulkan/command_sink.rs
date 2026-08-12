//! GPU命令を積む先。記録中のコマンドバッファと、そのコマンドバッファを配った論理デバイスの組である。
//!
//! 2つを組で持つのは、命令を積む工程へ論理デバイスとコマンドバッファを別々に運ばせると、
//! 別の局面のコマンドバッファと組み合わせた呼び出しが署名の上で成立してしまうためである。
//! 組を作れるのはコマンドバッファの積み込みを開始したセッション型だけであり、工程の側は組を受け取って積むだけになる。
//!
//! 参照: _doc/計画/ユビキタス言語.md「セッション型」

use ash::vk;

#[derive(Clone, Copy)]
pub(crate) struct GPU命令の積み先<'デバイス> {
    device: &'デバイス ash::Device,
    command_buffer: vk::CommandBuffer,
}

impl<'デバイス> GPU命令の積み先<'デバイス> {
    /// 前提: command_bufferは呼び出し元が積み込みを開始したものであり、deviceはそれを配った論理デバイスである。
    pub(crate) fn 生成する(device: &'デバイス ash::Device, command_buffer: vk::CommandBuffer) -> Self {
        Self { device, command_buffer }
    }

    pub(crate) fn 論理デバイス(&self) -> &'デバイス ash::Device {
        self.device
    }

    pub(crate) fn コマンドバッファ(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
}
