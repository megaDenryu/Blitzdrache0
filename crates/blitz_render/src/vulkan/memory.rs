//! Vulkanメモリ型の選定。読み戻し・頂点/インデックスバッファが使う
//! ホスト可視メモリと、深度バッファが使うデバイスローカルメモリの両方を集約する。

use ash::vk;

use crate::error::レンダラーエラー;

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
    選ぶ(プロパティ一覧, 要求メモリ型ビット, vk::MemoryPropertyFlags::DEVICE_LOCAL)
        .ok_or(レンダラーエラー::デバイスローカルメモリ型なし)
}

fn 選ぶ(
    プロパティ一覧: &vk::PhysicalDeviceMemoryProperties,
    要求メモリ型ビット: u32,
    要求プロパティ: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let 有効件数 = usize::try_from(プロパティ一覧.memory_type_count).unwrap_or(0);
    プロパティ一覧.memory_types[..有効件数]
        .iter()
        .enumerate()
        .find_map(|(添字, 型)| {
            let 添字u32 = u32::try_from(添字).ok()?;
            let 対象か = (要求メモリ型ビット & (1 << 添字u32)) != 0;
            let プロパティ適合か = 型.property_flags.contains(要求プロパティ);
            (対象か && プロパティ適合か).then_some(添字u32)
        })
}
