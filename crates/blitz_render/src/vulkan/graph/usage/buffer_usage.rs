//! バッファ用途enum。画像用途と対になる型。
//!
//! 参照: `_doc/設計/レンダーグラフ.md`。波2のGPU粒子トイでコンピュート書き・
//! 頂点段シェーダー読みを使う。それ以外の変種は波2時点で未使用のため個別に許可する。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum バッファ用途 {
    #[allow(dead_code)]
    頂点読み,
    #[allow(dead_code)]
    インデックス読み,
    #[allow(dead_code)]
    ユニフォーム読み,
    #[allow(dead_code)]
    コンピュート読み,
    コンピュート書き,
    /// 頂点シェーダー段からストレージバッファとして読む(SV_VertexIDでの粒子位置読み等)。
    /// `頂点読み`(頂点入力バインドのVERTEX_ATTRIBUTE_READ)とは異なる同期状態を持つため
    /// 別の用途として区別する。
    頂点段シェーダー読み,
    #[allow(dead_code)]
    転送元,
    #[allow(dead_code)]
    転送先,
}
