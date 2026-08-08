//! グラフィックスパスが書く色系アタッチメントの並び。担うのは、並びの取りうる形を型で数え上げることと、
//! その並びをdynamic renderingのアタッチメント記述へ展開することである。
//!
//! 添字つきの一般の列にせず枝で数え上げるのは、第2添字が何を意味するかがパスごとに変わらないためである。
//! このエンジンで第2のカラー添付を持つのはシーン描画と空の2本だけであり、そこへ書くのは常に動きベクトルである。
//! 一般の列にすると、書く側のシェーダーの出力番号と並びの添字が対応することを型の外で守る約束になる。
//!
//! 注意: 動きベクトルのクリア値は零である。零は「前のフレームでも同じ位置に見えていた」を表す規約上の値であり
//! (`blitz_engine::temporal_reconstruction::動きベクトル::動いていない`)、ジオメトリも空も書かなかった画素へ
//! この値が残ることは意味の上で正しい。

use ash::vk;

use super::handle::画像ハンドル;
use super::registry::画像レジストリ;

#[derive(Clone, Copy)]
pub(crate) enum カラー添付列 {
    /// 深度だけを書くパス(シャドウ・深度プリパス)。
    無し,
    /// 色を1枚だけ書くパス。
    色だけ(画像ハンドル),
    /// 色と動きベクトルを書くパス(シーン描画と空)。
    色と動きベクトル {
        色: 画像ハンドル, 動きベクトル: 画像ハンドル
    },
}

impl カラー添付列 {
    /// render_areaの基準に使う先頭のハンドル。1枚も無いパスでは深度が基準になる。
    pub(super) fn 先頭(self) -> Option<画像ハンドル> {
        match self {
            Self::無し => None,
            Self::色だけ(色) | Self::色と動きベクトル { 色, .. } => Some(色),
        }
    }

    pub(super) fn 一枚も無いか(self) -> bool {
        matches!(self, Self::無し)
    }

    /// dynamic renderingへ渡すアタッチメント記述の並び。ロード操作は色と動きベクトルで揃える。
    /// 色を消去するフレームは動きベクトルも消去し、色を読み込むフレームは動きベクトルも読み込む。
    /// 揃えないと、色を消去した画素に前のフレームの動きベクトルが残る。
    pub(super) fn 記述を並べる(
        self,
        レジストリ: &画像レジストリ,
        ロード操作: vk::AttachmentLoadOp,
        色のクリア値: vk::ClearValue,
    ) -> Vec<vk::RenderingAttachmentInfo<'static>> {
        let 動いていない値 = vk::ClearValue {
            color: vk::ClearColorValue { float32: [0.0; 4] },
        };
        match self {
            Self::無し => Vec::new(),
            Self::色だけ(色) => vec![記述を作る(レジストリ, 色, ロード操作, 色のクリア値)],
            Self::色と動きベクトル { 色, 動きベクトル } => vec![
                記述を作る(レジストリ, 色, ロード操作, 色のクリア値),
                記述を作る(レジストリ, 動きベクトル, ロード操作, 動いていない値),
            ],
        }
    }
}

fn 記述を作る(
    レジストリ: &画像レジストリ,
    ハンドル: 画像ハンドル,
    ロード操作: vk::AttachmentLoadOp,
    クリア値: vk::ClearValue,
) -> vk::RenderingAttachmentInfo<'static> {
    vk::RenderingAttachmentInfo::default()
        .image_view(レジストリ.ビューを取得する(ハンドル))
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(ロード操作)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(クリア値)
}
