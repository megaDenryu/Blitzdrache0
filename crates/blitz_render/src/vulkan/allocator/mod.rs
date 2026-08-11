//! GPU資源の確保を1つの操作サービスへ集めた階層。バッファ・専用メモリ・サンプラー・シェーダーモジュール・
//! コマンドプールの確保は、どれも論理デバイスと物理デバイスのメモリ性質だけを材料にするため、材料を保持する
//! 型を1つ立ててメソッドで公開する。
//!
//! 論理デバイスとメモリ性質を確保のたびに引数で受け取らないのは、確保を頼む側の署名にGPUの依存を運ばせないためである。
//! 借用で保持するのは、論理デバイスの所有者が`GPU環境`ただ1つであり、確保係はその環境が生きている構築の局面だけ立てる道具だからである。
//!
//! 参照: _doc/計画/ユビキタス言語.md「操作サービス」「資源型」

mod allocated_buffer;
mod buffer;
mod dedicated_memory;
mod frame_commands;
mod image_memory;
mod memory_type;
mod per_frame_buffers;
mod rollback_ledger;
mod sampler;
mod shader_module;

use ash::vk;

use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use allocated_buffer::専用メモリ付きバッファ;
pub(crate) use frame_commands::フレーム記録のコマンド一式;
pub(crate) use per_frame_buffers::フレームスロットごとのバッファ;
pub(crate) use rollback_ledger::巻き戻せる確保の台帳;

pub(crate) struct GPU資源の確保係<'デバイス> {
    device: &'デバイス GPUデバイス,
    メモリプロパティ: vk::PhysicalDeviceMemoryProperties,
}

impl<'デバイス> GPU資源の確保係<'デバイス> {
    pub(crate) fn 生成する(device: &'デバイス GPUデバイス, メモリプロパティ: vk::PhysicalDeviceMemoryProperties) -> Self {
        Self {
            device, メモリプロパティ
        }
    }

    /// 確保以外のGPU操作(画像の生成・ディスクリプタの更新・破棄)に使う論理デバイスを貸す。
    /// 返る参照が借りるのは元の`GPU環境`であり、この確保係ではない。
    pub(crate) fn 論理デバイス(&self) -> &'デバイス GPUデバイス {
        self.device
    }
}
