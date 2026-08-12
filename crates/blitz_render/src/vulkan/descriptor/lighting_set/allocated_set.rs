//! 照明問い合わせのレイアウトで割り当て済みの、set3のディスクリプタセット1枚。
//!
//! この型を作れるのはシーンセットレイアウト一式の割り当てだけであり、生の`vk::DescriptorSet`から組み立てる口は無い。
//! 口があると、材質のセットや別の役割のセットを照明問い合わせの書き込み先へ包む呼び出しが型検査を通ってしまう。

use ash::vk;

use super::照明問い合わせのセットの書き込み先;

pub(crate) struct 照明問い合わせの割り当て済みセット(vk::DescriptorSet);

impl 照明問い合わせの割り当て済みセット {
    pub(in crate::vulkan::descriptor) const fn 刻む(セット: vk::DescriptorSet) -> Self {
        Self(セット)
    }

    /// このセットへ照明問い合わせの束縛番号だけを結ぶ口。
    /// 前提: 書き込みの時点でGPUがこのセットを使用していない(生成直後またはdevice_wait_idle後)。
    pub(crate) fn 書き込み先<'書き込み>(
        &self,
        device: &'書き込み ash::Device,
    ) -> 照明問い合わせのセットの書き込み先<'書き込み> {
        照明問い合わせのセットの書き込み先::生成する(device, self.0)
    }

    /// パイプラインへの束縛へ渡す境界。ここから先はVulkanの生のハンドルであり、この木へ戻る口は無い。
    pub(crate) const fn セットのハンドル(&self) -> vk::DescriptorSet {
        self.0
    }
}
