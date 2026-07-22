//! engineが宣言したフレーム段階列を順に展開する。

use ash::vk;

use super::{base_images, post_passes, post_setup, stages};
use crate::clear_color::クリアカラー;
use crate::frame_composition::{フレーム構成, フレーム段階};
use crate::vulkan::frame::record::{readback_pass, ui_pass};
use crate::vulkan::frame::{任意描画入力, 描画対象入力, 描画方式};
use crate::vulkan::graph;

#[allow(clippy::too_many_arguments)]
pub(super) fn 積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    構成: &フレーム構成,
    基本: &base_images::基本画像ハンドル,
    ポスト: Option<&post_setup::ポスト構成<'a>>,
    カラー: graph::画像ハンドル,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    描画対象: 描画対象入力<'a>,
    任意: 任意描画入力<'a>,
    描画方式: &'a 描画方式,
    寸法: vk::Extent2D,
) {
    let mut スキン済み = None;
    let mut 布 = None;
    for 段階 in 構成.段階一覧() {
        match 段階 {
            フレーム段階::スキニング => スキン済み = stages::スキニングを積む(グラフ, 任意.スキニング),
            フレーム段階::布シミュレーション => 布 = stages::布を積む(グラフ, 任意.布, スキン済み),
            フレーム段階::影 => stages::影を積む(グラフ, 基本, スキン済み, 布, 描画対象.シャドウ),
            フレーム段階::シーン => {
                stages::シーンを積む(グラフ, 基本, カラー, スキン済み, 布, クリア色, pipeline, 描画対象.ジオメトリ, 寸法);
            }
            フレーム段階::粒子 => stages::粒子を積む(グラフ, 基本, カラー, 任意.粒子, 寸法),
            フレーム段階::ブルームとトーンマップ => {
                let ポスト = ポスト.unwrap_or_else(|| panic!("フレーム構成にポスト処理があるが資源が無い"));
                post_passes::積む(グラフ, ポスト, 基本.スワップチェーン, 寸法);
            }
            フレーム段階::UI => {
                if let Some(入力) = 任意.ui {
                    グラフ.パスを積む(ui_pass::作る(基本.スワップチェーン, 入力, 寸法));
                }
            }
            フレーム段階::読み戻し => {
                if let 描画方式::読み戻し { バッファ } = 描画方式 {
                    グラフ.パスを積む(readback_pass::作る(基本.スワップチェーン, *バッファ, 寸法));
                }
            }
        }
    }
}
