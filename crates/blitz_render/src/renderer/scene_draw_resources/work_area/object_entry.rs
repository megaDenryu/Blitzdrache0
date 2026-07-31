//! 描画対象1つぶんの描画入力を積む工程。受け取るのは対象の資源とそのフレームの外部条件、積むのは
//! 非空のLOD段ごとのシーンパス入力と距離区分ごとのシャドウ描画入力である。
//! 可視ID列のGPUへの書き込みもここで行う。書き込む内容と各パスの描くインスタンス数は同じ選択から出るため、離すと食い違いうる。
//! 注意: 距離区分ごとのシャドウ描画入力のインスタンス数はその距離区分の判定が決めた区間であり、シーン側や他の距離区分の可視数を
//! 流用してはならない(参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「可視ID列の距離区分別の区間(4区分並べの後継)」)。

use ash::vk;

use super::stage_issue;
use super::{作業領域更新入力, 描画発行受け皿};
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::frame_input::プリミティブ描画発行;
use crate::renderer::scene_draw_resources::chunk_draw_resources::対象のディスクリプタ選択;
use crate::renderer::scene_draw_resources::render_object_resources::描画対象資源;
use crate::terrain_detail::地形詳細段;
use crate::visible_instance_selection::段別描画範囲;
use crate::vulkan::relative_anchor::カメラ相対アンカー;

pub(super) struct 描画対象の材料<'a> {
    pub(super) 入力: &'a 作業領域更新入力<'a>,
    pub(super) 束id: 描画束ID,
    pub(super) 対象添字: usize,
    pub(super) 資源: &'a 描画対象資源,
    pub(super) ディスクリプタ選択: 対象のディスクリプタ選択<'a>,
    /// この描画対象が全詳細段にわたって描くプリミティブの並び。段ごとの絞り込みは発行を積む工程が行う。
    pub(super) プリミティブ発行: &'a [プリミティブ描画発行],
    pub(super) 段: 地形詳細段,
    /// スキニングはシーンの先頭対象だけへ適用する(判断44の既存契約)。可視判定で先頭が描かれなくなっても、
    /// 差し替え先の判断は走査順の先頭で決まる。
    pub(super) 先頭対象か: bool,
}

impl 描画対象の材料<'_> {
    pub(super) fn 相対アンカーを作る(&self) -> Result<カメラ相対アンカー, レンダラーエラー> {
        let 相対位置 = self.資源.大域アンカー.カメラ相対へ変換する(self.入力.カメラ大域原点)?;
        Ok(カメラ相対アンカー::相対位置から生成する(相対位置))
    }

    pub(super) fn 頂点バッファ差し替え先(&self) -> Option<vk::Buffer> {
        if self.先頭対象か {
            self.入力.スキン済み頂点バッファ
        } else {
            None
        }
    }
}

pub(super) fn 積む(材料: &描画対象の材料<'_>, 受け皿: &mut 描画発行受け皿<'_>) -> Result<(), レンダラーエラー> {
    let 相対アンカー = 材料.相対アンカーを作る()?;
    受け皿.集計.対象の個体数を加える(u64::from(材料.資源.個体数()));
    let Some(計画) = 材料.入力.可視個体選択一覧.引く(材料.束id, 材料.対象添字) else {
        return 全個体を束の段で積む(材料, 受け皿, 相対アンカー);
    };
    材料.資源.可視id列を書き込む(材料.入力.device, 材料.入力.フレーム添字, 計画)?;
    for (段番号, 範囲) in 計画.段範囲一覧.iter().enumerate() {
        stage_issue::積む(材料, 受け皿, 段番号, *範囲, 相対アンカー)?;
    }
    Ok(())
}

/// 可視判定と段選択の対象でない描画対象。束の詳細段で全個体を1回だけ描き、可視ID列は生成時の恒等の列をそのまま読む。
fn 全個体を束の段で積む(
    材料: &描画対象の材料<'_>,
    受け皿: &mut 描画発行受け皿<'_>,
    相対アンカー: カメラ相対アンカー,
) -> Result<(), レンダラーエラー> {
    let 範囲 = 段別描画範囲::全個体を描く(材料.資源.個体数());
    stage_issue::積む(材料, 受け皿, 材料.段.添字(), 範囲, 相対アンカー)
}
