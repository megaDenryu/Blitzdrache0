//! 描画対象の数に依らず1組だけ存在するディスクリプタセットの所有者。持つのはフレームスロットごとの
//! ビューとパスのセット(set0)と、1つだけの照明問い合わせのセット(set3)である。
//!
//! 束ごとに複製しないのは、どちらの内容も描画対象で変わらないためである。旧の巨大な共有セットは、
//! 描画対象数×材質スロット数×進行中フレーム数だけ同じ定数と同じシャドウマップを結び直していた
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 生成の局面は`create`が持つ。

mod create;

use ash::vk;

use super::シーンセットレイアウト一式;
use crate::error::レンダラーエラー;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::uniform::フレームシェーダー定数一式;

/// 描画発行で変わらない2つのセットを1組にした束縛先。ビューとパスのセット(set0)はそのフレームスロットのもの、
/// 照明問い合わせのセット(set3)は資源が固定のため常に同じものである。
/// パイプラインレイアウトが違うと束縛が無効になるため、パイプラインを切り替えたパスの各局面がそれぞれ1回束縛する。
#[derive(Clone, Copy)]
pub(crate) struct 共有セット束縛 {
    pub(crate) ビューとパス: vk::DescriptorSet,
    pub(crate) 照明問い合わせ: vk::DescriptorSet,
}

pub(crate) struct 共有ディスクリプタセット {
    pool: vk::DescriptorPool,
    ビューとパス一覧: [vk::DescriptorSet; 進行中フレーム数],
    照明問い合わせ: vk::DescriptorSet,
}

impl 共有ディスクリプタセット {
    pub(crate) fn 生成する(
        device: &ash::Device,
        レイアウト: &シーンセットレイアウト一式,
        シェーダー定数: &フレームシェーダー定数一式,
        シャドウマップ: &シャドウマップ,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, レイアウト, シェーダー定数, シャドウマップ)
    }

    /// そのフレームスロットのビューとパスのセット。空パスと空中遠近合成パスがset0へ束縛する。
    pub(crate) fn ビューとパス(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.ビューとパス一覧[フレーム添字.配列添字()]
    }

    /// そのフレームのシーン描画とシャドウ記録が使う束縛先の組。
    pub(crate) fn 束縛を作る(&self, フレーム添字: フレームスロット添字) -> 共有セット束縛 {
        共有セット束縛 {
            ビューとパス: self.ビューとパス(フレーム添字),
            照明問い合わせ: self.照明問い合わせ,
        }
    }

    /// 注意: プールの破棄がセットの解放を暗黙に行う。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}
