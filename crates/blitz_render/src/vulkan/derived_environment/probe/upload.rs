//! 検査が与える遠方環境の中身を載せたホスト可視バッファの所有者。担うのは、確保・中身の書き込み・転送元としての
//! 貸し出し・破棄である。バイト並びは`environment_bytes`、転送パスの宣言は`copy_pass`が持つ。
//!
//! 大気から焼くのでなくホストから与えるのは、畳み込みの正しさを解析解と突き合わせるためである。
//! 定数の環境なら拡散照度は円周率×放射輝度、鏡面畳込みはどの粗さ段も放射輝度そのものになる。

use ash::vk;

use crate::distant_environment::derived::遠方環境の内容;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 遠方環境の書き込みバッファ {
    pub(super) handle: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 遠方環境の書き込みバッファ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        内容: &遠方環境の内容,
    ) -> Result<Self, レンダラーエラー> {
        let バイト列 = super::environment_bytes::半精度のバイト列へ詰める(内容);
        let create_info = vk::BufferCreateInfo::default()
            .size(u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("書き込みバイト数がu64に収まらない")))
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // 安全性: deviceは生成済みで有効。
        let handle = unsafe { device.create_buffer(&create_info, None)? };
        match メモリを結びつけて書く(device, メモリプロパティ, handle, &バイト列) {
            Ok(memory) => Ok(Self { handle, memory }),
            Err(誤り) => {
                // 安全性: handleはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(handle, None) };
                Err(誤り)
            }
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
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
