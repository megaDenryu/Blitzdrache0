//! 空パス1フレームぶんの入力。フレーム構成に空段階があるときのみ`Some`で渡す。

use ash::vk;

/// 空パスが束縛する資源。係数はフレームユニフォーム(binding3)に載るため、ここには載らない。
///
/// ディスクリプタセットは走査順で最初の描画対象のものである。空のシェーダーが読むのはフレームユニフォームだけであり、
/// どの描画対象のセットでも同じ資源を指すため、可視判定の結果でこの値は変わらない(布の描画と同じ理由)。
pub(crate) struct 空描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
}
