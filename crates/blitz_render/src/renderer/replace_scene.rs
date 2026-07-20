//! アセットホットリロード時のシーン差し替え。device_wait_idle後に旧ジオメトリ・
//! 旧テクスチャを破棄して再構築し、ディスクリプタセットを新テクスチャへ更新する
//! (判断5)。パイプライン・ディスクリプタのレイアウト/プール/セット自体は変えない。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vertex::頂点;
use crate::vulkan;

impl レンダラー {
    pub fn シーンを差し替える(
        &mut self,
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
        ベースカラー: テクスチャ素材,
    ) -> Result<(), レンダラーエラー> {
        // 安全性: 旧ジオメトリ・旧テクスチャの破棄前にGPU使用完了を待つ。
        unsafe { self.device.device_wait_idle()? };

        // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
        let メモリプロパティ =
            unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };

        let 新ジオメトリ = vulkan::geometry::ジオメトリバッファ::生成する(
            &self.device,
            &メモリプロパティ,
            &self.転送環境,
            頂点一覧,
            インデックス一覧,
        )?;
        let 新テクスチャ = match vulkan::texture::テクスチャ::生成する(
            &self.device,
            &self.instance,
            self.physical_device,
            &メモリプロパティ,
            &self.転送環境,
            &ベースカラー,
        ) {
            Ok(テクスチャ) => テクスチャ,
            Err(誤り) => {
                新ジオメトリ.破棄する(&self.device);
                return Err(誤り);
            }
        };

        self.ジオメトリ.破棄する(&self.device);
        self.テクスチャ.破棄する(&self.device);
        self.ジオメトリ = 新ジオメトリ;
        self.テクスチャ = 新テクスチャ;
        self.ディスクリプタ.更新する(&self.device, &self.テクスチャ);
        Ok(())
    }
}
