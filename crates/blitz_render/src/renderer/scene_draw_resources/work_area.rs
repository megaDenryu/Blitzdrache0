//! 毎フレームの描画入力作業領域の充填と受け渡し。チャンク一覧を先頭から走査し、描画対象1つにつき非空のLOD段ごとに、そのパスへ描く個体が1体でもあればそのパスの描画入力を1件積む。
//! 作業領域はシーン1本と距離区分ごとに1本ずつであり、距離区分別の可視個体の選別では距離区分ごとに描く個体が違うため1本を4距離区分で共有できない。
//! 注意: 積む前に必ず空にする。各パスの入力の件数は、全チャンクの描画対象の非空段数の合計からそのパスの描画数が0の段を除いた数であり、パスごとに違う値になる。
//! ディスクリプタセットはチャンク自身が自分の添字で参照して返すため、この走査はチャンクをまたいだ通し添字をセットの選択には使わない。1つの描画対象ぶんの組み立ては`object_entry`、1つの段ぶんの発行は`stage_issue`、数え方は`tally`、積み終えたシーン発行の並べ替えは`sort`、充填し終えた作業領域の読み出しは`readout`にある。

mod object_entry;
mod readout;
mod sort;
mod stage_issue;
mod tally;

use ash::vk;
use blitz_math::大域ワールド位置;

use super::シーン描画資源;
use crate::cascade::距離区分数;
use crate::error::{フレーム入力不一致エラー, レンダラーエラー};
use crate::frame_input::プリミティブ発行受け皿;
use crate::terrain_detail::{地形詳細段選択, 段を参照する};
use crate::visible_instance_selection::可視個体選択一覧;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力};
use crate::vulkan::material_table::{材質資源表, 資源表世代の束縛};
use crate::vulkan::pipeline_ledger::材質描画族パイプライン台帳;
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
    /// 発行ごとにパイプラインキーを引くために要る。材質を読む描画族のパイプラインはこの台帳だけが持つ。
    pub(in crate::renderer) パイプライン台帳: &'a 材質描画族パイプライン台帳,
    pub(in crate::renderer) カメラ大域原点: 大域ワールド位置,
    /// 束ごとの詳細段。束の中の全描画対象へ同じ段を配る。個体別LODの選択を持つ対象はこの段を使わない。
    pub(in crate::renderer) 地形詳細段選択一覧: &'a [地形詳細段選択],
    /// そのフレームに描く個体の並びと段の切り分け。選択を持たない対象は全個体を束の段で描く。
    pub(in crate::renderer) 可視個体選択一覧: 可視個体選択一覧<'a>,
    /// 描画対象ごとに、その対象が描くプリミティブの並び。段の選択が選んだ詳細段のプリミティブだけが発行になる。
    pub(in crate::renderer) プリミティブ発行: &'a プリミティブ発行受け皿,
    /// 材質スロット番号から解決した材質IDを、そのフレームが束縛する世代のレコード添字へ写すために要る。
    pub(in crate::renderer) 材質資源表: &'a 材質資源表,
    /// そのフレームが束縛した資源表世代。解決した材質GPU参照がこの世代のものであることを確かめる材料である。
    pub(in crate::renderer) 資源表世代の束縛: 資源表世代の束縛,
}

/// 積み先をまとめて渡す受け皿。すべてが常に同じフレームの同じ走査の結果であることをこの型が示す。
pub(super) struct 描画発行受け皿<'a> {
    pub(super) ジオメトリ: &'a mut Vec<ジオメトリ入力>,
    pub(super) 距離区分別のシャドウ: &'a mut [Vec<シャドウ描画入力>; 距離区分数],
    pub(super) 集計: &'a mut 描画計数集計,
}

impl シーン描画資源 {
    pub(in crate::renderer) fn 作業領域を更新する(&mut self, 入力: &作業領域更新入力<'_>) -> Result<(), レンダラーエラー> {
        self.ジオメトリ入力作業領域.clear();
        for 列 in &mut self.距離区分別のシャドウ入力作業領域 {
            列.clear();
        }
        self.計数集計.集計を始める();
        let mut 通し添字 = 0usize;
        for チャンク in &self.チャンク一覧 {
            let 段 = 段を参照する(入力.地形詳細段選択一覧, チャンク.id());
            for (対象添字, 資源, ディスクリプタ選択) in チャンク.描画対象と対応セット(入力.フレーム添字) {
                let プリミティブ発行 = 入力.プリミティブ発行.引く(チャンク.id(), 対象添字).ok_or(
                    フレーム入力不一致エラー::プリミティブ発行不在 {
                        束id: チャンク.id(),
                        描画対象添字: 対象添字,
                    },
                )?;
                let mut 受け皿 = 描画発行受け皿 {
                    ジオメトリ: &mut self.ジオメトリ入力作業領域,
                    距離区分別のシャドウ: &mut self.距離区分別のシャドウ入力作業領域,
                    集計: &mut self.計数集計,
                };
                object_entry::積む(
                    &object_entry::描画対象の材料 {
                        入力,
                        束id: チャンク.id(),
                        対象添字,
                        資源,
                        ディスクリプタ選択,
                        プリミティブ発行,
                        段,
                        先頭対象か: 通し添字 == 0,
                    },
                    &mut 受け皿,
                )?;
                通し添字 += 1;
            }
        }
        self.シーン発行を整列する();
        Ok(())
    }
}
