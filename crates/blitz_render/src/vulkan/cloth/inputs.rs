//! 布一式から1フレームの描画入力を組み立てる。`mod.rs`の行数分割のための切り出し。

use super::布一式;
use crate::vulkan::frame::布描画入力;

impl 布一式 {
    pub(crate) fn 描画入力を作る(&self, フレーム添字: usize, 介入件数: u32) -> 布描画入力 {
        布描画入力 {
            layout: self.パイプライン群.layout,
            介入pipeline: self.パイプライン群.介入,
            積分pipeline: self.パイプライン群.積分,
            アタッチpipeline: self.パイプライン群.アタッチ,
            拘束pipeline: self.パイプライン群.拘束,
            ハッシュ消去pipeline: self.パイプライン群.ハッシュ消去,
            ハッシュ格納pipeline: self.パイプライン群.ハッシュ格納,
            分離pipeline: self.パイプライン群.分離,
            仕上げpipeline: self.パイプライン群.仕上げ,
            頂点生成pipeline: self.パイプライン群.頂点生成,
            ディスクリプタセット: self.ディスクリプタ.set一覧[フレーム添字],
            粒子数: self.固定部.粒子数,
            アタッチ件数: self.固定部.アタッチ件数,
            介入件数,
            粒子バッファ: self.バッファ.粒子.0,
            前位置バッファ: self.バッファ.前位置.0,
            セルカウントバッファ: self.バッファ.セルカウント.0,
            セル格納バッファ: self.バッファ.セル格納.0,
            布頂点バッファ: self.バッファ.布頂点.0,
            インデックスバッファ: self.バッファ.インデックス.0,
            インデックス数: self.インデックス数,
            描画pipeline: self.描画パイプライン.handle,
        }
    }
}
