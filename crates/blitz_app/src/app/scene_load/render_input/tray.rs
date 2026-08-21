//! 束の描画入力を積み上げる受け皿。描画対象素材・可視判定の材料・プリミティブ描画項目の3つを持ち、描画対象1つぶんを3つへ同時に積む。
//! 3つを1つの型が持つのは、どれもが束の中での描画対象の並び順を共通の識別に使い、別々に積むと並びが食い違いうるためである。
//! 触れるのはこの3つの列だけであり、走査順の決定も配置の複製もここには無い。

use blitz_engine::primitive_draw_item::プリミティブ描画項目一覧;
use blitz_engine::描画対象データ;
use blitz_math::{ローカル, ワールド, 変換, 大域ワールド位置};
use blitz_render::{描画シーン素材, 描画対象素材};

use super::super::surface_layer_tiles::地表の層のタイル一式;
use super::{convert, instance_transforms, material_slots, visibility_material};
use super::{束の描画入力, 束の登録一式};
use crate::app::visibility::群可視材料の登録;
use crate::error::起動エラー;

pub(super) struct 変換の受け皿 {
    素材一覧: Vec<描画対象素材>,
    可視材料一覧: Vec<群可視材料の登録>,
    プリミティブ描画項目一覧: プリミティブ描画項目一覧,
}

impl 変換の受け皿 {
    pub(super) fn 生成する(描画対象数: usize) -> Self {
        Self {
            素材一覧: Vec::with_capacity(描画対象数),
            可視材料一覧: Vec::new(),
            プリミティブ描画項目一覧: プリミティブ描画項目一覧::生成する(),
        }
    }

    /// `位置`は束の中での並び順であり、可視個体選択とプリミティブ描画項目の可視個体区間参照が描画対象を指す添字と同じ値である。
    pub(super) fn 積む(
        &mut self,
        元: &描画対象データ,
        位置: usize,
        大域の基準原点: 大域ワールド位置,
        ローカルからワールド: 変換<ローカル, ワールド>,
        地表の層のタイル: &地表の層のタイル一式,
    ) -> Result<(), 起動エラー> {
        self.プリミティブ描画項目一覧
            .描画対象1つぶんを積む(元.形状(), 元.材質集合(), 位置)
            .map_err(super::描画入力エラー::from)?;
        self.素材一覧.push(描画対象素材::生成する(
            大域の基準原点,
            instance_transforms::組み立てる(元.形状(), ローカルからワールド),
            convert::形状を変換する(元.形状()),
            material_slots::変換する(元.材質集合(), 地表の層のタイル)?,
        ));
        if let Some(登録) = visibility_material::群可視材料の登録を作る(元.形状(), 位置, 大域の基準原点, ローカルからワールド)?
        {
            self.可視材料一覧.push(登録);
        }
        Ok(())
    }

    /// 積み終えた3つを束の描画入力へ閉じる。空の描画シーンをVulkan生成境界へ渡さないため、先頭を別扱いにして取り出す。
    pub(super) fn 仕上げる(self) -> 束の描画入力 {
        let mut 反復 = self.素材一覧.into_iter();
        let 先頭 = match 反復.next() {
            Some(描画対象) => 描画対象,
            None => panic!("シーンデータは1つ以上の描画対象を持つ不変条件に違反した"),
        };
        束の描画入力 {
            描画シーン: 描画シーン素材::生成する(先頭, 反復.collect()),
            登録一式: 束の登録一式 {
                可視材料一覧: self.可視材料一覧,
                プリミティブ描画項目一覧: self.プリミティブ描画項目一覧,
            },
        }
    }
}
