//! ブルームの各ディスクリプタセットへ読み元ビューを書く。
//! 前処理set←HDR、縮小set[i]←縮小[i](縮小[i+1]への縮小パスの読み元)、
//! 拡大set[i]←(1段小さい拡大結果または縮小最終段, 縮小[i])。

use ash::vk;

use super::ブルーム一式;
use crate::vulkan::bloom_targets::ブルームピラミッド;

impl ブルーム一式 {
    /// 前提: 呼び出し時点でGPUがこれらのディスクリプタセットを使用していないこと
    /// (`ディスクリプタを作り直す`経由でのみ呼ばれ、その前提を引き継ぐ)。
    pub(super) fn ビューを書く(&self, device: &ash::Device, hdrビュー: vk::ImageView, ピラミッド: &ブルームピラミッド) {
        書く(device, self.sampler, self.前処理set, &[hdrビュー]);
        for (添字, &set) in self.縮小set一覧.iter().enumerate() {
            書く(device, self.sampler, set, &[ピラミッド.縮小一覧[添字].画像ビュー]);
        }
        for (添字, &set) in self.拡大set一覧.iter().enumerate() {
            let 小さい方 = if 添字 + 1 < self.拡大set一覧.len() {
                ピラミッド.拡大一覧[添字 + 1].画像ビュー
            } else {
                ピラミッド.縮小一覧[添字 + 1].画像ビュー
            };
            書く(device, self.sampler, set, &[小さい方, ピラミッド.縮小一覧[添字].画像ビュー]);
        }
    }
}

fn 書く(device: &ash::Device, sampler: vk::Sampler, set: vk::DescriptorSet, ビュー一覧: &[vk::ImageView]) {
    let 情報一覧: Vec<[vk::DescriptorImageInfo; 1]> = ビュー一覧
        .iter()
        .map(|&ビュー| {
            [vk::DescriptorImageInfo::default()
                .sampler(sampler)
                .image_view(ビュー)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)]
        })
        .collect();
    let write一覧: Vec<vk::WriteDescriptorSet<'_>> = 情報一覧
        .iter()
        .enumerate()
        .map(|(binding, 情報)| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(u32::try_from(binding).unwrap_or_else(|_| panic!("binding番号がu32に収まらない: {binding}")))
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(情報)
        })
        .collect();
    // 安全性: setは割り当て済みで、呼び出し元の前提によりGPU未使用の時点でのみ呼ばれる。
    unsafe { device.update_descriptor_sets(&write一覧, &[]) };
}
