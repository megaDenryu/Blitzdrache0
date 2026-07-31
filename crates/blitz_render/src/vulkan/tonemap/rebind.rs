//! 明るさの圧縮のディスクリプタセットへHDRビュー(binding0)と光のにじみビュー(binding1)を束縛する。

use ash::vk;

use super::明るさの圧縮一式;

impl 明るさの圧縮一式 {
    /// 生成直後と、スワップチェーン再構築でHDR/光のにじみ画像を作り直した後に呼ぶ。
    /// 前提: 呼び出し時点でGPUがこのディスクリプタセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn ビューを再束縛する(&self, device: &ash::Device, hdrビュー: vk::ImageView, 光のにじみビュー: vk::ImageView) {
        let hdr情報一覧 = [vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(hdrビュー)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let 光のにじみ情報一覧 = [vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(光のにじみビュー)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let write一覧 = [
            vk::WriteDescriptorSet::default()
                .dst_set(self.descriptor_set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&hdr情報一覧),
            vk::WriteDescriptorSet::default()
                .dst_set(self.descriptor_set)
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&光のにじみ情報一覧),
        ];
        // 安全性: setは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
        unsafe { device.update_descriptor_sets(&write一覧, &[]) };
    }
}
