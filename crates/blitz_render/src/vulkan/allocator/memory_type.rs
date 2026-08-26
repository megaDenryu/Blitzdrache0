//! 専用メモリをどこへ置くかの区分と、その区分に適合するメモリ型の選定。
//! 論理デバイスにもGPUの状態にも触れない純粋な計算であり、材料は物理デバイスが並べるメモリ性質の表と要求ビットだけである。

use ash::vk;

use crate::error::レンダラーエラー;

/// ホストから書けることを求めるか、GPUからの読み書きの速さだけを求めるかの区分。
#[derive(Clone, Copy)]
pub(super) enum メモリの置き場 {
    ホスト可視,       // ホストから写像して書き込める場所。ステージングとシェーダー定数が使う
    デバイスローカル, // GPUの中だけで読み書きする場所。頂点・索引・記憶バッファと画像が使う
}

impl メモリの置き場 {
    pub(super) fn 適合する型の添字を選ぶ(
        self,
        プロパティ一覧: &vk::PhysicalDeviceMemoryProperties,
        要求メモリ型ビット: u32,
    ) -> Result<u32, レンダラーエラー> {
        match self {
            Self::ホスト可視 => 選ぶ(
                プロパティ一覧,
                要求メモリ型ビット,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
            .ok_or(レンダラーエラー::ホスト可視メモリ型なし),
            Self::デバイスローカル => {
                選ぶ(プロパティ一覧, 要求メモリ型ビット, vk::MemoryPropertyFlags::DEVICE_LOCAL).ok_or(レンダラーエラー::デバイスローカルメモリ型なし)
            }
        }
    }
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
