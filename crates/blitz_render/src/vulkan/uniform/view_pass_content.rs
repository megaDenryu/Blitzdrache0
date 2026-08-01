//! ビュー・シーンパス定数へ書き込む内容。全シーン描画が同じビューで読む小値と、直接照明の値を持つ。
//! std140/HLSL cbufferのどちらでも16バイト境界を跨がないよう、行列以外は全フィールドをvec4単位にそろえて
//! GPU側(shaders/view_pass_uniform.slangのViewPassUniform)と1対1に対応させる(判断24)。
//!
//! 直接照明をここへ置くのは段5で照明問い合わせセットへ移すまでの仮置きである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
//!
//! 注意: フィールド順序はshaders/view_pass_uniform.slangの宣言順と完全に一致させること。
//! ここを崩すと`view_pass_bytes::バイト列にする`の開始位置がシェーダー側とずれ、値化けとして現れる。

pub(crate) struct ビューとシーンパスの定数内容 {
    pub(crate) ビュー射影行列: [[f32; 4]; 4],
    pub(crate) カメラ相対位置: [f32; 3],
    /// カメラの前方向(カメラ相対フレームの単位ベクトル)。ビュー空間深度を内積で求めるために渡す。
    pub(crate) カメラ前方: [f32; 3],
    pub(crate) 方向光方向: [f32; 3],
    pub(crate) 方向光色: [f32; 3],
    pub(crate) 方向光強度: f32,
    pub(crate) 点光源位置: [f32; 3],
    pub(crate) 点光源色: [f32; 3],
    pub(crate) 点光源強度: f32,
    pub(crate) 環境光係数: f32,
    pub(crate) ライティング有効: bool,
}
