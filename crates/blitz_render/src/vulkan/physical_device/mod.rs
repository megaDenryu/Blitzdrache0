//! 物理デバイスとキューファミリの選定。全候補を列挙し、基礎要件を満たすものを選定候補へ写して選ぶ規則へ渡す。
//! 基礎要件の判定は`base_requirement`、候補の材料は`candidate`、選ぶ規則は`choose`が持つ。

mod base_requirement;
mod candidate;
#[cfg(test)]
mod candidate_fixture;
mod choose;
#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod feature_choose_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor_indexing;
use candidate::選定候補;

/// 基礎要件(グラフィックス描画とサーフェス提示、dynamicRendering・synchronization2・shaderDrawParameters)と
/// ディスクリプタ索引の最低機能要件とテクスチャのブロック圧縮と立方体の配列画像への対応をすべて満たす物理デバイスを選ぶ。
/// 満たす候補の中ではdiscrete GPUを優先する。
pub(in crate::vulkan) fn 選定する(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<(vk::PhysicalDevice, u32), レンダラーエラー> {
    // 安全性: instanceは生成済みで、この呼び出しの間有効であることを呼び出し元が保証する。
    let 一覧 = unsafe { instance.enumerate_physical_devices()? };

    let mut 適合一覧: Vec<(vk::PhysicalDevice, u32)> = Vec::new();
    let mut 候補一覧: Vec<選定候補> = Vec::new();
    for 物理デバイス in 一覧 {
        let 探索結果 = base_requirement::満たすキューファミリを探す(instance, surface_loader, surface, 物理デバイス)?;
        let Some(キューファミリ) = 探索結果 else {
            continue;
        };
        候補一覧.push(候補を作る(instance, 物理デバイス, 適合一覧.len()));
        適合一覧.push((物理デバイス, キューファミリ));
    }

    let 選んだ添字 = choose::選ぶ(&候補一覧)?;
    let Some(&(物理デバイス, キューファミリ)) = 適合一覧.get(選んだ添字) else {
        panic!("選んだ候補の添字{選んだ添字}が適合一覧の外を指した(候補と適合ハンドルを同じ走査で1件ずつ積む不変条件が破れている)");
    };
    Ok((物理デバイス, キューファミリ))
}

fn 候補を作る(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice, 添字: usize) -> 選定候補 {
    // 安全性: instance・物理デバイスは生成・列挙済みで有効。
    let 性質 = unsafe { instance.get_physical_device_properties(物理デバイス) };
    let 機材名 = 性質
        .device_name_as_c_str()
        .map_or_else(|_| "(機材名を読めなかった)".to_string(), |名前| 名前.to_string_lossy().into_owned());
    let discreteか = 性質.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
    選定候補::生成する(
        添字,
        機材名,
        discreteか,
        descriptor_indexing::機能を採取する(instance, 物理デバイス),
        物理デバイスがテクスチャのブロック圧縮に対応するか(instance, 物理デバイス),
        物理デバイスが立方体の配列画像に対応するか(instance, 物理デバイス),
    )
}

/// 物理デバイスが`textureCompressionBC`(BC1からBC7までのブロック圧縮形式の族)に対応するか。
/// 非対応の機材へ非圧縮の版を焼き直す枝も、実行時にCPUで展開する枝も持たないため、対応しない機材は候補から外す
/// (参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「判断e」)。
///
/// 注意: 採取と有効化は別の場所にある。同じ機能を`crates/blitz_render/src/vulkan/device.rs`が論理デバイスの生成で立てる。
/// 片方だけを増減させると、選定を通った機材で有効化していない形式の画像を作ることになる。
fn 物理デバイスがテクスチャのブロック圧縮に対応するか(
    instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice
) -> bool {
    // 安全性: instance・物理デバイスは列挙済みで有効。
    let 機能 = unsafe { instance.get_physical_device_features(物理デバイス) };
    機能.texture_compression_bc == vk::TRUE
}

/// 物理デバイスが`imageCubeArray`(立方体の面を配列として持つ画像ビュー)に対応するか。
/// 点光源の影が1枚の立方体配列を標本する形に固まっており、面ごとの2D配列として引き直す枝を持たないため、
/// 対応しない機材は候補から外す(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断l」)。
///
/// 注意: 採取と有効化は別の場所にある。同じ機能を`crates/blitz_render/src/vulkan/device.rs`が論理デバイスの生成で立てる。
/// 片方だけを増減させると、選定を通った機材で有効化していない機能に依存したビューを作ることになる。
fn 物理デバイスが立方体の配列画像に対応するか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    // 安全性: instance・物理デバイスは列挙済みで有効。
    let 機能 = unsafe { instance.get_physical_device_features(物理デバイス) };
    機能.image_cube_array == vk::TRUE
}

/// 物理デバイスが`largePoints`(1.0を超えるSV_PointSize出力)に対応するか。
/// 粒子描画のPointSize指定(判断29)が効くかどうかを左右するのみで、
/// 未対応でも点は既定サイズで描かれ続けるため物理デバイス選定条件には含めない。
pub(in crate::vulkan) fn 大きな点描画に対応するか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    // 安全性: instance・物理デバイスは列挙済みで有効。
    let 機能 = unsafe { instance.get_physical_device_features(物理デバイス) };
    機能.large_points == vk::TRUE
}
