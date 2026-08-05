//! 局所可視性の資源を、フレーム記録が読む1フレーム入力へ平坦化する。
//! 呼び出しタイミング: 毎フレームの描画入力組み立て時(生成完了後・破棄前であることは保持側の`レンダラー`が保証する)。

use ash::vk;

use super::setting::即時定数を組み立てる;
use super::局所可視性一式;
use crate::local_visibility::射影の復元;

/// 遮蔽の標本化とぼかしのパスが要るハンドルと即時定数。拡散間接方式が環境のみの世界ではパスを積まないため使われない。
#[derive(Clone)]
pub(crate) struct 局所可視性描画入力 {
    pub(crate) 生の画像: vk::Image,
    pub(crate) 生のビュー: vk::ImageView,
    pub(crate) ぼかし後の画像: vk::Image,
    pub(crate) ぼかし後の画像ビュー: vk::ImageView,
    pub(crate) 遮蔽の標本化pipeline: vk::Pipeline,
    pub(crate) 両側ぼかしpipeline: vk::Pipeline,
    /// 2本が共有する1つのレイアウト。
    pub(crate) layout: vk::PipelineLayout,
    /// 2本が共有する1つのセット。どちらがどの束縛を触るかはレンダーグラフのパス宣言が持つ。
    pub(crate) セット: vk::DescriptorSet,
    /// 2本が同じ並びで押し込む定数。順は`shaders/local_visibility_setting.slang`の`LocalVisibilitySetting`と一致させる。
    pub(crate) 即時定数: Vec<u8>,
}

impl 局所可視性一式 {
    pub(crate) fn 描画入力を作る(&self, 射影: 射影の復元, 寸法: vk::Extent2D) -> 局所可視性描画入力 {
        局所可視性描画入力 {
            生の画像: self.画像組.生.画像,
            生のビュー: self.画像組.生.画像ビュー,
            ぼかし後の画像: self.画像組.ぼかし後.画像,
            ぼかし後の画像ビュー: self.画像組.ぼかし後.画像ビュー,
            遮蔽の標本化pipeline: self.パイプライン.遮蔽の標本化,
            両側ぼかしpipeline: self.パイプライン.両側ぼかし,
            layout: self.パイプライン.レイアウト,
            セット: self.ディスクリプタ.セット,
            即時定数: 即時定数を組み立てる(射影, 寸法, self.設定),
        }
    }

    /// 消費側のセット(set3)が結ぶ、ぼかし後の画像のビュー。
    pub(crate) fn ぼかし後のビュー(&self) -> vk::ImageView {
        self.画像組.ぼかし後.画像ビュー
    }
}
