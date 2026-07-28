//! 開発用UI(egui)1フレームぶんの入力。1つのサブシステムの描画契約だけを持つため、布の入力と同じく専用ファイルへ置く。

use ash::vk;

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
