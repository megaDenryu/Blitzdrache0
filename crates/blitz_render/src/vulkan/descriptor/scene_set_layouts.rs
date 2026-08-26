//! scene系パイプライン族が守るセット番号の契約を、実物のディスクリプタセットレイアウトとして所有する型。
//! set0がビューとパス、set1がジオメトリと可視、set2が材質、set3が照明問い合わせであり、この並びを
//! パイプラインレイアウトの宣言としてここが配る(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! シーンを差し替えても同じレイアウトを使い続けるため、束の生成より上でこの型を持つ。
//!
//! 材質テクスチャ表を読む固定サンプラーと、その表の要素数もここが持つ。固定サンプラーはそれを宣言したレイアウトより
//! 長生きしなければならず、要素数はレイアウトの一部であるため、所有者を分けると破棄順と一致の前提が散らばる。
//! `照明束縛`が持つのは照明問い合わせのセットが宣言した枝であり、資源を結ぶ側がこの値から結ぶ先を決める。
//! 生成の局面は`create`、レイアウトからセットを取り出す局面は`allocate`にある。

mod allocate;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::material_table::テクスチャ表レイアウト容量;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;
use crate::vulkan::texture::table_sampler::材質テクスチャ表のサンプラー;

pub(crate) struct シーンセットレイアウト一式 {
    材質サンプラー: 材質テクスチャ表のサンプラー,
    材質テクスチャ表容量: テクスチャ表レイアウト容量,
    照明束縛: 照明束縛レイアウト, // 照明問い合わせのセットが宣言した枝
    ビューとパス: vk::DescriptorSetLayout,
    ジオメトリ: vk::DescriptorSetLayout,
    材質: vk::DescriptorSetLayout,
    照明問い合わせ: vk::DescriptorSetLayout,
    空: vk::DescriptorSetLayout,
}

impl シーンセットレイアウト一式 {
    /// 材質テクスチャ表の要素数を受け取るのは、それがレイアウトの一部だからである。世代の内容ではこの値を変えない。
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        表容量: テクスチャ表レイアウト容量,
        照明束縛: 照明束縛レイアウト,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 表容量, 照明束縛)
    }

    pub(crate) fn 照明束縛(&self) -> 照明束縛レイアウト {
        self.照明束縛
    }

    pub(crate) fn ビューとパス(&self) -> vk::DescriptorSetLayout {
        self.ビューとパス
    }

    pub(crate) fn ジオメトリ(&self) -> vk::DescriptorSetLayout {
        self.ジオメトリ
    }

    pub(crate) fn 材質テクスチャ表容量(&self) -> テクスチャ表レイアウト容量 {
        self.材質テクスチャ表容量
    }

    /// シーン描画のパイプラインが宣言する4セット。
    pub(crate) fn シーンの並び(&self) -> [vk::DescriptorSetLayout; 4] {
        [self.ビューとパス, self.ジオメトリ, self.材質, self.照明問い合わせ]
    }

    /// シャドウ記録のパイプラインが宣言する2セット。材質も照明問い合わせも読まないため持たない。
    pub(crate) fn シャドウの並び(&self) -> [vk::DescriptorSetLayout; 2] {
        [self.ビューとパス, self.ジオメトリ]
    }

    /// 布の描画が宣言する並び。読むのはビューとパス・照明問い合わせの2つだけであり、間の2つは空のレイアウトで埋める。
    pub(crate) fn 布描画の並び(&self) -> [vk::DescriptorSetLayout; 4] {
        [self.ビューとパス, self.空, self.空, self.照明問い合わせ]
    }

    /// 注意: このレイアウトから割り当てたセットを持つディスクリプタプールをすべて破棄した後に呼ぶ。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        for handle in [self.空, self.照明問い合わせ, self.材質, self.ジオメトリ, self.ビューとパス] {
            // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
            unsafe { device.destroy_descriptor_set_layout(handle, None) };
        }
        self.材質サンプラー.破棄する(device);
    }
}
