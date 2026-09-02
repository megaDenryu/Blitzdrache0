//! 布パイプライン群の破棄の局面。呼ばれるのは布一式の破棄(レンダラーの破棄と布の差し替え)のときだけであり、生成(`pipelines.rs`)とは
//! 呼び出しの頻度もタイミングも異なるため局面で分ける(パーシャル規約2)。触れるのは13本のパイプラインのハンドルとレイアウトだけである。

use super::布パイプライン群;

impl 布パイプライン群 {
    pub(in crate::vulkan::cloth) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for handle in [
                self.介入,
                self.積分,
                self.目標の確定,
                self.乗数零化,
                self.拘束,
                self.曲げ拘束,
                self.目標拘束,
                self.ハッシュ消去,
                self.ハッシュ格納,
                self.分離,
                self.床とカプセルの押し出し,
                self.仕上げ,
                self.頂点生成,
            ] {
                device.destroy_pipeline(handle, None);
            }
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
