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

/// 空中遠近合成パス1フレームぶんの入力。大気LUT腕で合成を切っていないフレームだけ`Some`で渡す。
///
/// 最遠距離をここが運ぶのは、ボリュームを焼いた条件と引く条件を同じ値から作るためである。フレームユニフォームへ
/// 写すと、焼いた刻みと引く刻みが別の経路で更新されて食い違いうる。
pub(crate) struct 空中遠近合成描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    /// set0。フレームユニフォームを読むために走査順で最初の描画対象のセットを借りる(空パスと同じ理由)。
    pub(crate) シーンセット: vk::DescriptorSet,
    /// set1。深度と空中遠近ボリュームを結ぶ。
    pub(crate) 合成セット: vk::DescriptorSet,
    /// ボリュームの奥行きの目盛りの最遠端(メートル)。
    pub(crate) 最遠距離: f32,
}
