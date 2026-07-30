//! 毎フレームの描画入力作業領域の充填と受け渡し。チャンク一覧を先頭から走査し、描画対象1つにつき
//! 非空のLOD段ごとに、そのパスへ描く個体が1体でもあればそのパスの描画入力を1件積む。
//! 注意: 積む前に必ず空にする。2つのパスの入力の件数は、全チャンクの描画対象の非空段数の合計から
//! そのパスの描画数が0の段を除いた数であり、パスごとに違う値になる。
//! ディスクリプタセットはチャンク自身が自分の添字で参照して返すため、この走査はチャンクをまたいだ通し添字をセットの選択には使わない。
//! 1つの描画対象ぶんの組み立ては`object_entry`、1つの段ぶんの発行は`stage_issue`、数え方は`tally`にある。

mod object_entry;
mod stage_issue;
mod tally;

use ash::vk;
use blitz_math::大域ワールド位置;

use super::シーン描画資源;
use crate::error::レンダラーエラー;
use crate::renderer::draw_issue_breakdown::描画発行内訳;
use crate::terrain_detail::{地形詳細段選択, 段を参照する};
use crate::visible_instance_selection::可視個体選択一覧;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力, 描画対象入力};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::renderer) use tally::描画計数集計;

/// 作業領域の中身のうち、描画対象資源の外から与える値。パイプラインは束の外(レンダラー)が保持するためここで受け取る。
pub(in crate::renderer) struct 作業領域更新入力<'a> {
    /// 可視ID列をそのフレームのバッファへ書き込むために要る。作業領域の充填と同じ走査で書くため、この入力が運ぶ。
    pub(in crate::renderer) device: &'a GPUデバイス,
    pub(in crate::renderer) フレーム添字: フレームスロット添字,
    /// スキン付きシーンでの先頭描画対象の頂点バッファ差し替え先(判断44の既存契約)。スキン無しなら`None`。
    pub(in crate::renderer) スキン済み頂点バッファ: Option<vk::Buffer>,
    pub(in crate::renderer) シーンlayout: vk::PipelineLayout,
    pub(in crate::renderer) シャドウpipeline: vk::Pipeline,
    pub(in crate::renderer) シャドウlayout: vk::PipelineLayout,
    pub(in crate::renderer) カメラ大域原点: 大域ワールド位置,
    /// 束ごとの詳細段。束の中の全描画対象へ同じ段を配る。個体別LODの選択を持つ対象はこの段を使わない。
    pub(in crate::renderer) 地形詳細段選択一覧: &'a [地形詳細段選択],
    /// そのフレームに描く個体の並びと段の切り分け。選択を持たない対象は全個体を束の段で描く。
    pub(in crate::renderer) 可視個体選択一覧: 可視個体選択一覧<'a>,
}

/// 積み先の3つをまとめて渡す受け皿。3つが常に同じフレームの同じ走査の結果であることをこの型が示す。
pub(super) struct 描画発行受け皿<'a> {
    pub(super) ジオメトリ: &'a mut Vec<ジオメトリ入力>,
    pub(super) シャドウ: &'a mut Vec<シャドウ描画入力>,
    pub(super) 集計: &'a mut 描画計数集計,
}

impl シーン描画資源 {
    pub(in crate::renderer) fn 作業領域を更新する(&mut self, 入力: &作業領域更新入力<'_>) -> Result<(), レンダラーエラー> {
        self.ジオメトリ入力作業領域.clear();
        self.シャドウ入力作業領域.clear();
        self.計数集計.集計を始める();
        let mut 通し添字 = 0usize;
        for チャンク in &self.チャンク一覧 {
            let 段 = 段を参照する(入力.地形詳細段選択一覧, チャンク.id());
            for (対象添字, 資源, ディスクリプタセット) in チャンク.描画対象と対応セット(入力.フレーム添字) {
                let mut 受け皿 = 描画発行受け皿 {
                    ジオメトリ: &mut self.ジオメトリ入力作業領域,
                    シャドウ: &mut self.シャドウ入力作業領域,
                    集計: &mut self.計数集計,
                };
                object_entry::積む(
                    &object_entry::描画対象の材料 {
                        入力,
                        束id: チャンク.id(),
                        対象添字,
                        資源,
                        ディスクリプタセット,
                        段,
                        先頭対象か: 通し添字 == 0,
                    },
                    &mut 受け皿,
                )?;
                通し添字 += 1;
            }
        }
        Ok(())
    }

    /// 直近のフレームで積んだ内訳。`作業領域を更新する`が最後に数えた値から作る。
    pub(in crate::renderer) fn 描画発行内訳を作る(&self) -> 描画発行内訳 {
        self.計数集計
            .内訳を作る(self.ジオメトリ入力作業領域.len(), self.シャドウ入力作業領域.len())
    }

    /// 前提: 同じフレームで`作業領域を更新する`を済ませてから呼ぶ(中身は前フレームの残りではなく今フレームの内容である必要がある)。
    pub(in crate::renderer) fn 描画対象入力を作る(&self) -> 描画対象入力<'_> {
        描画対象入力 {
            ジオメトリ: &self.ジオメトリ入力作業領域,
            シャドウ: &self.シャドウ入力作業領域,
        }
    }
}
