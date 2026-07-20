//! ホスト可視・コヒーレントなメモリ型の選定。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn ホスト可視メモリ型を選ぶ(
    プロパティ一覧: &vk::PhysicalDeviceMemoryProperties,
    要求メモリ型ビット: u32,
) -> Result<u32, レンダラーエラー> {
    let 要求プロパティ = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
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
        .ok_or(レンダラーエラー::ホスト可視メモリ型なし)
}
