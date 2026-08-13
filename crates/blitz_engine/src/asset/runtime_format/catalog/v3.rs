//! 版3のカタログと最新版への変換。版3は世界の由来と高さ場標本数を持たない。

#[cfg(test)]
mod conversion_tests;
#[cfg(test)]
mod write;

use std::collections::HashSet;
use std::path::PathBuf;

use super::super::scene::bytes::読取位置;
use super::super::アセット実行時形式エラー;
use super::bytes;
use crate::asset::{アセットID, アセットメタデータ, カタログ, 高さ場の標本数};

struct カタログ項目V3 {
    id: アセットID,
    実行時パス: PathBuf,
    依存一覧: Vec<PathBuf>,
    メタデータ: アセットメタデータ,
}

struct カタログV3(Vec<カタログ項目V3>);

impl カタログV3 {
    fn 最新へ変換する(self) -> カタログ {
        let mut 最新 = カタログ::空を作る();
        for 項目 in self.0 {
            最新.詳細を登録する(項目.id, 項目.実行時パス, 項目.依存一覧, 項目.メタデータ);
        }
        最新
    }
}

const 項目最小長: usize = bytes::メタデータ以外の項目最小長 + 32;

pub(super) fn カタログ内容を読む(内容: &[u8]) -> Result<カタログ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 項目数 = 入力.件数(項目最小長)?;
    let mut id一覧 = HashSet::with_capacity(項目数);
    let mut 項目一覧 = Vec::with_capacity(項目数);
    for _ in 0..項目数 {
        let (id文字列, id, 実行時パス, 依存一覧) = bytes::項目本体を読む(&mut 入力)?;
        if !id一覧.insert(id文字列) {
            return Err(アセット実行時形式エラー::カタログID重複);
        }
        項目一覧.push(カタログ項目V3 {
            id,
            実行時パス,
            依存一覧,
            メタデータ: アセットメタデータ {
                頂点数: 入力.u64()?,
                インデックス数: 入力.u64()?,
                テクスチャ格納バイト数: 入力.u64()?,
                個体数: 入力.u64()?,
                高さ場の標本数: 高さ場の標本数::生成する(0),
            },
        });
    }
    入力.完了を検査する()?;
    Ok(カタログV3(項目一覧).最新へ変換する())
}
