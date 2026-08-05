//! シーン段階を展開する工程。深度プリパスが書いた深度を読む局所可視性補正の2本を先に積み、続いて本体のシーン描画を1本積む。
//!
//! 2つを1つの工程が持つのは、局所可視性補正の書き込みとシーン描画の画素段の読みが必ず対になるためである。
//! 積み込みと読み宣言が離れていると、呼び出し側が順序を誤ってもその場では何も起きず、絵にだけ食い違いが出る。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use super::local_visibility_stage;
use crate::clear_color::クリアカラー;
use crate::frame_composition::深度プリパス方式;
use crate::vulkan::frame::record::scene_pass;
use crate::vulkan::frame::{ジオメトリ入力, 共有セット束縛};
use crate::vulkan::graph;
use crate::vulkan::graph::クリア指定;
use crate::vulkan::local_visibility::局所可視性描画入力;

use super::indirect_lighting_stage::遠方環境の消費画像;

#[allow(clippy::too_many_arguments)]
pub(in crate::vulkan::frame::record::graph_build) fn シーンを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    カラー: graph::画像ハンドル,
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    クリア色: クリアカラー,
    方式: 深度プリパス方式,
    入力: &'a [ジオメトリ入力],
    共有: 共有セット束縛<'a>,
    遠方環境: Option<遠方環境の消費画像>,
    局所可視性: Option<&'a 局所可視性描画入力>,
    寸法: vk::Extent2D,
) {
    // 局所可視性補正は深度プリパスとシーン描画の間へ置く。前へ置くと消去値のままの深度を読み、後ろへ置くとそのフレームのシーン描画が前フレームの可視度を読む。
    // 積み込みと読み宣言をこの1箇所へ閉じるのは、2つを離すと呼び出し側が順序を誤れるためである。
    let 局所可視度 = local_visibility_stage::局所可視性を積む(グラフ, 基本.深度, 局所可視性, 寸法);
    let mut 読み画像一覧 = vec![(基本.シャドウマップ, graph::画像用途::深度シェーダー読み)];
    if let Some(遠方環境) = 遠方環境 {
        読み画像一覧.extend(遠方環境.読み宣言());
    }
    // 局所可視性補正を積まないフレームは、その画像を書くパスが1本も無いためハンドルも無い。読み宣言も立てない。
    if let Some(局所可視度) = 局所可視度 {
        読み画像一覧.push((局所可視度, graph::画像用途::焼いた画像の画素段参照));
    }
    // 深度プリパスが書いた深度を消去せずに読み込む。両方が消去すると、等値の比較で1画素も描かれなくなる。
    let クリア = if 方式.深度プリパスを積むか() {
        クリア指定::カラーだけを消去して深度は読み込む { カラー: クリア色 }
    } else {
        クリア指定::クリアする { カラー: クリア色 }
    };
    グラフ.パスを積む(scene_pass::作る(
        カラー,
        基本.深度,
        読み画像一覧,
        スキン済み,
        布,
        クリア,
        入力,
        共有,
        寸法,
    ));
}
