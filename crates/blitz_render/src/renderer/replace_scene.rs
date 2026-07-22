//! アセットホットリロード時のシーン差し替え。device_wait_idle後に対象資源と
//! ディスクリプタ一式を再構築し、パイプラインは互換レイアウトのまま維持する。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::render_object_resources;
use crate::vulkan;

impl レンダラー {
    pub fn シーンを差し替える(&mut self, 描画シーン: 描画シーン素材) -> Result<(), レンダラーエラー> {
        // M8最小実装の制約(判断44): スキニングバッファの再構築が未実装のため、
        // スキン付きシーンを保持中の差し替えは型付きエラーで拒否する(無言の壊れた描画をしない)。
        if self.スキニング.is_some() {
            return Err(レンダラーエラー::スキン付きシーン差し替え未対応);
        }
        // 安全性: 旧ジオメトリ・旧テクスチャの破棄前にGPU使用完了を待つ。
        unsafe { self.device.device_wait_idle()? };

        // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
        let メモリプロパティ = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };

        let 新資源一覧 = render_object_resources::描画対象資源一覧を生成する(
            &self.instance,
            self.physical_device,
            &self.device,
            &メモリプロパティ,
            &self.転送環境,
            &描画シーン,
        )?;
        let 新ディスクリプタ = self.ディスクリプタを生成する(&新資源一覧)?;

        self.ジオメトリ入力作業領域.clear();
        self.ジオメトリ入力作業領域.reserve(新資源一覧.len());
        self.シャドウ入力作業領域.clear();
        self.シャドウ入力作業領域.reserve(新資源一覧.len());
        let 旧資源一覧 = std::mem::replace(&mut self.描画対象資源一覧, 新資源一覧);
        let 旧ディスクリプタ = std::mem::replace(&mut self.ディスクリプタ, 新ディスクリプタ);
        旧ディスクリプタ.破棄する(&self.device);
        for 資源 in &旧資源一覧 {
            資源.破棄する(&self.device);
        }
        Ok(())
    }

    fn ディスクリプタを生成する(
        &self,
        資源一覧: &[render_object_resources::描画対象資源],
    ) -> Result<vulkan::descriptor::ディスクリプタ一式, レンダラーエラー> {
        let 参照一覧 = 資源一覧
            .iter()
            .map(|資源| vulkan::descriptor::描画対象ディスクリプタ参照 {
                テクスチャ: &資源.テクスチャ,
                ユニフォーム: &資源.ユニフォーム,
            })
            .collect::<Vec<_>>();
        match vulkan::descriptor::ディスクリプタ一式::生成する(&self.device, &参照一覧, &self.ユニフォーム, &self.シャドウマップ)
        {
            Ok(値) => Ok(値),
            Err(誤り) => {
                for 資源 in 資源一覧 {
                    資源.破棄する(&self.device);
                }
                Err(誤り)
            }
        }
    }
}
