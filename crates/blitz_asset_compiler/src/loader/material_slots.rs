//! glTFのmaterial indexを材質スロットIDへ正規化する工程。受け取るのは開いた文書と焼く対象のメッシュ列、返すのはスロットの語彙と材質集合である。
//! glTFの番号をそのまま実行時形式へ持ち込まないのは、その番号が入力境界だけのものであり、焼いた後は使われない文書の並びに依存するためである。
//! 番号はプリミティブの出現順に0から振る。焼く対象のメッシュ列だけを走査するため、使わないメッシュのマテリアルを材質集合へ混ぜない。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「入力契約と実行時形式(scene v4)」

use std::path::PathBuf;

use blitz_engine::{マテリアルデータ, 材質スロットID, 材質スロット割当, 材質集合};

use super::document::開いた文書;
use super::material;
use crate::error::アセットコンパイルエラー;
use crate::texture_storage::テクスチャ格納方針;

/// スロット番号順に並んだglTFのmaterial index。宣言なし(既定マテリアル)は`None`として区別する。
pub(super) struct 材質スロット語彙 {
    番号一覧: Vec<Option<usize>>,
}

/// 語彙と材質集合と、材質が参照した外部ファイルの一覧。
pub(super) struct 材質スロットの解決結果 {
    pub(super) 語彙: 材質スロット語彙,
    pub(super) 材質集合: 材質集合,
    pub(super) 参照ファイル一覧: Vec<PathBuf>,
}

impl 材質スロット語彙 {
    pub(super) fn メッシュ列から作る(メッシュ列: &[gltf::Mesh<'_>]) -> Self {
        let mut 番号一覧: Vec<Option<usize>> = Vec::new();
        for メッシュ in メッシュ列 {
            for プリミティブ in メッシュ.primitives() {
                let 番号 = プリミティブ.material().index();
                if !番号一覧.contains(&番号) {
                    番号一覧.push(番号);
                }
            }
        }
        Self { 番号一覧 }
    }

    /// 語彙に無いマテリアルを参照するプリミティブは、語彙を作った走査に含まれていなかったことを意味する。
    pub(super) fn スロットを参照する(&self, プリミティブ: &gltf::Primitive<'_>) -> Option<材質スロットID> {
        let 番号 = プリミティブ.material().index();
        let 位置 = self.番号一覧.iter().position(|登録済み| *登録済み == 番号)?;
        u32::try_from(位置).ok().map(材質スロットID::生成する)
    }
}

/// スロットごとに材質を1回だけ読む。同じマテリアルを参照するプリミティブが何件あってもテクスチャの復号は1回である。
pub(super) fn 解決する<'文書>(
    文書: &開いた文書,
    メッシュ列: impl Iterator<Item = gltf::Mesh<'文書>>,
    方針: テクスチャ格納方針,
) -> Result<材質スロットの解決結果, アセットコンパイルエラー> {
    let メッシュ一覧: Vec<gltf::Mesh<'文書>> = メッシュ列.collect();
    let 語彙 = 材質スロット語彙::メッシュ列から作る(&メッシュ一覧);
    let mut 割当一覧: Vec<材質スロット割当> = Vec::new();
    let mut 参照ファイル一覧 = Vec::new();
    for メッシュ in &メッシュ一覧 {
        for プリミティブ in メッシュ.primitives() {
            let スロット = 語彙
                .スロットを参照する(&プリミティブ)
                .ok_or(アセットコンパイルエラー::材質スロット未登録)?;
            if 割当一覧.iter().any(|割当| 割当.スロット == スロット) {
                continue;
            }
            let (金属粗さの材質, マテリアル参照ファイル一覧) = material::マテリアルを取り出す(文書, &プリミティブ, 方針)?;
            参照ファイル一覧.extend(マテリアル参照ファイル一覧);
            割当一覧.push(材質スロット割当 {
                スロット,
                マテリアル: マテリアルデータ::金属粗さPBR(金属粗さの材質),
            });
        }
    }
    Ok(材質スロットの解決結果 {
        語彙,
        材質集合: 材質集合::生成する(割当一覧)?,
        参照ファイル一覧,
    })
}
