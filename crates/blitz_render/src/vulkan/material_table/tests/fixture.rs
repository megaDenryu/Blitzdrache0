//! 検査が共有する、GPUの実物を持たない供給元と入力の組み立て。担当するのは、常駐した画像を通し番号で代表し、
//! 何枚作って何枚退役させたかを数えることである。
//!
//! 供給元がVulkanを触らないため、世代の公開と退役の規律をフェンスの実物なしで固定できる。
//! テクスチャ生成の失敗は`テクスチャblit非対応`で代表させる(失敗の種類ではなく、失敗したときの後始末を見る検査のため)。
//! 梱包工程へ渡す材質と容量の組み立ては`material_fixture`にある。

#![allow(clippy::unwrap_used)]

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::material_table::generation_record::世代内材質レコード;
use crate::vulkan::material_table::supplier::常駐テクスチャ供給元;

pub(super) struct 検査用供給元 {
    常駐の呼び出し回数: u32,
    生きている画像: Vec<u32>,
    退役した画像: Vec<u32>,
    失敗させる呼び出し番号: Option<u32>,
}

impl 検査用供給元 {
    pub(super) fn 常に成功する() -> Self {
        Self {
            常駐の呼び出し回数: 0,
            生きている画像: Vec::new(),
            退役した画像: Vec::new(),
            失敗させる呼び出し番号: None,
        }
    }

    pub(super) fn 指定回で失敗する(呼び出し番号: u32) -> Self {
        Self {
            失敗させる呼び出し番号: Some(呼び出し番号),
            ..Self::常に成功する()
        }
    }

    pub(super) fn 生存枚数(&self) -> usize {
        self.生きている画像.len()
    }

    pub(super) fn 退役枚数(&self) -> usize {
        self.退役した画像.len()
    }
}

impl 常駐テクスチャ供給元 for 検査用供給元 {
    type 常駐画像 = u32;
    /// 束縛先はVulkanの資源であり、状態遷移の検査は束縛しないため何も持たない。
    type 世代付属資源 = ();

    fn 常駐させる(&mut self, _素材: &テクスチャ素材) -> Result<u32, レンダラーエラー> {
        self.常駐の呼び出し回数 = self.常駐の呼び出し回数.saturating_add(1);
        if self.失敗させる呼び出し番号 == Some(self.常駐の呼び出し回数) {
            return Err(レンダラーエラー::テクスチャblit非対応);
        }
        let 画像 = self.常駐の呼び出し回数;
        self.生きている画像.push(画像);
        Ok(画像)
    }

    fn 世代を仕上げる(
        &mut self, _画像集合: &[u32], _レコード列: &[世代内材質レコード]
    ) -> Result<(), レンダラーエラー> {
        Ok(())
    }

    fn 退役させる(&mut self, 画像: u32) {
        let 位置 = self.生きている画像.iter().position(|生存| *生存 == 画像).unwrap();
        self.生きている画像.remove(位置);
        self.退役した画像.push(画像);
    }

    fn 付属資源を退役させる(&mut self, _付属資源: ()) {}
}
