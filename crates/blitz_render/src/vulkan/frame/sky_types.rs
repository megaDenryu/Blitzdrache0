//! 空パス1フレームぶんの入力。フレーム構成に空段階があるときのみ`Some`で渡す。

use ash::vk;

/// 空パスが放射輝度をどう評価するか。腕ごとに束縛するディスクリプタセットの本数が変わるため、
/// 追加のセットを腕が持つ。値のある腕だけがset1を束縛するため、束縛の有無をこの型が決める。
#[derive(Clone, Copy)]
pub(crate) enum 空描画の方式 {
    /// Hosek-Wilkie解析近似。set0のフレームユニフォームだけを読む。
    Hosek解析近似,
    /// 大気LUT。set1でスカイビューLUTと透過率LUTと媒体のユニフォームを読む。
    大気LUT { 標本セット: vk::DescriptorSet },
}

/// 空パスが束縛する資源。Hosek腕の係数はフレームユニフォーム(binding3)に載るため、ここには載らない。
///
/// `ディスクリプタセット`は走査順で最初の描画対象のものである。空のシェーダーがそこから読むのはフレームユニフォームだけであり、
/// どの描画対象のセットでも同じ資源を指すため、可視判定の結果でこの値は変わらない(布の描画と同じ理由)。
pub(crate) struct 空描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 方式: 空描画の方式,
}
