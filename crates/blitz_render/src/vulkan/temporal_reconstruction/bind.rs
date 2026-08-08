//! 時間再構成の2つのセットへ4つの画像を書き込む工程。
//! 呼び出しタイミング: 生成直後と、スワップチェーン再構築で深度画像と4枚を作り直した直後(どちらもGPU未使用の時点)。
//!
//! 触れるのはディスクリプタセットだけであり、深度のビューは引数で受け取る。セットの番号は履歴の読み側の添字であり、
//! 番号iのセットは履歴のi枚目を読む。書き込み先はカラー添付として渡るためセットに現れない。

use ash::vk;

use super::時間再構成一式;

impl 時間再構成一式 {
    /// 前提: 呼び出し時点でGPUがこれらのセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn 資源を束縛する(&self, device: &ash::Device, 深度ビュー: vk::ImageView) {
        for 読み添字 in 0..self.画像組.履歴.len() {
            self.一つのセットを束縛する(device, 深度ビュー, 読み添字);
        }
    }

    /// 注意: 今のフレームの色と動きベクトルのレイアウトは`画像用途::シェーダー読み画素段`が導く
    /// SHADER_READ_ONLY_OPTIMALと、履歴のレイアウトは`画像用途::履歴の画素段参照`が導くGENERALと、
    /// 深度のレイアウトは`画像用途::深度シェーダー読み`が導くDEPTH_READ_ONLY_OPTIMALと一致させる。
    /// 食い違うとvalidationがディスクリプタのレイアウト不一致を報告する。
    fn 一つのセットを束縛する(&self, device: &ash::Device, 深度ビュー: vk::ImageView, 読み添字: usize) {
        let 今の色一覧 = [読み取りの情報(
            self.画像組.今のフレームの色.画像ビュー,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )];
        let 履歴一覧 = [読み取りの情報(self.画像組.履歴[読み添字].画像ビュー, vk::ImageLayout::GENERAL).sampler(self.標本器)];
        let 動きベクトル一覧 = [読み取りの情報(
            self.画像組.動きベクトル.画像ビュー,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )];
        let 深度一覧 = [読み取りの情報(深度ビュー, vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
        let セット = self.ディスクリプタ.セット一覧[読み添字];
        let write一覧 = [
            画像の書き込み(セット, 0, vk::DescriptorType::SAMPLED_IMAGE, &今の色一覧),
            画像の書き込み(セット, 1, vk::DescriptorType::COMBINED_IMAGE_SAMPLER, &履歴一覧),
            画像の書き込み(セット, 2, vk::DescriptorType::SAMPLED_IMAGE, &動きベクトル一覧),
            画像の書き込み(セット, 3, vk::DescriptorType::SAMPLED_IMAGE, &深度一覧),
        ];
        // 安全性: セットは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
        unsafe { device.update_descriptor_sets(&write一覧, &[]) };
    }
}

fn 読み取りの情報(ビュー: vk::ImageView, レイアウト: vk::ImageLayout) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo::default().image_view(ビュー).image_layout(レイアウト)
}

fn 画像の書き込み<'a>(
    セット: vk::DescriptorSet,
    番号: u32,
    種別: vk::DescriptorType,
    情報一覧: &'a [vk::DescriptorImageInfo],
) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(セット)
        .dst_binding(番号)
        .descriptor_type(種別)
        .image_info(情報一覧)
}
