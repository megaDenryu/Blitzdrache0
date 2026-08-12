//! ディスクリプタを書き込む先。割り当て済みの1つのディスクリプタセットと、そのセットを配った論理デバイスの組である。
//!
//! 2つを組で持つのは、書き込みの工程へ論理デバイスとセットを別々に運ばせると、別の役割のセットと組み合わせた呼び出しが
//! 署名の上で成立してしまうためである。役割ごとのセットは、この組を包んだ自分の型を持ち、自分が持つ束縛番号だけを結ぶ
//! メソッドで公開する(参照: `lighting_set/set_write.rs`)。サブシステムが自分だけで使うセットは、束縛の並びを宣言して
//! 位置で結ぶ`declared_bindings`を通る。この型が知るのはVulkanの書き込み記述子の組み立て方だけであり、どの番号に何を
//! 結ぶかは知らない。同じ形の組は命令の記録側にもある(参照: `crates/blitz_render/src/vulkan/command_sink.rs`)。
//!
//! 注意: この型と書き込みのメソッドは`descriptor`の木の内側だけへ閉じている。木の外から作れると、
//! 照明問い合わせのセットへ材質の束縛番号を書く呼び出しが署名の上で成立し、役割ごとに型を分けた意味が消える。

use ash::vk;

use super::束縛番号;

#[derive(Clone, Copy)]
pub(super) struct ディスクリプタの書き込み先<'書き込み> {
    device: &'書き込み ash::Device,
    セット: vk::DescriptorSet,
}

impl<'書き込み> ディスクリプタの書き込み先<'書き込み> {
    /// 前提: セットは`device`が配ったプールから割り当て済みであり、書き込みの時点でGPUがそのセットを使用していない。
    pub(super) fn 生成する(device: &'書き込み ash::Device, セット: vk::DescriptorSet) -> Self {
        Self { device, セット }
    }

    pub(super) fn バッファ全体を結ぶ(&self, 番号: 束縛番号, 種別: vk::DescriptorType, buffer: vk::Buffer) {
        self.バッファの先頭からの範囲を結ぶ(番号, 種別, buffer, vk::WHOLE_SIZE);
    }

    pub(super) fn バッファの先頭からの範囲を結ぶ(
        &self,
        番号: 束縛番号,
        種別: vk::DescriptorType,
        buffer: vk::Buffer,
        範囲: vk::DeviceSize,
    ) {
        let 情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(範囲)];
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(self.セット)
            .dst_binding(番号.gpu境界値())
            .dst_array_element(0)
            .descriptor_type(種別)
            .buffer_info(&情報一覧)];
        self.書き込みを送る(&書き込み一覧);
    }

    pub(super) fn サンプラー付きの画像を結ぶ(
        &self,
        番号: 束縛番号,
        ビュー: vk::ImageView,
        サンプラー: vk::Sampler,
        レイアウト: vk::ImageLayout,
    ) {
        let 情報一覧 = [vk::DescriptorImageInfo::default()
            .sampler(サンプラー)
            .image_view(ビュー)
            .image_layout(レイアウト)];
        self.画像の並びを結ぶ(番号, vk::DescriptorType::COMBINED_IMAGE_SAMPLER, &情報一覧);
    }

    /// 配列の束縛先へ先頭から順に結ぶ。並びが空なら1件も書かず、要素を部分束縛のまま未書込で残す。
    pub(super) fn サンプラー無しの画像の並びを結ぶ(
        &self, 番号: 束縛番号, ビュー一覧: &[vk::ImageView], レイアウト: vk::ImageLayout
    ) {
        let 情報一覧: Vec<vk::DescriptorImageInfo> = ビュー一覧
            .iter()
            .map(|ビュー| vk::DescriptorImageInfo::default().image_view(*ビュー).image_layout(レイアウト))
            .collect();
        self.画像の並びを結ぶ(番号, vk::DescriptorType::SAMPLED_IMAGE, &情報一覧);
    }

    /// サンプラーを伴わない1枚の画像を、指定された種別のまま結ぶ。
    pub(super) fn 画像1枚を結ぶ(
        &self, 番号: 束縛番号, 種別: vk::DescriptorType, ビュー: vk::ImageView, レイアウト: vk::ImageLayout
    ) {
        let 情報一覧 = [vk::DescriptorImageInfo::default().image_view(ビュー).image_layout(レイアウト)];
        self.画像の並びを結ぶ(番号, 種別, &情報一覧);
    }

    fn 画像の並びを結ぶ(&self, 番号: 束縛番号, 種別: vk::DescriptorType, 情報一覧: &[vk::DescriptorImageInfo]) {
        if 情報一覧.is_empty() {
            return;
        }
        let 書き込み一覧 = [vk::WriteDescriptorSet::default()
            .dst_set(self.セット)
            .dst_binding(番号.gpu境界値())
            .dst_array_element(0)
            .descriptor_type(種別)
            .image_info(情報一覧)];
        self.書き込みを送る(&書き込み一覧);
    }

    fn 書き込みを送る(&self, 書き込み一覧: &[vk::WriteDescriptorSet<'_>]) {
        // 安全性: セットは割当済みであり、書き込みが指すバッファ・画像ビュー・サンプラーは生成済みで有効である
        // (生成の責任は各束縛番号を持つモジュールが引数の前提として負う)。
        unsafe { self.device.update_descriptor_sets(書き込み一覧, &[]) };
    }
}
