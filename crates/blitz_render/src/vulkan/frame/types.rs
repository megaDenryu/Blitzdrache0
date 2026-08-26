//! 1フレームの描画で受け渡す型: 描画方式・ジオメトリ入力・粒子描画入力・スキニング描画入力。
//! 影・布・UI・ポスト処理は対応するサブシステムの入力だけを持つため`shadow_types`・`cloth_types`・`ui_types`・`post_types`が別に持つ。

use ash::vk;

use crate::vulkan::material_table::大域材質ID;
use crate::vulkan::pipeline_ledger::パイプラインキー;
use crate::vulkan::readback::読み戻し対象;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;
use crate::vulkan::scene_draw_constants::シーン描画定数;

/// このフレームの描画後処理: 通常の提示前遷移のみか、読み戻し用のコピーを挟むか。
/// `読み戻し`は読み戻しコピーを挟む枝であり、転送元にする画像は`対象`が決める。
pub(crate) enum 描画方式 {
    通常,
    読み戻し { バッファ: vk::Buffer, 対象: 読み戻し対象 },
}

/// 送信の後にホストが読み戻しの完了を待つかどうか。読み戻しコピーを挟むフレームだけが待つ。
#[derive(Clone, Copy)]
pub(crate) enum 読み戻しの待機 {
    待つ,
    待たない,
}

impl 描画方式 {
    pub(crate) fn 読み戻しの待機(&self) -> 読み戻しの待機 {
        match self {
            描画方式::通常 => 読み戻しの待機::待たない,
            描画方式::読み戻し { .. } => 読み戻しの待機::待つ,
        }
    }
}
/// 頂点/インデックスバッファと、この発行が束縛するジオメトリのセット(set1)。
/// ビューとパスのセット(set0)・材質のセット(set2)・照明問い合わせのセット(set3)は発行ごとに変わらないため、
/// パスの先頭で1回だけ束縛する`共有セット束縛`が持つ。パイプラインのlayoutはセットの送信先を指定するために必要。
///
/// `pipeline`は発行ごとに持つ。材質変種でパイプラインが変わりうるためであり、直前の発行とキーが同じなら記録側が束縛を省く。
/// `深度プリパスpipeline`は同じ発行を深度プリパスで描くときのパイプラインであり、色パスと同じ頂点段のSPIR-Vから作った深度だけの実体で材質変種が同じなら同じものになる。深度プリパスを積まない方式でも値が入るのは、積むかどうかがフレームの組み立て方の選択であり、発行の内容が持つ性質ではないためである。
/// `パイプラインキー`は束縛したパイプラインを選んだキーであり、並べ替えの第1鍵で記録側が切替の要否を判断する材料でもある。
/// `先頭インデックス`はこの発行が描き始めるインデックスの位置であり、プリミティブごとに発行を分けるためメッシュ全体の先頭とは限らない。
/// `頂点基準`はインデックスの値へ足す頂点番号であり、プリミティブが自分の頂点範囲を持つ形式のために運ぶ。
/// `インスタンス数`はこの描画発行で描く個体の数であり、通常メッシュと地形は1、インスタンス群はそのLOD段の可視数である。
/// `先頭インスタンス`は可視ID列のうちこの発行が読み始める位置であり、頂点シェーダーは`SV_StartInstanceLocation`でこの値を受け取り`SV_InstanceID`に足して可視ID列を参照する。段ごとに1回発行するため、段の範囲の開始がそのままこの値になる。
/// `描画定数`はこの発行のカメラ相対の基準原点と、塗る材質のレコード添字であり、プッシュ定数で頂点ステージと画素段ステージへ渡す。
/// `大域材質id`はこの発行が塗る材質の論理的な識別であり、並べ替えの第2鍵で材質切替を数えるときの比較対象でもある。世代内のレコード添字を鍵にしないのは、添字が世代を作り直すたびに並び直るためである。
pub(crate) struct ジオメトリ入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) 深度プリパスpipeline: vk::Pipeline,
    pub(crate) パイプラインキー: パイプラインキー,
    pub(crate) 頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) 先頭インデックス: u32,
    pub(crate) インデックス数: u32,
    pub(crate) 頂点基準: i32,
    pub(crate) インスタンス数: u32,
    pub(crate) 先頭インスタンス: u32,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ジオメトリセット: vk::DescriptorSet,
    pub(crate) 描画定数: シーン描画定数,
    pub(crate) 大域材質id: 大域材質ID,
}

/// GPU粒子トイ(判断29)1フレームぶんの入力。`--particles`指定時のみ`Some`で渡す。
/// 呼び出し元(renderer層)がフレーム添字に対応するディスクリプタセットを
/// あらかじめ選んで渡す(`ジオメトリ入力`と同じ設計)。
/// `相対の基準原点`は粒子の位置が世界原点を基準に計算されるため、世界原点のカメラ相対値になる。
pub(crate) struct 粒子描画入力 {
    pub(crate) コンピュートパイプライン: vk::Pipeline,
    pub(crate) コンピュートlayout: vk::PipelineLayout,
    pub(crate) 描画パイプライン: vk::Pipeline,
    pub(crate) 描画layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) バッファ: vk::Buffer,
    pub(crate) 更新スレッド数: u32,
    pub(crate) 描画要素数: u32,
    pub(crate) 相対の基準原点: カメラ相対の基準原点,
}

/// GPUスキニング(判断44)1フレームぶんの入力。スキン付きシーンのときのみ`Some`で渡す。
/// `出力バッファ`はスキン済み頂点バッファであり、グラフ登録とシーン/シャドウの頂点入力差し替えに使う。
pub(crate) struct スキニング描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 頂点数: u32,
    pub(crate) 出力バッファ: vk::Buffer,
}
