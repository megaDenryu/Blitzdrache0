//! レンダラーの明示的な破棄処理。フィールドのDrop順序に頼らず、
//! 生成の逆順で全Vulkanリソースを破棄する:
//! device_wait_idle → 読み戻しバッファ・パイプライン・同期・コマンド・ジオメトリ・
//! 深度バッファ・スワップチェーン・デバイス → サーフェス → メッセンジャー → インスタンス。

use super::レンダラー;

impl レンダラー {
    pub(super) fn 破棄する(&mut self) {
        // 安全性: この呼び出し以降このレンダラーは使用されない(Dropの唯一の呼び出し元)。
        // device_wait_idleでGPU上の全作業完了を待ってから破棄することで、
        // 使用中リソースの破棄によるvalidationエラーを避ける。
        let _ = unsafe { self.device.device_wait_idle() };

        if let Some(バッファ) = &self.読み戻しバッファ {
            バッファ.破棄する(&self.device);
        }
        self.ui一式.破棄する(&self.device);
        if let Some(粒子) = &self.粒子 {
            粒子.破棄する(&self.device);
        }
        if let Some(計測) = &self.gpu計測 {
            計測.破棄する(&self.device);
        }
        self.pipeline.破棄する(&self.device);
        self.シャドウパイプライン.破棄する(&self.device);
        self.フレーム同期.破棄する(&self.device);
        self.提示同期.破棄する(&self.device);
        // 安全性: command_bufferはcommand_poolの破棄で暗黙に解放されるため、
        // 個別のfree_command_buffersは不要。
        unsafe { self.device.destroy_command_pool(self.command_pool, None) };
        self.ディスクリプタ.破棄する(&self.device);
        self.ユニフォーム.破棄する(&self.device);
        self.テクスチャ.破棄する(&self.device);
        self.ジオメトリ.破棄する(&self.device);
        self.転送環境.破棄する(&self.device);
        self.深度バッファ.破棄する(&self.device);
        self.シャドウマップ.破棄する(&self.device);
        self.swapchain.破棄する(&self.device, &self.swapchain_loader);
        // 安全性: deviceはSelfが唯一の所有者で、上記の全依存リソースは破棄済み。
        unsafe { self.device.destroy_device(None) };
        // 安全性: surfaceはSelfが唯一の所有者で、対応するinstanceはこの後に破棄する。
        unsafe { self.surface_loader.destroy_surface(self.surface, None) };
        if let Some(デバッグメッセンジャー) = &self.デバッグメッセンジャー {
            デバッグメッセンジャー.破棄する();
        }
        // 安全性: instanceはSelfが唯一の所有者で、上記の全依存リソースは破棄済み。
        unsafe { self.instance.destroy_instance(None) };
    }
}
