//! 外部Vulkanエラーを公開エラー型へ閉じ込める変換。

use super::レンダラーエラー;
use crate::vulkan_failure::Vulkan失敗コード;

impl From<ash::LoadingError> for レンダラーエラー {
    fn from(誤り: ash::LoadingError) -> Self {
        Self::ローダー読み込み失敗(誤り.to_string())
    }
}

impl From<ash::vk::Result> for レンダラーエラー {
    fn from(結果: ash::vk::Result) -> Self {
        Self::Vulkan呼び出し失敗(Vulkan失敗コード::生成する(結果))
    }
}
