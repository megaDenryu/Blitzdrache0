//! デバッグビルドでのみvalidationメッセンジャーを生成する。

use crate::error::レンダラーエラー;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

pub(super) fn デバッグメッセンジャーを作る(
    entry: &ash::Entry,
    instance: &ash::Instance,
    検証カウンタ: &検証カウンタ,
    デバッグ有効か: bool,
) -> Result<Option<vulkan::debug_messenger::デバッグメッセンジャー>, レンダラーエラー> {
    デバッグ有効か
        .then(|| vulkan::debug_messenger::デバッグメッセンジャー::生成する(entry, instance, 検証カウンタ))
        .transpose()
}
