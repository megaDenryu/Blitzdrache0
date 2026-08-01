//! ウィンドウなし実行で使う物理デバイスとキューファミリの選定。受け取るのはインスタンス、返すのは物理デバイスと添字である。
//!
//! 提示側の選定と条件が違う。あちらはサーフェスへの提示対応とグラフィックス能力を要求するが、
//! こちらはコンピュートの計算の発行と転送さえできればよい。専用のコンピュートキューがあればそれを選び、
//! 無ければグラフィックスも兼ねるキューを選ぶ。

use ash::vk;

use crate::error::{デバイス要件エラー, レンダラーエラー};

pub(super) fn 選定する(instance: &ash::Instance) -> Result<(vk::PhysicalDevice, u32), レンダラーエラー> {
    // 安全性: instanceは生成済みで有効。
    let 候補一覧 = unsafe { instance.enumerate_physical_devices()? };
    for 物理デバイス in 候補一覧 {
        if let Some(添字) = コンピュートキューを探す(instance, 物理デバイス) {
            return Ok((物理デバイス, 添字));
        }
    }
    Err(デバイス要件エラー::適合物理デバイスなし.into())
}

fn コンピュートキューを探す(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> Option<u32> {
    // 安全性: instance・物理デバイスは生成済み・列挙済みで有効。
    let ファミリ一覧 = unsafe { instance.get_physical_device_queue_family_properties(物理デバイス) };
    ファミリ一覧
        .iter()
        .enumerate()
        .filter(|(_, ファミリ)| ファミリ.queue_flags.contains(vk::QueueFlags::COMPUTE) && ファミリ.queue_flags.contains(vk::QueueFlags::TRANSFER))
        .map(|(添字, _)| 添字)
        .find_map(|添字| u32::try_from(添字).ok())
}
