//! VK_KHR_present_id / VK_KHR_present_wait の対応判定。拡張の有無と機能フラグの両方を確かめる。

use std::ffi::CStr;

use ash::vk;

use crate::present_display::実表示計測状況;
use crate::present_display::実表示計測要求;

/// 論理デバイス生成時に有効化する拡張名。present_wait は present_id に依存するため常に2つで1組にする。
pub(crate) const 有効化する拡張名一覧: [&CStr; 2] = [ash::khr::present_id::NAME, ash::khr::present_wait::NAME];

/// 拡張が存在するだけでは使えないため、機能フラグまで確認して対応状況を決める。
/// 注意: 要求が無いときは問い合わせ自体を行わない。拡張を有効化しないことが既定条件の維持そのものだからである。
pub(crate) fn 調べる(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice, 要求: 実表示計測要求) -> 実表示計測状況 {
    if 要求 == 実表示計測要求::要求しない {
        return 実表示計測状況::要求していない;
    }
    if !両拡張があるか(instance, 物理デバイス) {
        return 実表示計測状況::拡張が無い;
    }
    if !両機能が使えるか(instance, 物理デバイス) {
        return 実表示計測状況::機能が無効;
    }
    実表示計測状況::計測できる
}

fn 両拡張があるか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    // 安全性: instance・物理デバイスは選定済みで、この呼び出しの間有効である。
    let 列挙結果 = unsafe { instance.enumerate_device_extension_properties(物理デバイス) };
    let Ok(一覧) = 列挙結果 else {
        return false;
    };
    有効化する拡張名一覧.iter().all(|名前| 含むか(&一覧, 名前))
}

fn 含むか(一覧: &[vk::ExtensionProperties], 名前: &CStr) -> bool {
    一覧.iter().any(|性質| 性質.extension_name_as_c_str() == Ok(名前))
}

fn 両機能が使えるか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    let mut 提示id機能 = vk::PhysicalDevicePresentIdFeaturesKHR::default();
    let mut 提示待機機能 = vk::PhysicalDevicePresentWaitFeaturesKHR::default();
    let mut 機能 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut 提示id機能)
        .push_next(&mut 提示待機機能);
    // 安全性: instance・物理デバイスは選定済みで有効。連結した2つの機能構造体はこの関数内で生存する。
    unsafe { instance.get_physical_device_features2(物理デバイス, &mut 機能) };
    提示id機能.present_id == vk::TRUE && 提示待機機能.present_wait == vk::TRUE
}
