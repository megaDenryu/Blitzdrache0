//! 描画段階がGPUへ発行するときに束縛するパイプラインと、そのパイプラインレイアウトの所有者。
//! シーン段階・影段階・空段階が使う現物を具体フィールドで持ち、段階の種類を鍵にした表では持たない。
//! 段階が増えるたびにレンダラー直下のフィールドが増えることを止めるためにこの型がある
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
//! 破棄はレンダラーの単一破棄元(`renderer/destroy.rs`)がこの型の`破棄する`を1回呼ぶ形を保つ。
//! 生成はレンダラー生成時の1回だけの局面であり、`create`が担う。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 描画段階資源 {
    シーン: vulkan::pipeline::パイプライン,
    シャドウ: vulkan::pipeline::シャドウパイプライン,
    /// フレーム構成に空段階があるときだけ`Some`。有無はフレーム描画入力の空と常に一致させる。
    空: Option<vulkan::pipeline::空パイプライン>,
}

impl 描画段階資源 {
    /// `シーンカラー形式`はシーン段階の色アタッチメントの形式(ポスト処理があればHDR中間画像、無ければスワップチェーン)。
    /// 影段階は深度だけへ書くため色形式を要らない。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        シーンカラー形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー束,
        構成: フレーム構成,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, シーンカラー形式, ディスクリプタlayout, シェーダー, 構成)
    }

    /// 空段階を持つフレーム構成かどうか。フレーム描画入力の空の有無と突き合わせる判定に使う。
    pub(super) fn 空を描くか(&self) -> bool {
        self.空.is_some()
    }

    pub(super) fn 空描画入力を作る(&self, ディスクリプタセット: vk::DescriptorSet) -> Option<vulkan::frame::空描画入力> {
        self.空.as_ref().map(|空| vulkan::frame::空描画入力 {
            pipeline: 空.handle,
            layout: 空.layout,
            ディスクリプタセット,
        })
    }

    pub(super) fn シーンpipeline(&self) -> vk::Pipeline {
        self.シーン.handle
    }

    pub(super) fn シーンlayout(&self) -> vk::PipelineLayout {
        self.シーン.layout
    }

    pub(super) fn シャドウpipeline(&self) -> vk::Pipeline {
        self.シャドウ.handle
    }

    pub(super) fn シャドウlayout(&self) -> vk::PipelineLayout {
        self.シャドウ.layout
    }

    /// ホットリロードでシーンのパイプラインだけを入れ替える。
    /// 前提: 呼び出し元が旧パイプラインのGPU使用完了を待ってから呼ぶ(`renderer/replace_shader.rs`)。
    pub(super) fn シーンを差し替える(&mut self, device: &GPUデバイス, 新パイプライン: vulkan::pipeline::パイプライン) {
        self.シーン.破棄する(device);
        self.シーン = 新パイプライン;
    }

    /// 空・シーン・シャドウの順に破棄する。空はシーンの色形式と深度形式に合わせて後から作った段であり、生成の逆順で片付ける。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        if let Some(空) = &self.空 {
            空.破棄する(device);
        }
        self.シーン.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
