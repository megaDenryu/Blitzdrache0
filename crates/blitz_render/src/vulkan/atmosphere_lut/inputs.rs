//! 大気LUT生成パス1フレームぶんの入力。フレーム構成に空段階があり、かつそのフレームでLUTを焼くときだけ`Some`で渡す。
//!
//! 画像そのものでなくハンドルを運ばないのは、グラフへの登録が`graph_build`側の担当だからである。
//! ここが運ぶのは束縛先(パイプライン・レイアウト・ディスクリプタセット)と、ディスパッチするワークグループ数だけである。

use ash::vk;

/// 1本のコンピュートパスが束縛する資源とディスパッチの大きさ。
pub(crate) struct 大気LUT生成入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) ワークグループ数: [u32; 2],
}

/// そのフレームに積む大気LUTの生成パス一式と、パスが触れる画像。
/// 透過率と多重散乱を1つの`Option`が束ねるため、片方だけを焼く状態をフレーム記録からは作れない。
pub(crate) struct 大気LUT描画入力 {
    pub(crate) 透過率: 大気LUT生成入力,
    pub(crate) 多重散乱: 大気LUT生成入力,
    pub(crate) 透過率画像: vk::Image,
    pub(crate) 透過率ビュー: vk::ImageView,
    pub(crate) 透過率寸法: vk::Extent2D,
    pub(crate) 多重散乱画像: vk::Image,
    pub(crate) 多重散乱ビュー: vk::ImageView,
    pub(crate) 多重散乱寸法: vk::Extent2D,
}

/// 1ワークグループが受け持つテクセルの一辺。生成シェーダーの`numthreads`と一致させる。
const ワークグループの一辺: u32 = 8;

/// LUTの寸法から、全テクセルを覆うのに要るワークグループの数を求める。端数のワークグループは
/// 書き込み先の寸法を超えたスレッドを自分で捨てるため、切り上げてよい。
pub(super) fn ワークグループ数を求める(幅: u32, 高さ: u32) -> [u32; 2] {
    [幅.div_ceil(ワークグループの一辺), 高さ.div_ceil(ワークグループの一辺)]
}
