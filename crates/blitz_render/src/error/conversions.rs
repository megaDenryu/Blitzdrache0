//! 外部Vulkanエラーと数学境界のエラーを公開エラー型へ閉じ込める変換。

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

/// 多段の組み立てが返すライティング入力の失敗を、層の`多段エラー`を経てレンダラーの失敗へ写す。
/// 2段の`#[from]`は連鎖しないため、`?`で直接伝播できるようここで明示する。
impl From<crate::lighting_input::ライティング入力エラー> for レンダラーエラー {
    fn from(誤り: crate::lighting_input::ライティング入力エラー) -> Self {
        Self::多段不正(crate::error::多段エラー::構築失敗(誤り))
    }
}

impl From<blitz_math::座標変換エラー> for レンダラーエラー {
    fn from(誤り: blitz_math::座標変換エラー) -> Self {
        Self::カメラ相対変換失敗(誤り)
    }
}

/// 間接照明の層の内側の2つの失敗を、層の型を経てレンダラーの失敗へ写す。
/// 2段の`#[from]`は連鎖しないため、`?`で直接伝播できるようここで明示する。
impl From<crate::indirect_lighting::間接照明契約エラー> for レンダラーエラー {
    fn from(誤り: crate::indirect_lighting::間接照明契約エラー) -> Self {
        Self::間接照明不正(誤り.into())
    }
}

impl From<crate::indirect_lighting::間接照明入力不一致エラー> for レンダラーエラー {
    fn from(誤り: crate::indirect_lighting::間接照明入力不一致エラー) -> Self {
        Self::間接照明不正(誤り.into())
    }
}
