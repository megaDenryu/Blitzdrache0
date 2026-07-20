//! Vulkanインスタンス生成。デバッグビルドではvalidation layerと同期検証を有効化する。

use ash::vk;
use raw_window_handle::RawDisplayHandle;

use crate::error::レンダラーエラー;

/// Vulkanインスタンスを生成する。
///
/// `デバッグ有効か` が真のとき、`VK_LAYER_KHRONOS_validation` を有効化し、
/// `VkValidationFeaturesEXT` でSYNCHRONIZATION_VALIDATIONを追加で有効にする。
pub(crate) fn 生成する(
    entry: &ash::Entry,
    表示ハンドル: RawDisplayHandle,
    デバッグ有効か: bool,
) -> Result<ash::Instance, レンダラーエラー> {
    let app_info = vk::ApplicationInfo::default()
        .application_name(c"Blitzdrache0")
        .engine_name(c"Blitzdrache0")
        .api_version(vk::API_VERSION_1_3);

    let mut 拡張一覧: Vec<*const std::ffi::c_char> =
        ash_window::enumerate_required_extensions(表示ハンドル)?.to_vec();
    if デバッグ有効か {
        拡張一覧.push(ash::ext::debug_utils::NAME.as_ptr());
    }

    let 層一覧: [*const std::ffi::c_char; 1] = [c"VK_LAYER_KHRONOS_validation".as_ptr()];

    let 有効化する検証機能: [vk::ValidationFeatureEnableEXT; 1] =
        [vk::ValidationFeatureEnableEXT::SYNCHRONIZATION_VALIDATION];
    let mut 検証機能情報 =
        vk::ValidationFeaturesEXT::default().enabled_validation_features(&有効化する検証機能);

    let mut create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&拡張一覧);

    if デバッグ有効か {
        create_info = create_info
            .enabled_layer_names(&層一覧)
            .push_next(&mut 検証機能情報);
    }

    // 安全性: create_info は関数内で構築した値を指すのみで、entryが正しくロードされて
    // いることは呼び出し元(レンダラー生成)が保証する。
    let instance = unsafe { entry.create_instance(&create_info, None)? };
    Ok(instance)
}
