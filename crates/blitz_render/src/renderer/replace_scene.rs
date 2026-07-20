//! アセットホットリロード時のシーン差し替え。device_wait_idle後に旧ジオメトリ・
//! 旧テクスチャを破棄して再構築し、ディスクリプタセットのテクスチャ部分を新テクスチャへ
//! 更新する(判断5)。パイプライン・ディスクリプタのレイアウト/プール/セット自体、
//! UBOバインディングは変えない。マテリアル係数は次フレーム以降のUBO書き込みに
//! 反映されるようここで更新する(判断24)。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vertex::頂点;
use crate::vulkan;

impl レンダラー {
    pub fn シーンを差し替える(
        &mut self,
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
        マテリアル: マテリアル素材,
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
        let 新テクスチャ = match vulkan::texture::マテリアルテクスチャ一式::生成する(
            &self.device,
            &self.instance,
            self.physical_device,
            &メモリプロパティ,
            &self.転送環境,
            &マテリアル,
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
        self.ディスクリプタ.テクスチャを更新する(&self.device, &self.テクスチャ);
        self.ベースカラー係数 = マテリアル.ベースカラー係数();
        self.金属度係数 = マテリアル.金属度係数();
        self.粗さ係数 = マテリアル.粗さ係数();
        Ok(())
    }
}
