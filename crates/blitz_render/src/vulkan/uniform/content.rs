//! フレームユニフォームへ書き込む内容。std140/HLSL cbufferのどちらでも
//! 16バイト境界を跨がないよう、行列以外は全フィールドをvec4単位にそろえて
//! GPU側(shaders/scene.slangのFrameUniform)と1対1に対応させる(判断24)。
//!
//! 注意: フィールド順序はshaders/scene.slangのFrameUniform構造体の宣言順と
//! 完全に一致させること。ここを崩すと`bytes::バイト列にする`のオフセット計算が
//! シェーダー側とずれ、値化けとして現れる。

pub(crate) struct フレームユニフォーム内容 {
    pub(crate) ビュー射影行列: [[f32; 4]; 4],
    pub(crate) カメラワールド位置: [f32; 3],
    pub(crate) 方向光方向: [f32; 3],
    pub(crate) 方向光色: [f32; 3],
    pub(crate) 方向光強度: f32,
    pub(crate) 点光源位置: [f32; 3],
    pub(crate) 点光源色: [f32; 3],
    pub(crate) 点光源強度: f32,
    pub(crate) 環境光係数: f32,
    pub(crate) ライティング有効: bool,
    pub(crate) ベースカラー係数: [f32; 4],
    pub(crate) 金属度係数: f32,
    pub(crate) 粗さ係数: f32,
}
