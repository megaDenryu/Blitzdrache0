//! 資源表世代のテクスチャをVulkanの画像として常駐させ、世代1つぶんの材質のセットを仕上げる本番の供給元。
//! 担当するのは、既存のテクスチャ生成経路と材質のセットの確保を、世代の構築と退役から呼べる1つの口へ束ねることである。
//!
//! 前提: 借りている論理デバイス・物理デバイスへの問い合わせ・転送実行環境・セットレイアウトは、この供給元より長生きする
//! (レンダラーの破棄順は`renderer/destroy.rs`が持ち、転送実行環境の破棄はこの供給元を使い終えた後である)。
//! 世代1つぶんの束縛先の確保と解放は`generation_binding`が持つ。

mod generation_binding;

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::texture::テクスチャ;

use super::generation_record::世代内材質レコード;
use super::resource_table::材質資源の作業環境;
use super::supplier::常駐テクスチャ供給元;

pub(in crate::vulkan::material_table) use generation_binding::世代束縛資源;

pub(in crate::vulkan::material_table) struct デバイス常駐供給元<'環境> {
    環境: 材質資源の作業環境<'環境>,
}

impl<'環境> デバイス常駐供給元<'環境> {
    pub(in crate::vulkan::material_table) fn 生成する(環境: 材質資源の作業環境<'環境>) -> Self {
        Self { 環境 }
    }
}

impl 常駐テクスチャ供給元 for デバイス常駐供給元<'_> {
    type 常駐画像 = テクスチャ;
    type 世代付属資源 = 世代束縛資源;

    fn 常駐させる(&mut self, 素材: &テクスチャ素材) -> Result<テクスチャ, レンダラーエラー> {
        テクスチャ::生成する(self.環境.確保係, self.環境.問い合わせ, self.環境.転送環境, 素材)
    }

    fn 世代を仕上げる(
        &mut self,
        画像集合: &[テクスチャ],
        レコード列: &[世代内材質レコード],
    ) -> Result<世代束縛資源, レンダラーエラー> {
        世代束縛資源::生成する(self.環境, 画像集合, レコード列)
    }

    fn 退役させる(&mut self, 画像: テクスチャ) {
        画像.破棄する(self.環境.論理デバイス());
    }

    fn 付属資源を退役させる(&mut self, 付属資源: 世代束縛資源) {
        付属資源.破棄する(self.環境.論理デバイス());
    }
}
