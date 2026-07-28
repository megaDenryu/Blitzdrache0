//! 毎フレームの描画入力作業領域の充填と受け渡し。チャンク一覧を先頭から走査し、描画対象1つにつきシャドウ描画入力を1件、
//! シーンパスへ描く個体が1体でもあればジオメトリ入力を1件積む。
//! 注意: 積む前に必ず空にする。シャドウ入力の件数は常に全チャンクの描画対象数の合計と一致し、ジオメトリ入力は可視数0の対象だけ欠ける。
//! ディスクリプタセットはチャンク自身が自分の添字で引いて返すため、この走査はチャンクをまたいだ通し添字をセットの選択には使わない。
//! 1つの描画対象ぶんの組み立ては`object_entry`にある。

mod object_entry;

use ash::vk;
use blitz_math::大域ワールド位置;

use super::シーン描画資源;
use crate::error::レンダラーエラー;
use crate::renderer::draw_issue_breakdown::{パス別描画発行, 描画発行内訳};
use crate::terrain_detail::{地形詳細段選択, 段を引く};
use crate::visible_instance_selection::可視個体選択一覧;
use crate::vulkan::frame::描画対象入力;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

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
    /// 束ごとの詳細段。束の中の全描画対象へ同じ段を配る。
    pub(in crate::renderer) 地形詳細段選択一覧: &'a [地形詳細段選択],
    /// そのフレームにシーンパスが描く個体の選択。選択を持たない対象は全個体を描く。
    pub(in crate::renderer) 可視個体選択一覧: 可視個体選択一覧<'a>,
}

impl シーン描画資源 {
    pub(in crate::renderer) fn 作業領域を更新する(&mut self, 入力: &作業領域更新入力<'_>) -> Result<(), レンダラーエラー> {
        self.ジオメトリ入力作業領域.clear();
        self.シャドウ入力作業領域.clear();
        let mut 全個体数 = 0u64;
        let mut シーン可視数 = 0u64;
        let mut 通し添字 = 0usize;
        for チャンク in &self.チャンク一覧 {
            let 段 = 段を引く(入力.地形詳細段選択一覧, チャンク.id());
            for (対象添字, 資源, ディスクリプタセット) in チャンク.描画対象と対応セット(入力.フレーム添字) {
                let 対象 = object_entry::組み立てる(&object_entry::描画対象の材料 {
                    入力,
                    束id: チャンク.id(),
                    対象添字,
                    資源,
                    ディスクリプタセット,
                    段,
                    先頭対象か: 通し添字 == 0,
                })?;
                全個体数 += u64::from(資源.個体数());
                シーン可視数 += u64::from(対象.シーン可視数);
                if let Some(ジオメトリ) = 対象.ジオメトリ {
                    self.ジオメトリ入力作業領域.push(ジオメトリ);
                }
                self.シャドウ入力作業領域.push(対象.シャドウ);
                通し添字 += 1;
            }
        }
        self.直近の内訳 = 内訳を作る(self, 全個体数, シーン可視数);
        Ok(())
    }

    /// 直近のフレームで積んだ内訳。`作業領域を更新する`が最後に数えた値をそのまま返す。
    pub(in crate::renderer) fn 描画発行内訳を作る(&self) -> 描画発行内訳 {
        self.直近の内訳
    }

    /// 前提: 同じフレームで`作業領域を更新する`を済ませてから呼ぶ(中身は前フレームの残りではなく今フレームの内容である必要がある)。
    pub(in crate::renderer) fn 描画対象入力を作る(&self) -> 描画対象入力<'_> {
        描画対象入力 {
            ジオメトリ: &self.ジオメトリ入力作業領域,
            シャドウ: &self.シャドウ入力作業領域,
        }
    }
}

/// 個体数は積んだ描画発行のインスタンス数から数え直す。可視判定が出した可視数とは別の出どころであり、
/// 2つが食い違えば充填の側の誤りとして計器に現れる。
fn 内訳を作る(資源: &シーン描画資源, 全個体数: u64, シーン可視数: u64) -> 描画発行内訳 {
    let シーン個体数 = 資源.ジオメトリ入力作業領域.iter().map(|入力| u64::from(入力.インスタンス数)).sum();
    let シャドウ個体数 = 資源.シャドウ入力作業領域.iter().map(|入力| u64::from(入力.インスタンス数)).sum();
    描画発行内訳::生成する(
        パス別描画発行::生成する(資源.ジオメトリ入力作業領域.len(), 全個体数, シーン可視数, シーン個体数),
        パス別描画発行::生成する(資源.シャドウ入力作業領域.len(), 全個体数, 全個体数, シャドウ個体数),
    )
}
