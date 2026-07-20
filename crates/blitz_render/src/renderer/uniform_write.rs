//! フレームユニフォームへ書き込む内容の組み立てとGPUへの反映(判断24)。
//! 呼び出しタイミング: draw.rsの`フレームを実行する`がフェンス待ち後に呼ぶ
//! (前回のGPU使用完了後であることが保証される)。

use blitz_math::{ワールド, 位置};

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::frame_input::フレーム描画入力;
use crate::vulkan::uniform::{light_transform, lighting_constants, フレームユニフォーム内容};

impl レンダラー {
    pub(super) fn ユニフォームを書き込む(
        &self,
        フレーム添字: usize,
        入力: &フレーム描画入力,
        ビュー射影行列: &[[f32; 4]; 4],
    ) -> Result<(), レンダラーエラー> {
        let 内容 = フレームユニフォーム内容 {
            ライトビュー射影行列: light_transform::ライトビュー射影行列(),
            ビュー射影行列: *ビュー射影行列,
            カメラワールド位置: 位置をgpu境界配列にする(入力.カメラ位置),
            方向光方向: lighting_constants::方向光方向(),
            方向光色: lighting_constants::方向光色(),
            方向光強度: lighting_constants::方向光強度(),
            点光源位置: lighting_constants::点光源位置(),
            点光源色: lighting_constants::点光源色(),
            点光源強度: lighting_constants::点光源強度(),
            環境光係数: lighting_constants::環境光係数(),
            ライティング有効: 入力.ライティング有効,
            ベースカラー係数: self.ベースカラー係数,
            金属度係数: self.金属度係数,
            粗さ係数: self.粗さ係数,
        };
        self.ユニフォーム.書き込む(&self.device, フレーム添字, &内容)
    }
}

/// GPU境界(生型境界): フレーム型・単位型を外して生の[f32;3]へ戻す唯一の場所
/// (数学DDD。参照: CLAUDE.md「型安全性」)。
fn 位置をgpu境界配列にする(位置: 位置<ワールド>) -> [f32; 3] {
    [位置.x().値(), 位置.y().値(), 位置.z().値()]
}
