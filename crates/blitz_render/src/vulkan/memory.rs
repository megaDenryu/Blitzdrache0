//! Vulkanメモリ型の選定。読み戻し・頂点/インデックスバッファが使う
//! ホスト可視メモリと、深度バッファが使うデバイスローカルメモリの両方を集約する。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 専用メモリを確保する(
    device: &ash::Device,
    バイト数: u64,
    メモリ型添字: u32,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    let alloc_info = vk::MemoryAllocateInfo::default()
        .allocation_size(バイト数)
        .memory_type_index(メモリ型添字);
    // 安全性: deviceは生成済みで有効。alloc_infoはこの関数内の値だけを参照する。
    Ok(unsafe { device.allocate_memory(&alloc_info, None)? })
}

pub(crate) fn ホスト可視メモリ型を選ぶ(
    プロパティ一覧: &vk::PhysicalDeviceMemoryProperties,
    要求メモリ型ビット: u32,
) -> Result<u32, レンダラーエラー> {
    選ぶ(
        プロパティ一覧,
        要求メモリ型ビット,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )
    .ok_or(レンダラーエラー::ホスト可視メモリ型なし)
}

pub(crate) fn デバイスローカルメモリ型を選ぶ(
    プロパティ一覧: &vk::PhysicalDeviceMemoryProperties,
    要求メモリ型ビット: u32,
) -> Result<u32, レンダラーエラー> {
    選ぶ(プロパティ一覧, 要求メモリ型ビット, vk::MemoryPropertyFlags::DEVICE_LOCAL).ok_or(レンダラーエラー::デバイスローカルメモリ型なし)
}

fn 選ぶ(
    プロパティ一覧: &vk::PhysicalDeviceMemoryProperties, 要求メモリ型ビット: u32, 要求プロパティ: vk::MemoryPropertyFlags
) -> Option<u32> {
    let 有効件数 = usize::try_from(プロパティ一覧.memory_type_count).unwrap_or(0);
    プロパティ一覧.memory_types[..有効件数].iter().enumerate().find_map(|(添字, 型)| {
        let 添字u32 = u32::try_from(添字).ok()?;
        let 対象か = (要求メモリ型ビット & (1 << 添字u32)) != 0;
        let プロパティ適合か = 型.property_flags.contains(要求プロパティ);
        (対象か && プロパティ適合か).then_some(添字u32)
    })
}
