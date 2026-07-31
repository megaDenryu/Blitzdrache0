//! ウィンドウなし実行GPU環境のvalidation層。担当する工程は「層と拡張が機材にあるかを調べ、あればそれらを有効にした
//! インスタンスとdebug utilsメッセンジャーを作る」ことである。受け取るのはVulkanローダーと検証カウンタ、
//! 返すのはインスタンス・メッセンジャー・層の在否である。
//!
//! 提示側(`vulkan/instance.rs`)がデバッグビルドで層を無条件に要求するのに対し、ここが在否を先に調べるのは、
//! 読み戻し検査が層を入れていない機材でも走らなければならず、かつ層が無いことを無言で通してはならないためである。
//! 層が無ければ指摘は1件も観測できず、指摘0件は何の保証にもならない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::validation_counter::{検証カウンタ, 検証層の状況};
use crate::vulkan::debug_messenger::デバッグメッセンジャー;

const 検証層名: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";

/// 生成済みのインスタンスと、そこへ結び付けたメッセンジャー。破棄はメッセンジャーが先である。
pub(super) struct 検証つきインスタンス {
    pub(super) instance: ash::Instance,
    pub(super) メッセンジャー: Option<デバッグメッセンジャー>,
    pub(super) 状況: 検証層の状況,
}

pub(super) fn 作る(entry: &ash::Entry, カウンタ: &検証カウンタ) -> Result<検証つきインスタンス, レンダラーエラー> {
    let 状況 = 在否を調べる(entry)?;
    let instance = インスタンスを作る(entry, 状況)?;
    let メッセンジャー = match 状況 {
        検証層の状況::環境に無い => None,
        検証層の状況::有効 => match デバッグメッセンジャー::生成する(entry, &instance, カウンタ) {
            Ok(メッセンジャー) => Some(メッセンジャー),
            Err(誤り) => {
                // 安全性: instanceはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { instance.destroy_instance(None) };
                return Err(誤り);
            }
        },
    };
    Ok(検証つきインスタンス {
        instance,
        メッセンジャー,
        状況,
    })
}

impl 検証つきインスタンス {
    /// 注意: メッセンジャーをインスタンスより先に破棄する(メッセンジャーがインスタンスの子オブジェクトであるため)。
    pub(super) fn 破棄する(&self) {
        if let Some(メッセンジャー) = &self.メッセンジャー {
            メッセンジャー.破棄する();
        }
        // 安全性: instanceはSelfが唯一の所有者であり、破棄時点で子オブジェクトはすべて片付いている。
        unsafe { self.instance.destroy_instance(None) };
    }
}

/// 層そのものと、層が提供するdebug utils拡張の両方がそろっているかを調べる。
/// 拡張を層一覧とは別に調べるのは、層があっても拡張を出さない構成では`vkCreateInstance`が失敗するためである。
fn 在否を調べる(entry: &ash::Entry) -> Result<検証層の状況, レンダラーエラー> {
    // 安全性: entryは読み込み済みで有効。
    let 層一覧 = unsafe { entry.enumerate_instance_layer_properties()? };
    let 層がある = 層一覧.iter().any(|層| 層.layer_name_as_c_str().is_ok_and(|名前| 名前 == 検証層名));
    if !層がある {
        return Ok(検証層の状況::環境に無い);
    }
    if !拡張がある(entry, None)? && !拡張がある(entry, Some(検証層名))? {
        return Ok(検証層の状況::環境に無い);
    }
    Ok(検証層の状況::有効)
}

fn 拡張がある(entry: &ash::Entry, 層名: Option<&std::ffi::CStr>) -> Result<bool, レンダラーエラー> {
    // 安全性: entryは読み込み済みで有効。層名はこのスコープの静的文字列を指す。
    let 一覧 = unsafe { entry.enumerate_instance_extension_properties(層名)? };
    Ok(一覧
        .iter()
        .any(|拡張| 拡張.extension_name_as_c_str().is_ok_and(|名前| 名前 == ash::ext::debug_utils::NAME)))
}

/// 層を有効にできるときだけ、層名とdebug utils拡張と同期検証を要求する。
/// 同期検証を足すのは、この検査がレンダーグラフのバリア導出そのものを確かめる経路だからである。
fn インスタンスを作る(entry: &ash::Entry, 状況: 検証層の状況) -> Result<ash::Instance, レンダラーエラー> {
    let アプリ情報 = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 3, 0));
    let 層一覧 = [検証層名.as_ptr()];
    let 拡張一覧 = [ash::ext::debug_utils::NAME.as_ptr()];
    let 有効化する検証機能 = [vk::ValidationFeatureEnableEXT::SYNCHRONIZATION_VALIDATION];
    let mut 検証機能情報 = vk::ValidationFeaturesEXT::default().enabled_validation_features(&有効化する検証機能);
    let mut 生成情報 = vk::InstanceCreateInfo::default().application_info(&アプリ情報);
    if 状況 == 検証層の状況::有効 {
        生成情報 = 生成情報
            .enabled_layer_names(&層一覧)
            .enabled_extension_names(&拡張一覧)
            .push_next(&mut 検証機能情報);
    }
    // 安全性: 生成情報はこのスコープの値と静的文字列だけを参照する。
    Ok(unsafe { entry.create_instance(&生成情報, None)? })
}
