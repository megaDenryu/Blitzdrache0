//! ホットリロード時のパイプライン差し替え。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断7」「判断10」。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan;
use crate::vulkan::depth::深度形式;

impl レンダラー {
    /// 新しいシェーダー一式で新パイプラインを生成し、GPU使用完了を待ってから
    /// 旧パイプラインと差し替える。生成に失敗した場合は旧パイプラインを保持したまま
    /// エラーを返す（呼び出し元のblitz_appはこれを「コンパイル失敗、旧パイプライン継続」
    /// として扱う）。
    pub fn シェーダーを差し替える(&mut self, シェーダー: シェーダー一式) -> Result<(), レンダラーエラー> {
        let device = self.環境.device();
        let ディスクリプタlayout = self.シーン描画資源.ディスクリプタlayout();
        let 新パイプライン =
            vulkan::pipeline::パイプライン::生成する(device, self.swapchain.画像形式, 深度形式, ディスクリプタlayout, &シェーダー)?;

        // 旧パイプラインの使用完了を待ってから破棄する。
        self.環境.gpuの全作業完了を待つ()?;
        self.pipeline.破棄する(device);
        self.pipeline = 新パイプライン;
        Ok(())
    }
}
