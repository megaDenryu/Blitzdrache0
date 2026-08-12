//! 自動露出のセットへ4つの資源を書き込む工程。
//! 呼び出しタイミング: 生成直後と、スワップチェーン再構築でHDR中間画像を作り直した直後(どちらもGPU未使用の時点)。
//!
//! 触れるのはディスクリプタセット1つだけであり、バッファもHDR中間画像も引数で受け取る。

use ash::vk;

use super::自動露出一式;

impl 自動露出一式 {
    /// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn 資源を束縛する(&self, device: &ash::Device, hdrビュー: vk::ImageView) {
        // 注意: レイアウトは`画像用途::シェーダー読みコンピュート段`が導くSHADER_READ_ONLY_OPTIMALと一致させる。
        // 食い違うとvalidationがディスクリプタのレイアウト不一致を報告する。
        let 画像情報一覧 = [vk::DescriptorImageInfo::default()
            .image_view(hdrビュー)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let 境界情報一覧 = [バッファ全体(self.バッファ.境界.バッファのハンドル())];
        let ヒストグラム情報一覧 = [バッファ全体(self.バッファ.ヒストグラム.バッファのハンドル())];
        let 露出状態情報一覧 = [バッファ全体(self.バッファ.露出状態.バッファのハンドル())];
        let セット = self.ディスクリプタ.セット;
        let write一覧 = [
            vk::WriteDescriptorSet::default()
                .dst_set(セット)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&画像情報一覧),
            記憶の書き込み(セット, 1, &境界情報一覧),
            記憶の書き込み(セット, 2, &ヒストグラム情報一覧),
            記憶の書き込み(セット, 3, &露出状態情報一覧),
        ];
        // 安全性: セットは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
        unsafe { device.update_descriptor_sets(&write一覧, &[]) };
    }
}

fn バッファ全体(バッファ: vk::Buffer) -> vk::DescriptorBufferInfo {
    vk::DescriptorBufferInfo::default().buffer(バッファ).offset(0).range(vk::WHOLE_SIZE)
}

fn 記憶の書き込み<'a>(セット: vk::DescriptorSet, 番号: u32, 情報一覧: &'a [vk::DescriptorBufferInfo]) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(セット)
        .dst_binding(番号)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(情報一覧)
}
