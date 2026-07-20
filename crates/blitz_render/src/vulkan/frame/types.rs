//! 1フレームの描画で受け渡す型: 描画方式・ジオメトリ入力・粒子描画入力。

use ash::vk;

/// このフレームの描画後処理: 通常の提示前遷移のみか、読み戻し用のコピーを挟むか。
pub(crate) enum 描画方式 {
    通常,
    読み戻し { バッファ: vk::Buffer },
}

/// 頂点/インデックスバッファと、マテリアルテクスチャ+フレームユニフォームを
/// 束ねたディスクリプタセット。ビュー射影行列等はUBO(判断24)経由で渡すため
/// ここには含まない。パイプラインのlayoutはディスクリプタセットの送信先を
/// 指定するために必要。
pub(crate) struct ジオメトリ入力 {
    pub(crate) 頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) インデックス数: u32,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
}

/// シャドウパス(判断35)1フレームぶんの入力。常に存在する(シーンパスと同じ
/// 頂点/インデックスバッファ・ディスクリプタセットを、シャドウ専用の
/// パイプライン/layoutで束ね直すだけのため`ジオメトリ入力`とは別型にする)。
pub(crate) struct シャドウ描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) 頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) インデックス数: u32,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
}

/// GPU粒子トイ(判断29)1フレームぶんの入力。`--particles`指定時のみ`Some`で渡す。
/// 呼び出し元(renderer層)がフレーム添字に対応するディスクリプタセットを
/// あらかじめ選んで渡す(`ジオメトリ入力`と同じ設計)。
pub(crate) struct 粒子描画入力 {
    pub(crate) コンピュートパイプライン: vk::Pipeline,
    pub(crate) コンピュートlayout: vk::PipelineLayout,
    pub(crate) 描画パイプライン: vk::Pipeline,
    pub(crate) 描画layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) バッファ: vk::Buffer,
}

/// トーンマップパス(判断38・39)1フレームぶんの入力。ポストプロセス有効時のみ`Some`で渡す。
pub(crate) struct トーンマップ描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    /// トーンマップ前にHDR輝度へ掛ける露出倍率(プッシュ定数で渡す)。
    pub(crate) 露出: f32,
}

/// 開発用UI(egui)1フレームぶんの入力。呼び出し元(renderer層)が今フレームの
/// メッシュ列をジオメトリバッファへ書き込み済みで、そのバッファハンドルと
/// メッシュごとの描画項目一覧を渡す(判断33・34)。
pub(crate) struct UI描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) 頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) 項目一覧: Vec<UI描画項目>,
}

/// UIメッシュ1つぶんの描画項目: 結合バッファ内での要素オフセット(頂点/インデックスの
/// 両バッファは1回だけ束縛し、`cmd_draw_indexed`のfirst_index/vertex_offsetで
/// メッシュごとの範囲を指定する)と、テクスチャ・シザー矩形。
pub(crate) struct UI描画項目 {
    pub(crate) 頂点要素オフセット: i32,
    pub(crate) インデックス要素オフセット: u32,
    pub(crate) インデックス数: u32,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) シザー: vk::Rect2D,
}
