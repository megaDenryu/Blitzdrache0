//! 描画段階がGPUへ発行するときに束縛するパイプラインと、そのパイプラインレイアウト・専用ディスクリプタ・専用画像の所有者。
//! シーン段階・影段階・空段階・影段階の布専用経路が使う現物を具体フィールドで持ち、段階の種類を鍵にした表では持たない。
//! 空段階の大気LUTのように段階が専有する画像もここが持つ。段階に属する資源をレンダラー直下へ足さないことがこの器の目的であり、
//! 資源の種類がパイプラインかどうかで置き場所を分けると目的が達せられないためである。
//! 段階が増えるたびにレンダラー直下のフィールドが増えることを止めるためにこの型がある
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
//! 破棄はレンダラーの単一破棄元(`renderer/destroy.rs`)がこの型の`破棄する`を1回呼ぶ形を保つ。
//! 生成はレンダラー生成時の1回だけの局面であり、`create`が担う。段階ごとの操作は、大気LUTだけに触れるものを`atmosphere`、
//! 空段階だけに触れるものを`sky`が持つ。

mod atmosphere;
mod create;
mod sky;

use ash::vk;

pub(super) use create::生成要求;

use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 描画段階資源 {
    シーン: vulkan::pipeline::パイプライン,
    シャドウ: vulkan::pipeline::シャドウパイプライン,
    /// フレーム構成に空段階があるときだけ`Some`。有無はフレーム描画入力の空と常に一致させる。方式はこの型が持つ。
    空: Option<vulkan::sky_stage::空段階資源>,
    /// フレーム構成に空段階があるときだけ`Some`。有無はフレーム描画入力の大気媒体と常に一致させる。
    大気lut: Option<vulkan::atmosphere_lut::大気LUT一式>,
    /// フレーム構成に布シミュレーション段階があるときだけ`Some`。有無はレンダラーの布一式の有無と常に一致する(`レンダラー::生成する`の`構成と素材を検査する`が対応を検査済み)。
    布シャドウ: Option<vulkan::cloth_shadow::布シャドウ資源>,
}

impl 描画段階資源 {
    pub(super) fn 生成する(要求: 生成要求<'_>) -> Result<Self, レンダラーエラー> {
        create::生成する(要求)
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

    /// 布をシャドウパスへ記録するときの束縛先。布無しの構成では`None`を返し、布一式との有無の食い違いを呼び出し元が失敗させる。
    pub(super) fn 布シャドウ描画入力を作る(
        &self,
        フレーム添字: フレームスロット添字,
    ) -> Option<vulkan::frame::布シャドウ描画入力> {
        self.布シャドウ.as_ref().map(|布シャドウ| 布シャドウ.描画入力を作る(フレーム添字))
    }

    /// ホットリロードでシーンのパイプラインだけを入れ替える。
    /// 前提: 呼び出し元が旧パイプラインのGPU使用完了を待ってから呼ぶ(`renderer/replace_shader.rs`)。
    pub(super) fn シーンを差し替える(&mut self, device: &GPUデバイス, 新パイプライン: vulkan::pipeline::パイプライン) {
        self.シーン.破棄する(device);
        self.シーン = 新パイプライン;
    }

    /// 布シャドウ・空・大気LUT・シーン・シャドウの順に破棄する。生成の逆順で片付ける。
    /// 空を大気LUTより先に置くのは、大気LUT腕の標本ディスクリプタが大気LUTの画像ビューを結んでいるためである。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        if let Some(布シャドウ) = &self.布シャドウ {
            布シャドウ.破棄する(device);
        }
        if let Some(空) = &self.空 {
            空.破棄する(device);
        }
        if let Some(大気lut) = &self.大気lut {
            大気lut.破棄する(device);
        }
        self.シーン.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
