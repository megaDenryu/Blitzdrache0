//! 局所可視性のセットへ3つの画像を書き込む工程。
//! 呼び出しタイミング: 生成直後と、スワップチェーン再構築で深度画像と2枚の可視度画像を作り直した直後
//! (どちらもGPU未使用の時点)。
//!
//! 触れるのはディスクリプタセット1つだけであり、深度のビューは引数で受け取る。

use ash::vk;

use super::局所可視性一式;

impl 局所可視性一式 {
    /// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn 資源を束縛する(&self, device: &ash::Device, 深度ビュー: vk::ImageView) {
        // 注意: 深度のレイアウトは`画像用途::深度コンピュート読み`が導くDEPTH_READ_ONLY_OPTIMALと一致させる。
        // 2枚の可視度画像は`コンピュート書き`と`コンピュート記憶読み`が導くGENERALと一致させる。
        // 食い違うとvalidationがディスクリプタのレイアウト不一致を報告する。
        let 深度情報一覧 = [vk::DescriptorImageInfo::default()
            .image_view(深度ビュー)
            .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
        let 生情報一覧 = [記憶画像の情報(self.画像組.生.画像ビュー)];
        let ぼかし後情報一覧 = [記憶画像の情報(self.画像組.ぼかし後.画像ビュー)];
        let セット = self.ディスクリプタ.セット;
        let write一覧 = [
            vk::WriteDescriptorSet::default()
                .dst_set(セット)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&深度情報一覧),
            記憶画像の書き込み(セット, 1, &生情報一覧),
            記憶画像の書き込み(セット, 2, &ぼかし後情報一覧),
        ];
        // 安全性: セットは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
        unsafe { device.update_descriptor_sets(&write一覧, &[]) };
    }
}

fn 記憶画像の情報(ビュー: vk::ImageView) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo::default()
        .image_view(ビュー)
        .image_layout(vk::ImageLayout::GENERAL)
}

fn 記憶画像の書き込み<'a>(
    セット: vk::DescriptorSet, 番号: u32, 情報一覧: &'a [vk::DescriptorImageInfo]
) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(セット)
        .dst_binding(番号)
        .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
        .image_info(情報一覧)
}
