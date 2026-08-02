//! 資源表世代がテクスチャをGPUへ常駐させる口。担当するのは、世代の構築と退役から実際の画像資源の作り方を切り離すことである。
//!
//! 本番はVulkanの画像を作る実装を渡し、世代の状態遷移の検査は資源を持たない実装を渡す。構築と退役の手順そのものが
//! GPUの実物に依存しないため、公開と退役の規律を狭い単体テストで固定できる。
//! 参照: `crates/blitz_render/src/vulkan/material_table/device_supplier.rs`

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;

pub(crate) trait 常駐テクスチャ供給元 {
    type 常駐画像;

    fn 常駐させる(&mut self, 素材: &テクスチャ素材) -> Result<Self::常駐画像, レンダラーエラー>;

    /// 注意: この画像を参照しうるフレームのフェンス通過後にだけ呼ぶ。世代の退役の規律は`資源表世代台帳`が持つ。
    fn 退役させる(&mut self, 画像: Self::常駐画像);
}
