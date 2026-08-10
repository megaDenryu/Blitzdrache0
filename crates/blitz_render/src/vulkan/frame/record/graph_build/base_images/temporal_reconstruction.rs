//! 時間再構成のパスが読み書きする3枚(今のフレームの色・履歴の読み側と書き側)の登録と、そのハンドルの組。
//! 触れるのはこの3枚だけであり、毎フレーム必ず在る画像には触れない。
//!
//! 分けて持つのは、この3枚が積むフレームだけ在る資源だからである。積まない世界では1枚も登録せず、
//! バリアも1つも増えない(参照: `_doc/設計/時間再構成.md`「判断d」)。

use ash::vk;

use crate::vulkan::graph;
use crate::vulkan::temporal_reconstruction::時間再構成描画入力;

/// 時間再構成のパスが読み書きする3枚のハンドル。
pub(in crate::vulkan::frame::record::graph_build) struct 時間再構成の登録 {
    /// シーン・空・粒子の描画先。時間再構成のパスがこれを読む。
    pub(in crate::vulkan::frame::record::graph_build) 今のフレームの色: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) 履歴読み: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) 履歴書き: graph::画像ハンドル,
}

pub(super) fn 登録する(
    グラフ: &mut graph::グラフ<'_>, 入力: &時間再構成描画入力, 寸法: vk::Extent2D
) -> 時間再構成の登録 {
    let 色 = graph::前フレーム今のフレームの色読み直後状態();
    let 履歴 = graph::履歴画像の前フレーム直後状態();
    時間再構成の登録 {
        今のフレームの色: 登録(グラフ, 入力.今のフレームの色の画像, 入力.今のフレームの色のビュー, 色, 寸法),
        履歴読み: 登録(グラフ, 入力.履歴読みの画像, 入力.履歴読みのビュー, 履歴, 寸法),
        履歴書き: 登録(グラフ, 入力.履歴書きの画像, 入力.履歴書きのビュー, 履歴, 寸法),
    }
}

fn 登録(
    グラフ: &mut graph::グラフ<'_>,
    画像: vk::Image,
    ビュー: vk::ImageView,
    初期状態: graph::画像状態,
    寸法: vk::Extent2D,
) -> graph::画像ハンドル {
    グラフ.画像を登録する(画像, ビュー, graph::画像アスペクト::カラー, 初期状態, 寸法)
}
