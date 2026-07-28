//! 布シミュレーション(判断54)1フレームぶんの入力。布付きレンダラーのときのみ`Some`で渡す。

use ash::vk;

use crate::vulkan::relative_anchor::カメラ相対アンカー;

pub(crate) struct 布描画入力 {
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) 介入pipeline: vk::Pipeline,
    pub(crate) 積分pipeline: vk::Pipeline,
    pub(crate) アタッチpipeline: vk::Pipeline,
    pub(crate) 拘束pipeline: vk::Pipeline,
    pub(crate) ハッシュ消去pipeline: vk::Pipeline,
    pub(crate) ハッシュ格納pipeline: vk::Pipeline,
    pub(crate) 分離pipeline: vk::Pipeline,
    pub(crate) 仕上げpipeline: vk::Pipeline,
    pub(crate) 頂点生成pipeline: vk::Pipeline,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 粒子数: u32,
    pub(crate) アタッチ件数: u32,
    pub(crate) 介入件数: u32,
    pub(crate) 粒子バッファ: vk::Buffer,
    pub(crate) 前位置バッファ: vk::Buffer,
    pub(crate) セルカウントバッファ: vk::Buffer,
    pub(crate) セル格納バッファ: vk::Buffer,
    pub(crate) 布頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) インデックス数: u32,
    pub(crate) 描画pipeline: vk::Pipeline,
    /// 布の粒子位置は世界原点を基準に計算されるため、アンカーは世界原点のカメラ相対値になる。
    pub(crate) 相対アンカー: カメラ相対アンカー,
}
