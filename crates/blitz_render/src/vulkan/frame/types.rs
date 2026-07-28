//! 1フレームの描画で受け渡す型: 描画方式・ジオメトリ入力・シャドウ描画入力・粒子描画入力・スキニング描画入力・ポスト処理の描画入力。
//! 布とUIは対応するサブシステムの入力だけを持つため`cloth_types`・`ui_types`が別に持つ。

use ash::vk;

use crate::vulkan::relative_anchor::カメラ相対アンカー;

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
    /// この描画発行で描く個体の数。通常メッシュと地形は1、インスタンス群はそのLOD段の可視数である。
    pub(crate) インスタンス数: u32,
    /// 可視ID列のうちこの発行が読み始める位置。頂点シェーダーは`SV_StartInstanceLocation`でこの値を受け取り、
    /// `SV_InstanceID`に足して可視ID列を引く。段ごとに1回発行するため、段の範囲の開始がそのままこの値になる。
    pub(crate) 先頭インスタンス: u32,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    /// この描画のアンカーからカメラ大域原点を引いた値。プッシュ定数で頂点ステージへ渡す。
    pub(crate) 相対アンカー: カメラ相対アンカー,
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
    /// この描画発行で描く個体の数。シャドウパスは可視判定を持たないため、常にそのLOD段の全個体を描く。
    pub(crate) インスタンス数: u32,
    /// 可視ID列のうちこの発行が読み始める位置。シーンパスと同じ段の範囲の開始を使う。
    pub(crate) 先頭インスタンス: u32,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 相対アンカー: カメラ相対アンカー,
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
    pub(crate) 更新スレッド数: u32,
    pub(crate) 描画要素数: u32,
    /// 粒子の位置は世界原点を基準に計算されるため、アンカーは世界原点のカメラ相対値になる。
    pub(crate) 相対アンカー: カメラ相対アンカー,
}

/// GPUスキニング(判断44)1フレームぶんの入力。スキン付きシーンのときのみ`Some`で渡す。
pub(crate) struct スキニング描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 頂点数: u32,
    /// スキン済み頂点バッファ。グラフ登録とシーン/シャドウの頂点入力差し替えに使う。
    pub(crate) 出力バッファ: vk::Buffer,
}

/// ブルームピラミッド(判断41)1フレームぶんの入力。ポストプロセス有効時のみ`Some`で渡す。
/// セット一覧の長さは段数-1(縮小set[i]は縮小[i+1]へのパスの読み元、拡大set[i]は拡大[i]へのパスの読み元)。
pub(crate) struct ブルーム描画入力 {
    pub(crate) 前処理pipeline: vk::Pipeline,
    pub(crate) 前処理layout: vk::PipelineLayout,
    pub(crate) 縮小pipeline: vk::Pipeline,
    pub(crate) 縮小layout: vk::PipelineLayout,
    pub(crate) 拡大pipeline: vk::Pipeline,
    pub(crate) 拡大layout: vk::PipelineLayout,
    pub(crate) 前処理set: vk::DescriptorSet,
    pub(crate) 縮小set一覧: Vec<vk::DescriptorSet>,
    pub(crate) 拡大set一覧: Vec<vk::DescriptorSet>,
}

/// トーンマップパス(判断38・39)1フレームぶんの入力。ポストプロセス有効時のみ`Some`で渡す。
pub(crate) struct トーンマップ描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    /// トーンマップ前にHDR輝度へ掛ける露出倍率(プッシュ定数で渡す)。
    pub(crate) 露出: f32,
}
