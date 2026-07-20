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
