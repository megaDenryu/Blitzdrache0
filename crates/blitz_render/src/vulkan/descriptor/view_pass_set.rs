//! set0(ビューとパスのセット)のレイアウトと、そのセットへシェーダー定数3本を結ぶ書き込み先。触れるのは
//! ビュー定数(binding0)・多段影定数(binding1)・空パス定数(binding2)の3つだけである。
//! 3本を1つのセットへ置くのは、どれも寿命がフレーム×ビューであり束縛頻度が同じためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 番号の正本は`shaders/view_uniform.slang`・`cascade_shadow_uniform.slang`・`sky_pass_uniform.slang`の宣言である。

use ash::vk;

use super::{ディスクリプタの書き込み先, 束縛番号};
use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(crate) const ビュー定数の束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const 多段影の定数の束縛番号: 束縛番号 = 束縛番号::生成する(1);
pub(crate) const 空パスの定数の束縛番号: 束縛番号 = 束縛番号::生成する(2);

/// 1つのビューとパスのセットへ現物を書き込む先。
pub(super) struct ビューとパスのセットの書き込み先<'書き込み>(ディスクリプタの書き込み先<'書き込み>);

impl<'書き込み> ビューとパスのセットの書き込み先<'書き込み> {
    pub(super) fn 生成する(device: &'書き込み ash::Device, セット: vk::DescriptorSet) -> Self {
        Self(ディスクリプタの書き込み先::生成する(device, セット))
    }

    /// そのフレームスロットの定数3本を結ぶ。空段階を持たない構成でも空パス定数の束縛先は結ぶ(中身は書かれず誰も読まない)。
    pub(super) fn フレームの定数3本を結ぶ(
        &self, 定数: &フレームシェーダー定数一式, フレーム添字: フレームスロット添字
    ) {
        let 対応 = [
            (ビュー定数の束縛番号, 定数.ビュー定数のbuffer(フレーム添字)),
            (多段影の定数の束縛番号, 定数.多段影のbuffer(フレーム添字)),
            (空パスの定数の束縛番号, 定数.空パスのbuffer(フレーム添字)),
        ];
        for (番号, buffer) in 対応 {
            self.0.バッファ全体を結ぶ(番号, vk::DescriptorType::UNIFORM_BUFFER, buffer);
        }
    }
}

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        定数バインド(ビュー定数の束縛番号),
        定数バインド(多段影の定数の束縛番号),
        定数バインド(空パスの定数の束縛番号),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// ビュー射影行列を頂点段が、影の判定と空の視線復元を画素段が読むため、3本とも両ステージへ宣言する。
fn 定数バインド(番号: 束縛番号) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(番号.gpu境界値())
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
}
