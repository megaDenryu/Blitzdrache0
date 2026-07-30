//! 空段階をレンダーグラフのパスへ展開する。担当する工程は「方式に応じて空パスの読み画像を決め、パスを1本積む」ことである。
//! 受け取るのはグラフと空描画入力と大気LUTの読み先、返り値は無い。
//!
//! 読み画像が方式で変わるのは、大気LUT腕だけがフラグメント段でLUTを引くためである。用途に
//! `大気LUTフラグメント読み`を使うのは、大気LUTが休むレイアウトをGENERALに保つ不変条件があるためであり、
//! 通常の`シェーダー読みフラグメント`を使ってはならない(参照: `graph/initial_state/atmosphere_lut.rs`)。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use super::atmosphere_lut_stage::大気LUT読みハンドル;
use crate::vulkan::frame::record::sky_pass;
use crate::vulkan::frame::{空描画の方式, 空描画入力};
use crate::vulkan::graph;

/// 空パスが大気LUTを引くか。引くフレームは、そのフレームで焼かなくてもLUT画像をグラフへ登録する必要がある。
pub(in crate::vulkan::frame::record::graph_build) fn 空パスがlutを引くか(入力: Option<&空描画入力>) -> bool {
    入力.is_some_and(|入力| matches!(入力.方式, 空描画の方式::大気LUT { .. }))
}

pub(in crate::vulkan::frame::record::graph_build) fn 空を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    カラー: graph::画像ハンドル,
    入力: Option<&'a 空描画入力>,
    読みハンドル: Option<大気LUT読みハンドル>,
    寸法: vk::Extent2D,
) {
    if let Some(入力) = 入力 {
        let 読み画像 = 読み画像を決める(入力, 読みハンドル);
        グラフ.パスを積む(sky_pass::作る(カラー, 基本.深度, 読み画像, 入力, 寸法));
    }
}

/// 大気LUT腕は2枚のLUTを読み、Hosek腕は1枚も読まない。
///
/// 大気LUT腕でハンドルが無いのは、空段階があるのに大気LUT資源が無いという到達しない状態である
/// (どちらもフレーム構成の空段階の有無だけで決まり、レンダラーの入力検査が食い違いを先に落とす)。
fn 読み画像を決める(
    入力: &空描画入力, 読みハンドル: Option<大気LUT読みハンドル>
) -> Vec<(graph::画像ハンドル, graph::画像用途)> {
    match 入力.方式 {
        空描画の方式::Hosek解析近似 => Vec::new(),
        空描画の方式::大気LUT { .. } => {
            let ハンドル = 読みハンドル.unwrap_or_else(|| panic!("大気LUT腕の空パスがあるのに大気LUT画像がグラフへ登録されていない"));
            vec![
                (ハンドル.透過率, graph::画像用途::大気LUTフラグメント読み),
                (ハンドル.スカイビュー, graph::画像用途::大気LUTフラグメント読み),
            ]
        }
    }
}
