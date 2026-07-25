//! GPU環境の生成と、その環境を材料にする最初の資源(提示資源・実表示計測)の取得を1つの束にまとめる工程。
//! `generate_resources`より前の依存段であり、後続段が要る材料をこの束が全部そろえる。

use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::present_display_request::実表示計測要求;
use crate::renderer::present_resources::提示資源;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;
use crate::vulkan::gpu_environment::GPU環境;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub(super) struct コア資源 {
    pub(super) 環境: GPU環境,
    pub(super) 提示資源: 提示資源,
    pub(super) 検証カウンタ: 検証カウンタ,
    /// 選定したキューファミリのtimestamp_valid_bits > 0 か(判断30)。
    pub(super) タイムスタンプ対応か: bool,
    /// `vk::PhysicalDeviceLimits::timestamp_period`(1tickあたりのns)。
    pub(super) タイムスタンプ周期ns: f32,
    /// 実表示時刻計測の使用可否。非対応環境でも生成は成立し、記録開始時に非対応を返す。
    pub(super) 実表示計測: vulkan::present_timing::実表示計測,
}

pub(super) fn 組み立てる(
    表示ハンドル: RawDisplayHandle,
    ウィンドウハンドル: RawWindowHandle,
    寸法: ウィンドウ寸法,
    実表示計測要求: 実表示計測要求,
) -> Result<コア資源, レンダラーエラー> {
    let 検証カウンタ = 検証カウンタ::生成する();
    let 環境 = GPU環境::生成する(表示ハンドル, ウィンドウハンドル, &検証カウンタ, 実表示計測要求)?;
    let 実表示計測 = 環境.実表示計測を作る();
    let 提示資源 = 提示資源::生成する(&環境, 寸法)?;
    let (タイムスタンプ対応か, タイムスタンプ周期ns) =
        環境.物理デバイス問い合わせ().タイムスタンプ計測条件を調べる(環境.キューファミリ添字());

    Ok(コア資源 {
        環境,
        提示資源,
        検証カウンタ,
        タイムスタンプ対応か,
        タイムスタンプ周期ns,
        実表示計測,
    })
}
