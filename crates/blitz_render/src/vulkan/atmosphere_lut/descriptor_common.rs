//! 大気のベイク済み画像のディスクリプタが共通に使う、フレームスロット数ぶんの数え方。
//! 透過率生成と多重散乱生成のどちらもスロットごとに1セットを持つため、プールの容量を決める数をこの1箇所が保つ。
//! 割り当てそのものは宣言から作ったセットレイアウトが持つ(参照: `crates/blitz_render/src/vulkan/descriptor/declared_bindings/set_layout.rs`)。

use crate::vulkan::sync::進行中フレーム数;

pub(in crate::vulkan) fn セット数() -> u32 {
    u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない: {進行中フレーム数}"))
}
