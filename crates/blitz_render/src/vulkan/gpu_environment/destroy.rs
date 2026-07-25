//! 破棄局面: 終了時に1度だけ呼ぶ保持ハンドルの破棄。この4段の順序を持つことがこの束の存在理由である。
//! サーフェスはインスタンス水準のオブジェクトのため、vkDestroyDeviceの後・vkDestroyInstanceの前に
//! 破棄しなければならない。ローダーの生存はentryに依るため、entryはこの4段のあいだ保持し続ける。
//!
//! 前提: 論理デバイスに依存する全資源(スワップチェーン・各バッファ・各パイプライン)の破棄と、
//! GPU作業完了の待機と、専用メモリの全解放確認は、呼び出し元(renderer/destroy.rs)がここより前に済ませる。

use super::GPU環境;

impl GPU環境 {
    pub(crate) fn 破棄する(&self) {
        // 安全性: deviceはSelfが唯一の所有者で、依存する全リソースの破棄は呼び出し元が済ませている。
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
