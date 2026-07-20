//! バリア導出が扱う、バッファの同期状態(stage/access)の組。画像状態と対になるが、
//! バッファにはレイアウトの概念が無いため2つ組で済む。

use ash::vk;

/// ある時点でバッファがどのVulkan同期状態にあるか。
/// 用途から写像された値、または外部登録時の初期値として使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct バッファ状態 {
    pub(crate) stage: vk::PipelineStageFlags2,
    pub(crate) access: vk::AccessFlags2,
}

impl バッファ状態 {
    pub(crate) fn 生成する(stage: vk::PipelineStageFlags2, access: vk::AccessFlags2) -> Self {
        Self { stage, access }
    }
}
