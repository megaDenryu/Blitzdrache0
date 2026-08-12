//! 材質のレイアウトで割り当て済みの、set2のディスクリプタセット1枚。
//!
//! この型を作れるのはシーンセットレイアウト一式の割り当てだけであり、生の`vk::DescriptorSet`から組み立てる口は無い。
//! 口があると、照明問い合わせのセットや別の役割のセットを材質の書き込み先へ包む呼び出しが型検査を通ってしまう。

use ash::vk;

use super::材質のセットの書き込み先;

pub(crate) struct 材質の割り当て済みセット(vk::DescriptorSet);

impl 材質の割り当て済みセット {
    pub(in crate::vulkan::descriptor) const fn 刻む(セット: vk::DescriptorSet) -> Self {
        Self(セット)
    }

    /// このセットへ材質の束縛番号だけを結ぶ口。
    /// 前提: 書き込みの時点でGPUがこのセットを使用していない(世代を作った直後である)。
    pub(crate) fn 書き込み先<'書き込み>(&self, device: &'書き込み ash::Device) -> 材質のセットの書き込み先<'書き込み> {
        材質のセットの書き込み先::生成する(device, self.0)
    }

    /// パイプラインへの束縛へ渡す境界。ここから先はVulkanの生のハンドルであり、この木へ戻る口は無い。
    pub(crate) const fn セットのハンドル(&self) -> vk::DescriptorSet {
        self.0
    }
}
