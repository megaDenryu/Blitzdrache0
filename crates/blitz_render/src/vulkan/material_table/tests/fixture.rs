//! 検査が共有する、GPUの実物を持たない供給元と入力の組み立て。担当するのは、常駐した画像を通し番号で代表し、
//! 何枚作って何枚退役させたかを数えることである。
//!
//! 供給元がVulkanを触らないため、世代の公開と退役の規律をフェンスの実物なしで固定できる。
//! テクスチャ生成の失敗は`テクスチャblit非対応`で代表させる(失敗の種類ではなく、失敗したときの後始末を見る検査のため)。

#![allow(clippy::unwrap_used)]

use crate::descriptor_indexing_limits::ディスクリプタ索引上限;
use crate::error::レンダラーエラー;
use crate::texture_material::{テクスチャ用途, テクスチャ素材};
use crate::vulkan::material_table::capacity::テクスチャ表レイアウト容量;
use crate::vulkan::material_table::stage_reserve::画素段の予約枠;
use crate::vulkan::material_table::supplier::常駐テクスチャ供給元;
use crate::vulkan::material_table::{
    image_id::画像ID, material_id::大域材質ID, pack_input::梱包対象材質, texture_id::テクスチャID, texture_spec::テクスチャ指定,
};

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

    fn 常駐させる(&mut self, _素材: &テクスチャ素材) -> Result<u32, レンダラーエラー> {
        self.常駐の呼び出し回数 = self.常駐の呼び出し回数.saturating_add(1);
        if self.失敗させる呼び出し番号 == Some(self.常駐の呼び出し回数) {
            return Err(レンダラーエラー::テクスチャblit非対応);
        }
        let 画像 = self.常駐の呼び出し回数;
        self.生きている画像.push(画像);
        Ok(画像)
    }

    fn 退役させる(&mut self, 画像: u32) {
        let 位置 = self.生きている画像.iter().position(|生存| *生存 == 画像).unwrap();
        self.生きている画像.remove(位置);
        self.退役した画像.push(画像);
    }
}

pub(super) fn 検査用素材(用途: テクスチャ用途) -> テクスチャ素材 {
    テクスチャ素材::生成する(1, 1, vec![1, 2, 3, 4], 用途).unwrap()
}

/// 材質テクスチャ表の枚数が実機の上限でも要望でも頭打ちにならない、余裕のあるレイアウト容量。
pub(super) fn 余裕のあるレイアウト容量() -> テクスチャ表レイアウト容量 {
    let 上限 = ディスクリプタ索引上限::生成する(1_000, 1_000, 1_000);
    テクスチャ表レイアウト容量::決める(上限, 画素段の予約枠::現行のシーン画素段(), 64).unwrap()
}

/// ベースカラーだけを持ちうる材質。テクスチャIDと画像IDを材質番号から作るため、材質ごとに別の画像になる。
pub(super) fn 材質を作る<'素材>(番号: u64, ベースカラー: Option<&'素材 テクスチャ素材>) -> 梱包対象材質<'素材> {
    let 指定 = ベースカラー.map(|素材| テクスチャ指定::生成する(テクスチャID::生成する(番号), 画像ID::生成する(番号), 素材));
    梱包対象材質::生成する(大域材質ID::生成する(番号), [1.0, 1.0, 1.0, 1.0], 0.25, 0.75, [指定, None, None])
}

/// テクスチャIDと画像IDを別々に選べるベースカラー付きの材質。重複除去と衝突の検査が使う。
pub(super) fn 画像を選んだ材質<'素材>(
    材質番号: u64,
    テクスチャ番号: u64,
    画像番号: u64,
    素材: &'素材 テクスチャ素材,
) -> 梱包対象材質<'素材> {
    let 指定 = テクスチャ指定::生成する(テクスチャID::生成する(テクスチャ番号), 画像ID::生成する(画像番号), 素材);
    梱包対象材質::生成する(大域材質ID::生成する(材質番号), [1.0, 1.0, 1.0, 1.0], 0.0, 1.0, [Some(指定), None, None])
}
