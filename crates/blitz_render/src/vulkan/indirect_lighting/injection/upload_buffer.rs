//! 注入する中身を載せたホスト可視バッファの所有者。担うのは確保・書き込み・転送元としての貸し出し・破棄である。
//! 中身の意味は知らず、渡されたバイト列をそのまま持つ。
//!
//! 生成は注入の入口が呼ばれた1回だけであり、以降のフレームは同じバッファを転送元に取る。
//! 毎フレーム確保し直さないのは、注入する値が起動時に決まって以降変わらないためである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan::indirect_lighting) struct 注入元バッファ {
    pub(in crate::vulkan::indirect_lighting) handle: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 注入元バッファ {
    pub(in crate::vulkan::indirect_lighting) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        バイト列: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let 長さ = u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("注入するバイト数がu64に収まらない"));
        let create_info = vk::BufferCreateInfo::default()
            .size(長さ)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // 安全性: deviceは生成済みで有効。
        let handle = unsafe { device.create_buffer(&create_info, None)? };
        match メモリを結びつけて書く(device, メモリプロパティ, handle, バイト列) {
            Ok(memory) => Ok(Self { handle, memory }),
            Err(誤り) => {
                // 安全性: handleはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(handle, None) };
                Err(誤り)
            }
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、このバッファは遠方環境の照明資源の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan::indirect_lighting) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: handle・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe { device.destroy_buffer(self.handle, None) };
        device.メモリを解放する(self.memory);
    }
}

fn メモリを結びつけて書く(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    handle: vk::Buffer,
    バイト列: &[u8],
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: handleは直前に生成済み。
    let 要件 = unsafe { device.get_buffer_memory_requirements(handle) };
    let メモリ型添字 = memory::ホスト可視メモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let memory = memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::読み戻しバッファ)?;
    // 安全性: handle・memoryはともに生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_buffer_memory(handle, memory, 0) } {
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    if let Err(誤り) = 書き込む(device, memory, バイト列) {
        device.メモリを解放する(memory);
        return Err(誤り);
    }
    Ok(memory)
}

fn 書き込む(device: &ash::Device, memory: vk::DeviceMemory, バイト列: &[u8]) -> Result<(), レンダラーエラー> {
    // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済みであり、マッピングは確保容量全体を対象にする。
    let ポインタ = unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
    // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、長さは確保時に要求した容量と同じである。
    unsafe { std::ptr::copy_nonoverlapping(バイト列.as_ptr(), ポインタ.cast::<u8>(), バイト列.len()) };
    // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
    unsafe { device.unmap_memory(memory) };
    Ok(())
}
