//! 生成済みの画像へ専用メモリを結び付ける工程。画像そのものの生成条件(形式・寸法・タイリング・用途)は
//! それぞれの画像を所有する型が決めるため、この工程が受け取るのは画像ハンドルとメモリの用途だけである。

use ash::vk;

use super::GPU資源の確保係;
use super::memory_type::メモリの置き場;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;

impl GPU資源の確保係<'_> {
    /// 前提: 画像は生成済みで、まだどのメモリにも結び付いていない。
    /// 失敗したときは確保したメモリをこの中で解放するため、呼び出し側は画像だけを片付ければよい。
    pub(crate) fn 画像へデバイスローカルメモリを結び付ける(
        &self,
        画像: vk::Image,
        メモリ用途: GPUメモリ用途,
    ) -> Result<vk::DeviceMemory, レンダラーエラー> {
        // 安全性: 画像は呼び出し元が直前に生成済み。
        let 要件 = unsafe { self.device.get_image_memory_requirements(画像) };
        let memory = self.要件に合う専用メモリを確保する(要件, メモリの置き場::デバイスローカル, メモリ用途)?;
        // 安全性: 画像とmemoryはともに生成済みで、offsetは0(専用確保のため衝突しない)。
        if let Err(誤り) = unsafe { self.device.bind_image_memory(画像, memory, 0) } {
            self.device.メモリを解放する(memory);
            return Err(誤り.into());
        }
        Ok(memory)
    }
}
