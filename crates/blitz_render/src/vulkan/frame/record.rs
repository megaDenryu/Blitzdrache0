//! コマンド記録の局面: レンダーグラフを構築して実行するだけにする。バリア発行・
//! begin/end renderingはグラフ実行器に集約する（参照: `_doc/設計/レンダーグラフ.md`、
//! `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`判断27〜28）。グラフの
//! 組み立て自体は`graph_build`に委ねる。
//!
//! セッション型のうち「積む」局面だけをここが持つ。積み始めは`session`、閉じて送信し提示する局面は`submit_present`にある。

mod aerial_composite_pass;
mod auto_exposure_passes;
mod bloom_down_pass;
mod bloom_up_pass;
mod cloth_passes;
mod cluster_light_assignment_pass;
mod depth_prepass_pass;
mod fullscreen_draw;
mod graph_build;
mod local_visibility_passes;
mod particle_draw_pass;
mod particle_update_pass;
pub(crate) mod pass_names;
mod point_light_shadow_draw;
pub(crate) mod point_light_shadow_pass;
mod readback_pass;
mod scene_pass;
mod shadow_draw;
mod shadow_pass;
mod skinning_pass;
mod sky_pass;
mod temporal_reconstruction_pass;
mod tonemap_pass;
mod ui_pass;

use ash::vk;

use super::session::フレームのGPU命令を積むコマンドバッファ;
use super::{フレーム画像一式, 任意描画入力, 描画対象入力, 描画方式};
use crate::clear_color::クリアカラー;
use crate::frame_composition::フレーム構成;
use crate::vulkan::graph;

/// 1フレームぶんの記録が読む材料の束。1つの束にするのは、この7つがどれもそのフレームのグラフの形を決める材料であり、
/// 別々に受け取ると記録の署名と、グラフを組む工程の署名の両方が同じ並びで長くなるためである。
#[derive(Clone, Copy)]
pub(crate) struct フレームの記録の材料<'材料> {
    pub(crate) フレーム構成: &'材料 フレーム構成,
    pub(crate) 画像一式: &'材料 フレーム画像一式<'材料>,
    pub(crate) 寸法: vk::Extent2D,
    pub(crate) クリア色: クリアカラー,
    pub(crate) 描画対象: 描画対象入力<'材料>,
    pub(crate) 任意入力: 任意描画入力<'材料>,
    pub(crate) 描画方式: &'材料 描画方式,
}

/// 1フレームぶんの記録の実績。GPU計測のマッピングと、計器が数える大気のベイク済み画像生成パスの本数を返す。
pub(crate) struct 記録の実績 {
    pub(crate) 計測マッピング: Vec<(&'static str, u32)>,
    pub(crate) 大気のベイク済み画像生成パス数: u32,
    pub(crate) 間接照明生成パス数: crate::distant_environment::間接照明生成パス数,
}

impl フレームのGPU命令を積むコマンドバッファ<'_> {
    /// そのフレームのグラフを組み、パス列を宣言順に積む。閉じるのは`記録を閉じて送信し提示する`である。
    pub(crate) fn フレームのgpu命令を積む(&self, 材料: フレームの記録の材料<'_>) -> 記録の実績 {
        let 構築 = graph_build::グラフを構築する(材料);
        let 計測マッピング = graph::グラフ実行器::生成する(self.積み先(), self.計測のクエリプール()).グラフを積む(構築.グラフ);
        記録の実績 {
            計測マッピング,
            大気のベイク済み画像生成パス数: 構築.大気のベイク済み画像生成パス数,
            間接照明生成パス数: 構築.間接照明生成パス数,
        }
    }
}
