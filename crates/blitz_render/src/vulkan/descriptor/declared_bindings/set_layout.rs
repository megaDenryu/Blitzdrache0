//! 宣言した束縛の並びそのものを、1つのVkDescriptorSetLayoutとして所有する資源型。宣言を一緒に持ち回るため、
//! このレイアウトから割り当てたセットには必ず同じ宣言が刻まれる。
//!
//! レイアウトを宣言から切り離さないのは、別の宣言で作ったレイアウトのセットへこの宣言の番号を書く呼び出しを、
//! 型検査で成立させないためである。生のレイアウトのハンドルを受け取る割り当ての口は持たない。
//!
//! 注意: `Drop`を持たない。破棄の順番は、このレイアウトから割り当てたセットを持つプールの持ち主が決める。

use ash::vk;

use super::{宣言から割り当てたセット, 宣言した束縛の並び};
use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;

pub(crate) struct 宣言から作ったセットレイアウト<const 本数: usize> {
    宣言: 宣言した束縛の並び<本数>,
    レイアウト: vk::DescriptorSetLayout,
}

impl<const 本数: usize> 宣言から作ったセットレイアウト<本数> {
    pub(super) fn 確保する(device: &ash::Device, 宣言: 宣言した束縛の並び<本数>) -> Result<Self, レンダラーエラー> {
        let バインド一覧 = 宣言.セットレイアウトの宣言();
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
        // 安全性: deviceは生成済みで有効。create_infoは本メソッド内で構築した値のみを参照する。
        let レイアウト = unsafe { device.create_descriptor_set_layout(&create_info, None)? };
        Ok(Self { 宣言, レイアウト })
    }

    /// パイプラインレイアウトの宣言へ渡す境界。ここから先はVulkanの生のハンドルであり、この木へ戻る口は無い。
    pub(crate) const fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.レイアウト
    }

    /// このレイアウトのセットを、呼び出し元が用意したプールから要求の数だけ取り出す。
    /// 前提: プールはこの宣言の`プールの内訳`で数えた容量を持つ。足りなければ割り当てが型付きの失敗として返る。
    pub(crate) fn プールからセットを割り当てる(
        &self,
        device: &ash::Device,
        pool: vk::DescriptorPool,
        セット数: usize,
    ) -> Result<Vec<宣言から割り当てたセット<本数>>, レンダラーエラー> {
        if セット数 == 0 {
            return Ok(Vec::new());
        }
        let レイアウト一覧 = vec![self.レイアウト; セット数];
        let 割当情報 = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&レイアウト一覧);
        // 安全性: poolとレイアウトはどちらも生成済みで有効である。
        let 一覧 = unsafe { device.allocate_descriptor_sets(&割当情報)? };
        if 一覧.len() != セット数 {
            panic!("allocate_descriptor_setsが要求{セット数}個と異なる件数を返した");
        }
        Ok(一覧.into_iter().map(|セット| 宣言から割り当てたセット::刻む(self.宣言, セット)).collect())
    }

    /// 進行中フレームスロットごとに1枚ずつセットを取り出す。スロットの数と割り当ての件数を1箇所で結び付けるため、
    /// スロットを増やしたときに件数だけが取り残されない。
    pub(crate) fn 進行中フレームスロットごとのセットを割り当てる(
        &self,
        device: &ash::Device,
        pool: vk::DescriptorPool,
    ) -> Result<[宣言から割り当てたセット<本数>; 進行中フレーム数], レンダラーエラー> {
        let 一覧 = self.プールからセットを割り当てる(device, pool, 進行中フレーム数)?;
        match <[宣言から割り当てたセット<本数>; 進行中フレーム数]>::try_from(一覧) {
            Ok(組) => Ok(組),
            Err(_) => panic!("割り当てたセットの件数が進行中フレーム数と一致しない"),
        }
    }

    /// 前提: このレイアウトから割り当てたセットを持つプールをすべて破棄した後に呼ぶ。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: レイアウトはSelfが唯一の所有者である。
        unsafe { device.destroy_descriptor_set_layout(self.レイアウト, None) };
    }
}
