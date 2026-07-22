//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。共通画像の登録は`base_images`、
//! ポストプロセス画像の登録は`post_setup`へ分離。

mod base_images;
mod post_passes;
mod post_setup;
mod stages;

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{
    UI描画入力, シャドウ描画入力, ジオメトリ入力, スキニング描画入力, トーンマップ描画入力, フレーム画像一式, ブルーム描画入力, 布描画入力, 描画方式,
    粒子描画入力,
};
use crate::vulkan::graph;

#[allow(clippy::too_many_arguments)]
pub(super) fn グラフを構築する<'a>(
    画像一式: &フレーム画像一式,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ一覧: &'a [ジオメトリ入力],
    シャドウ一覧: &'a [シャドウ描画入力],
    スキニング入力: Option<&'a スキニング描画入力>,
    布入力: Option<&'a 布描画入力>,
    粒子入力: Option<&'a 粒子描画入力>,
    ブルーム入力: Option<&'a ブルーム描画入力>,
    トーンマップ入力: Option<&'a トーンマップ描画入力>,
    ui入力: Option<&'a UI描画入力>,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規();
    let 基本 = base_images::登録する(&mut グラフ, 画像一式, 寸法);

    // シーン・粒子の描画先: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    let ポスト = post_setup::登録する(&mut グラフ, 画像一式, トーンマップ入力, ブルーム入力, 寸法);
    let シーンカラーハンドル = ポスト.as_ref().map_or(基本.スワップチェーン, |構成| 構成.hdrハンドル);

    let (スキン済み, 布ドロー) = stages::シミュレーションを積む(&mut グラフ, スキニング入力, 布入力);
    stages::シーンを積む(
        &mut グラフ,
        &基本,
        シーンカラーハンドル,
        スキン済み,
        布ドロー,
        クリア色,
        pipeline,
        ジオメトリ一覧,
        シャドウ一覧,
        寸法,
    );
    stages::後段を積む(
        &mut グラフ,
        &基本,
        ポスト.as_ref(),
        粒子入力,
        ui入力,
        描画方式,
        シーンカラーハンドル,
        寸法,
    );

    グラフ.最終用途を宣言する(基本.スワップチェーン, graph::画像用途::提示);
    グラフ
}
