//! ブルームの3ディスクリプタセットへ読み元ビューを束縛する
//! (抽出set←HDR、横ぼかしset←a、縦ぼかしset←b)。

use ash::vk;

use super::ブルーム一式;

impl ブルーム一式 {
    /// 生成直後と、スワップチェーン再構築でHDR/ブルーム画像を作り直した後に呼ぶ。
    /// 前提: 呼び出し時点でGPUがこれらのディスクリプタセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn ビューを再束縛する(
        &self,
        device: &ash::Device,
        hdrビュー: vk::ImageView,
        aビュー: vk::ImageView,
        bビュー: vk::ImageView,
    ) {
        for (set, ビュー) in [(self.抽出set, hdrビュー), (self.横set, aビュー), (self.縦set, bビュー)] {
            let 情報一覧 = [vk::DescriptorImageInfo::default()
                .sampler(self.sampler)
                .image_view(ビュー)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let write = vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&情報一覧);
            // 安全性: setは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
            unsafe { device.update_descriptor_sets(std::slice::from_ref(&write), &[]) };
        }
    }
}
