//! 1つのセットが宣言する束縛の並び。束縛番号と記述子の種別の対を並び順に持ち、セットレイアウトの宣言・
//! プールの内訳・現物の書き込み先の3つを同じ並びから配る。
//!
//! 3つを1つの並びから配るのは、番号と種別が3箇所で別々に書かれると食い違うためである。レイアウトが宣言した種別と
//! 書き込みが指定した種別が違えば、validationがその場で落とす。プールの内訳が足りなければ割り当てが失敗する。
//!
//! 役割ごとに型を分けたシーンの4セット(`lighting_set`等)と違い、これは1つのサブシステムが自分だけで使うセットのための
//! 器である。書ける番号がそのサブシステム自身の宣言に限られるため、低水準の書き込み先を木の外へ出さずに済む。

mod bound_resource;
mod write_target;

use ash::vk;

use super::束縛番号;
pub(crate) use bound_resource::結ぶ現物;
pub(crate) use write_target::宣言どおりに結ぶ書き込み先;

/// 束縛1本の宣言。番号・記述子の種別・その束縛を読むシェーダーの段の3つで1つの契約になる。
pub(crate) type 束縛1本の宣言 = (束縛番号, vk::DescriptorType, vk::ShaderStageFlags);

#[derive(Clone, Copy)]
pub(crate) struct 宣言した束縛の並び<const 本数: usize> {
    並び: [束縛1本の宣言; 本数],
}

impl<const 本数: usize> 宣言した束縛の並び<本数> {
    pub(crate) const fn 生成する(並び: [束縛1本の宣言; 本数]) -> Self {
        Self { 並び }
    }

    /// セットレイアウトの生成へ渡すバインドの並び。要素数はどれも1であり、配列の束縛はこの器を通らない。
    pub(crate) fn セットレイアウトの宣言(&self) -> [vk::DescriptorSetLayoutBinding<'static>; 本数] {
        self.並び.map(|(番号, 種別, 段)| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(番号.gpu境界値())
                .descriptor_type(種別)
                .descriptor_count(1)
                .stage_flags(段)
        })
    }

    /// プールの生成へ渡す内訳。同じ宣言のセットを`セット数`枚配れるだけの数を種別ごとに数える。
    pub(crate) fn プールの内訳(&self, セット数: u32) -> [vk::DescriptorPoolSize; 本数] {
        self.並び
            .map(|(_, 種別, _)| vk::DescriptorPoolSize::default().ty(種別).descriptor_count(セット数))
    }

    /// この宣言で割り当てたセット1枚へ現物を書き込む先。
    /// 前提: セットはこの宣言から作ったレイアウトで割り当て済みであり、書き込みの時点でGPUがそのセットを使用していない。
    pub(crate) fn 書き込み先<'書き込み>(
        &self,
        device: &'書き込み ash::Device,
        セット: vk::DescriptorSet,
    ) -> 宣言どおりに結ぶ書き込み先<'書き込み, 本数> {
        宣言どおりに結ぶ書き込み先::生成する(device, セット, *self)
    }

    pub(super) const fn 位置の束縛(&self, 位置: usize) -> 束縛1本の宣言 {
        self.並び[位置]
    }
}
