//! 時間再構成の資源を、フレーム記録が読む1フレーム入力へ平坦化する。
//! 呼び出しタイミング: 毎フレームの描画入力組み立て時(生成完了後・破棄前であることは保持側の`レンダラー`が保証する)。
//!
//! 履歴の読み側と書き側をこの地点で1つに固定するのは、同じフレームの中でグラフの登録・パスの束縛・ディスクリプタの
//! 選択の3箇所が同じ添字を読まなければならないためである。3箇所が別々に状態へ問い合わせる形にすると、
//! 状態を進める地点が変わったときに片方だけが古い添字を読む。

use ash::vk;

use super::setting::即時定数を組み立てる;
use super::時間再構成一式;
use crate::local_visibility::射影の復元;

/// 時間再構成のパスが要るハンドルと即時定数。方式が使わないの世界と、ポスト処理を組まない構成では作らない。
#[derive(Clone)]
pub(crate) struct 時間再構成描画入力 {
    /// シーン・空・粒子がこのフレームの色を描く先。パスはこれを読み、HDR中間画像へ結果を書く。
    pub(crate) 今のフレームの色の画像: vk::Image,
    pub(crate) 今のフレームの色のビュー: vk::ImageView,
    /// 前のフレームの結果を持つ側。パスが標本器で参照する。
    pub(crate) 履歴読みの画像: vk::Image,
    pub(crate) 履歴読みのビュー: vk::ImageView,
    /// このフレームの結果を書き込む側。第2のカラー添付として束ねる。
    pub(crate) 履歴書きの画像: vk::Image,
    pub(crate) 履歴書きのビュー: vk::ImageView,
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    /// 履歴の読み側に対応するセット。
    pub(crate) セット: vk::DescriptorSet,
    /// 並びは`shaders/temporal_reconstruction_setting.slang`の`TemporalReconstructionSetting`と一致させる。
    pub(crate) 即時定数: Vec<u8>,
}

impl 時間再構成一式 {
    /// そのフレームの入力。方式が使わないの世界では`None`であり、パスを1本も積まない。
    pub(crate) fn 描画入力を作る(&self, 射影: 射影の復元, 寸法: vk::Extent2D) -> Option<時間再構成描画入力> {
        if !self.設定.方式.履歴画像を読むパスが積まれるか() {
            return None;
        }
        let 読み = &self.画像組.履歴[self.履歴の状態.読み添字()];
        let 書き = &self.画像組.履歴[self.履歴の状態.書き添字()];
        Some(時間再構成描画入力 {
            今のフレームの色の画像: self.画像組.今のフレームの色.画像,
            今のフレームの色のビュー: self.画像組.今のフレームの色.画像ビュー,
            履歴読みの画像: 読み.画像,
            履歴読みのビュー: 読み.画像ビュー,
            履歴書きの画像: 書き.画像,
            履歴書きのビュー: 書き.画像ビュー,
            pipeline: self.パイプライン.pipeline,
            layout: self.パイプライン.レイアウト,
            セット: self.ディスクリプタ.セット一覧[self.履歴の状態.読み添字()],
            即時定数: 即時定数を組み立てる(射影, 寸法, self.設定, self.履歴の状態.混ぜるか()),
        })
    }
}
