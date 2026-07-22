//! ホスト可視バッファの確保とホストからの書き込み。頂点/インデックス/テクスチャの
//! ステージングバッファがすべてこの1箇所を経由する(判断20の転送基盤)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::memory;

pub(crate) fn 確保して書き込む(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    データ: &[u8],
    用途: vk::BufferUsageFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    let create_info = vk::BufferCreateInfo::default()
        .size(u64::try_from(データ.len()).unwrap_or_else(|_| panic!("バッファサイズがu64に収まらない")))
        .usage(用途)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    // 安全性: deviceは生成済みで有効。
    let buffer = unsafe { device.create_buffer(&create_info, None)? };
    // 安全性: bufferは直前に生成済み。
    let 要件 = unsafe { device.get_buffer_memory_requirements(buffer) };

    let メモリ型添字 = match memory::ホスト可視メモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits) {
        Ok(添字) => 添字,
        Err(誤り) => {
            // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(誤り);
        }
    };
    let memory = match memory::専用メモリを確保する(device, 要件.size, メモリ型添字) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(誤り);
        }
    };
    // 安全性: buffer・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    unsafe { device.bind_buffer_memory(buffer, memory, 0)? };

    上書きする(device, memory, データ)?;
    Ok((buffer, memory))
}

/// 確保済みのホスト可視メモリへ内容を書き込む(全置き換え)。フレームユニフォームの
/// 毎フレーム更新(判断24)がこの関数を再利用する。
pub(crate) fn 上書きする(device: &ash::Device, memory: vk::DeviceMemory, データ: &[u8]) -> Result<(), レンダラーエラー> {
    // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済み。WHOLE_SIZEでの
    // マッピングは確保済み容量全体を対象にするため、データ長がバッファ容量以下である
    // 限り常に安全。
    let ポインタ = unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
    // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、データ長は生成時に
    // 同じ長さで要求したバッファ容量以下(呼び出し元が長さを一致させて確保している)。
    unsafe { std::ptr::copy_nonoverlapping(データ.as_ptr(), ポインタ.cast::<u8>(), データ.len()) };
    // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
    unsafe { device.unmap_memory(memory) };
    Ok(())
}
