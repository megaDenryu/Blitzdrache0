//! scene系パイプライン族が守るセット番号の契約を、実物のディスクリプタセットレイアウトとして所有する型。
//! set0がビューとパス、set1がジオメトリと可視、set2が材質、set3が照明問い合わせであり、この並びを
//! パイプラインレイアウトの宣言としてここが配る(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! シーンを差し替えても同じレイアウトを使い続けるため、束の生成より上でこの型を持つ。

use ash::vk;

use super::{empty_set, geometry_set, lighting_set, material_set, view_pass_set};
use crate::error::レンダラーエラー;

/// 生成する5つのレイアウトの数。4つの役割に、役割を読まない位置を埋める空のレイアウトを足したものである。
const レイアウト数: usize = 5;

pub(crate) struct シーンセットレイアウト一式 {
    ビューとパス: vk::DescriptorSetLayout,
    ジオメトリ: vk::DescriptorSetLayout,
    材質: vk::DescriptorSetLayout,
    照明問い合わせ: vk::DescriptorSetLayout,
    空: vk::DescriptorSetLayout,
}

impl シーンセットレイアウト一式 {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let 一覧 = 順に生成する(device)?;
        Ok(Self {
            ビューとパス: 一覧[0],
            ジオメトリ: 一覧[1],
            材質: 一覧[2],
            照明問い合わせ: 一覧[3],
            空: 一覧[4],
        })
    }

    pub(crate) fn ビューとパス(&self) -> vk::DescriptorSetLayout {
        self.ビューとパス
    }

    pub(crate) fn ジオメトリ(&self) -> vk::DescriptorSetLayout {
        self.ジオメトリ
    }

    pub(crate) fn 材質(&self) -> vk::DescriptorSetLayout {
        self.材質
    }

    pub(crate) fn 照明問い合わせ(&self) -> vk::DescriptorSetLayout {
        self.照明問い合わせ
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
    }
}

/// 途中で失敗したら生成済みのレイアウトをその場で破棄するため、部分的に生成された一式は呼び出し元から見えない。
fn 順に生成する(device: &ash::Device) -> Result<[vk::DescriptorSetLayout; レイアウト数], レンダラーエラー> {
    type 生成手順 = fn(&ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー>;
    let 手順一覧: [生成手順; レイアウト数] = [
        view_pass_set::レイアウトを生成する,
        geometry_set::レイアウトを生成する,
        material_set::レイアウトを生成する,
        lighting_set::レイアウトを生成する,
        empty_set::レイアウトを生成する,
    ];
    let mut 一覧 = [vk::DescriptorSetLayout::null(); レイアウト数];
    for (添字, 手順) in 手順一覧.into_iter().enumerate() {
        match 手順(device) {
            Ok(値) => 一覧[添字] = 値,
            Err(誤り) => {
                for 生成済み in 一覧.iter().take(添字) {
                    // 安全性: 生成途中のレイアウトはこのスコープの唯一の所有者で、以降使用しない。
                    unsafe { device.destroy_descriptor_set_layout(*生成済み, None) };
                }
                return Err(誤り);
            }
        }
    }
    Ok(一覧)
}
