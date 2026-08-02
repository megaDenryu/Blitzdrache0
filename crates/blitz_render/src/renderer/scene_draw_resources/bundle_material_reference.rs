//! 描画対象の材質スロット番号を解決した結果。担当するのは、解決の答えが「その描画対象のディスクリプタセットの並びの添字」と
//! 「その描画対象が持つ材質レコード列の添字」の両方であることを1つの型で表すことである。
//! 2つは同じ並びから来るため、生のusizeで運ぶと片方だけ別の値を渡しても型が通ってしまう。
//!
//! 名前が束の内側であることを言うのは、この参照が資源表の世代に拘束されない値だからである。段4で入れる大域の材質IDと
//! 世代拘束済みの材質GPU参照は別の型であり、この参照をフレームをまたいで持ち回してはならない。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」

#[derive(Clone, Copy)]
pub(in crate::renderer::scene_draw_resources) struct 束内材質参照 {
    添字: usize,
}

impl 束内材質参照 {
    pub(in crate::renderer::scene_draw_resources) fn 生成する(添字: usize) -> Self {
        Self { 添字 }
    }

    /// 材質スロットを1件しか持たない対象と、布のように材質を読まない描画が使う先頭の参照。
    pub(in crate::renderer::scene_draw_resources) fn 先頭() -> Self {
        Self::生成する(0)
    }

    pub(in crate::renderer::scene_draw_resources) fn セットの並びの添字(self) -> usize {
        self.添字
    }

    /// 描画定数へ載せる材質レコードの添字。材質スロット数は1つの原型へ作者が割り当てた材質の数であり、
    /// u32へ収まらないことは呼び出し元の作り方の誤りである。
    pub(in crate::renderer::scene_draw_resources) fn 材質レコード添字(self) -> u32 {
        u32::try_from(self.添字).unwrap_or_else(|_| panic!("材質レコードの添字がu32に収まらない: {}", self.添字))
    }
}
