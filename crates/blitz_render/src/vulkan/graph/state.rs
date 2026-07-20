//! バリア導出が扱う、画像の同期状態(stage/access/layout)の3つ組。

use ash::vk;

/// ある時点で画像がどのVulkan同期状態にあるか。
/// 用途から写像された値、または外部登録時の初期値として使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 画像状態 {
    pub(crate) stage: vk::PipelineStageFlags2,
    pub(crate) access: vk::AccessFlags2,
    pub(crate) layout: vk::ImageLayout,
}

impl 画像状態 {
    pub(crate) fn 生成する(
        stage: vk::PipelineStageFlags2,
        access: vk::AccessFlags2,
        layout: vk::ImageLayout,
    ) -> Self {
        Self { stage, access, layout }
    }
}
