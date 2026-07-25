//! 開発用UI描画データ(公開API)から、このフレームぶんのジオメトリ書き込みと
//! `vulkan::frame::UI描画入力`の組み立てを行う(判断33・34)。メッシュ結合・
//! シザー変換は`merge`に委ねる。

mod merge;

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::ui_draw_data::UI描画データ;
use crate::ui_mesh::UIメッシュ;
use crate::vulkan;

impl レンダラー {
    /// `データ`が`None`、または有効なメッシュ(頂点・インデックスとも非空)が
    /// 1つも無ければ`None`を返し、UIパス自体をグラフへ積ませない。
    pub(super) fn ui描画入力を組み立てる(
        &mut self,
        フレーム添字: usize,
        データ: Option<&UI描画データ>,
    ) -> Result<Option<vulkan::frame::UI描画入力>, レンダラーエラー> {
        let Some(データ) = データ else { return Ok(None) };
        let 有効一覧: Vec<&UIメッシュ> = データ.メッシュ一覧.iter().filter(|メッシュ| merge::有効なメッシュか(メッシュ)).collect();
        if 有効一覧.is_empty() {
            return Ok(None);
        }

        let 寸法 = self.提示資源.寸法();
        let (頂点一覧結合, インデックス一覧結合, 項目一覧) = self.メッシュ列を結合する(&有効一覧, 寸法);

        let メモリプロパティ = self.環境.メモリプロパティを取得する();
        let (頂点バッファ, インデックスバッファ) =
            self.ui一式
                .ジオメトリを書き込む(self.環境.device(), &メモリプロパティ, フレーム添字, &頂点一覧結合, &インデックス一覧結合)?;

        Ok(Some(vulkan::frame::UI描画入力 {
            pipeline: self.ui一式.pipeline_handle(),
            layout: self.ui一式.pipeline_layout(),
            頂点バッファ,
            インデックスバッファ,
            項目一覧,
        }))
    }
}
