//! 資源を1つも持たないディスクリプタセットレイアウト。担当するのは、scene系のセット番号の契約を守りながら
//! 「そのパイプラインはこの役割を読まない」ことを表すことである。
//!
//! Vulkanのパイプラインレイアウトはセット番号0から連続した並びで宣言するため、set0とset3だけを読む布の描画は
//! set1とset2の位置に何かを置かざるを得ない。ここへジオメトリや材質の実レイアウトを置くと、読まない役割を
//! 宣言したことになり、束縛の契約が曖昧になる。バインドを1つも持たないレイアウトは資源を1つも要求しないため、
//! 「読まない」ことをそのまま表せる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let create_info = vk::DescriptorSetLayoutCreateInfo::default();
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}
