//! 布シミュレーション(判断54)1フレームぶんの入力と、布の描画が束縛する外部資源。布付きレンダラーのときのみ`Some`で渡す。

use ash::vk;

use crate::vulkan::relative_anchor::カメラ相対の基準原点;

/// 布描画の外部資源とは、布の描画コマンドが束縛する資源のうち、布一式でなくレンダラーが所有するもののことである。
/// 布はシーンパスにもシャドウパスにも、通常の描画対象が1件も可視でないフレームに描かれる。
/// パイプラインもレイアウトも可視個体の有無と無関係に存在するため、可視な通常入力の先頭から借りずにこの型で受け取る。
/// 布が読むビューとパスのセットと照明問い合わせのセットは描画対象によらず同じであり、`共有セット束縛`が運ぶ。
pub(crate) struct 布描画の外部資源 {
    pub(crate) シャドウ: 布シャドウ描画入力, // シャドウパスで布だけを描く専用の一式
}

/// シャドウパスで布を描くための束縛先。パイプラインは布専用であり、描画対象からは1つも借りない。
/// これにより布の影は描画対象の登録数にも走査順にも依存しない
/// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
pub(crate) struct 布シャドウ描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

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
    pub(crate) 描画layout: vk::PipelineLayout, // シーンパスで`描画pipeline`を束縛したまま、プッシュ定数とディスクリプタセットを送る先
    pub(crate) 相対の基準原点: カメラ相対の基準原点, // 布の粒子位置は世界原点基準のため、基準原点は世界原点のカメラ相対値になる
    pub(crate) 外部資源: 布描画の外部資源,
}
