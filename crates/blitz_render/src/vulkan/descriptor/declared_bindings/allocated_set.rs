//! 宣言から作ったセットレイアウトで割り当て済みの、ディスクリプタセット1枚。割り当てたときの宣言を刻んで持つ。
//!
//! この型を作れるのは同じ木の中のレイアウトの割り当てだけであり、生の`vk::DescriptorSet`から組み立てる口は無い。
//! 口があると、別の宣言で割り当てたセットをこの宣言の書き込み先へ包む呼び出しが型検査を通ってしまう。

use ash::vk;

use super::{宣言した束縛の並び, 宣言どおりに結ぶ書き込み先};

pub(crate) struct 宣言から割り当てたセット<const 本数: usize> {
    宣言: 宣言した束縛の並び<本数>,
    セット: vk::DescriptorSet,
}

impl<const 本数: usize> 宣言から割り当てたセット<本数> {
    pub(super) const fn 刻む(宣言: 宣言した束縛の並び<本数>, セット: vk::DescriptorSet) -> Self {
        Self { 宣言, セット }
    }

    /// 刻まれた宣言の並び順に現物を結ぶ口。
    /// 前提: 書き込みの時点でGPUがこのセットを使用していない(生成直後またはdevice_wait_idle後)。
    pub(crate) fn 書き込み先<'書き込み>(
        &self,
        device: &'書き込み ash::Device,
    ) -> 宣言どおりに結ぶ書き込み先<'書き込み, 本数> {
        宣言どおりに結ぶ書き込み先::生成する(device, self.セット, self.宣言)
    }

    /// パイプラインへの束縛へ渡す境界。ここから先はVulkanの生のハンドルであり、この木へ戻る口は無い。
    pub(crate) const fn セットのハンドル(&self) -> vk::DescriptorSet {
        self.セット
    }
}
