//! 選定済みキューファミリのtimestamp_valid_bitsと、物理デバイスのtimestamp_periodを読む
//! (判断30の前提確認)。

use ash::vk;

pub(super) fn タイムスタンプ対応状況を調べる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
) -> (bool, f32) {
    // 安全性: instance・物理デバイスは選定済みで有効。
    let キューファミリ一覧 = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let 添字usize = usize::try_from(queue_family_index).unwrap_or_else(|_| panic!("キューファミリ添字がusizeに収まらない: {queue_family_index}"));
    let タイムスタンプ対応か = キューファミリ一覧.get(添字usize).is_some_and(|性質| 性質.timestamp_valid_bits > 0);

    // 安全性: instance・物理デバイスは選定済みで有効。
    let 性質 = unsafe { instance.get_physical_device_properties(physical_device) };
    (タイムスタンプ対応か, 性質.limits.timestamp_period)
}
