//! debug utilsメッセンジャー。validationのエラー・警告を検証カウンタへ集計し、
//! メッセージ全文をstderrへ出力する。

use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use ash::vk;

use crate::error::レンダラーエラー;
use crate::validation_counter::検証カウンタ;

/// 生成済みのdebug utilsメッセンジャー。破棄まで内部でカウンタのArcを保持する。
pub(crate) struct デバッグメッセンジャー {
    loader: ash::ext::debug_utils::Instance,
    handle: vk::DebugUtilsMessengerEXT,
    _カウンタ保持: Arc<AtomicU64>, // 注意: コールバックへ渡した生ポインタの指す先を破棄まで生存させる
}

impl デバッグメッセンジャー {
    pub(crate) fn 生成する(
        entry: &ash::Entry, instance: &ash::Instance, カウンタ: &検証カウンタ
    ) -> Result<Self, レンダラーエラー> {
        let loader = ash::ext::debug_utils::Instance::new(entry, instance);
        let カウンタ保持 = カウンタ.内部参照を複製する();

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING)
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(コールバック))
            .user_data(Arc::as_ptr(&カウンタ保持).cast_mut().cast::<c_void>());

        // 安全性: create_infoのuser_dataはカウンタ保持（このArc）が指す実体のポインタで、
        // このArcはSelfのフィールドとして破棄処理まで生存する。ここで渡す関数ポインタは
        // extern "system" fnであり、Vulkanローダーが要求するABIに一致する。
        let handle = unsafe { loader.create_debug_utils_messenger(&create_info, None)? };

        Ok(Self {
            loader,
            handle,
            _カウンタ保持: カウンタ保持,
        })
    }

    pub(crate) fn 破棄する(&self) {
        // 安全性: handleはSelfが生成した唯一の所有者であり、この呼び出しが
        // インスタンス破棄より必ず先に行われることをレンダラーの破棄順序が保証する。
        unsafe { self.loader.destroy_debug_utils_messenger(self.handle, None) };
    }
}

unsafe extern "system" fn コールバック(
    _severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    user_data: *mut c_void,
) -> vk::Bool32 {
    // 安全性: dataとuser_dataはVulkanローダーが本コールバックの契約どおりに渡す。
    // user_dataは生成時にArc<AtomicU64>の生ポインタを渡しており、
    // メッセンジャー破棄まで指す先が生存することは呼び出し元が保証する。
    unsafe {
        if let Some(カウンタ) = user_data.cast::<AtomicU64>().as_ref() {
            カウンタ.fetch_add(1, Ordering::SeqCst);
        }
        if let Some(データ) = data.as_ref() {
            let メッセージ = データ.message_as_c_str().map(|c| c.to_string_lossy());
            if let Some(メッセージ) = メッセージ {
                eprintln!("[validation] {メッセージ}");
            }
        }
    }
    vk::FALSE
}
