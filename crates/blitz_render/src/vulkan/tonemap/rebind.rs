//! 明るさの圧縮のディスクリプタセットへHDRビュー(binding0)と光のにじみビュー(binding1)と
//! GPU上の露出状態(binding2)を束縛する。露出状態は作り直さないため、生成直後の1回だけ別の入口で書く。

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

    /// 生成直後に1度だけ呼ぶ。露出状態のバッファはスワップチェーン再構築で作り直さないため、再束縛の対象にしない。
    pub(crate) fn 露出状態を束縛する(&self, device: &ash::Device, 露出状態バッファ: vk::Buffer) {
        let 情報一覧 = [vk::DescriptorBufferInfo::default()
            .buffer(露出状態バッファ)
            .offset(0)
            .range(vk::WHOLE_SIZE)];
        let write一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(self.descriptor_set)
            .dst_binding(2)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&情報一覧)];
        // 安全性: setは割り当て済みで、生成直後のGPU未使用の時点でのみ呼ばれる。
        unsafe { device.update_descriptor_sets(&write一覧, &[]) };
    }
}
