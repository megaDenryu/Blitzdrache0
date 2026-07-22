//! 版1カタログ内容を境界検査し、安定IDの重複を拒否して復元する。

use std::collections::HashSet;
use std::path::PathBuf;

use super::super::scene_v1::bytes::読取位置;
use super::super::アセット実行時形式エラー;
use crate::asset::{asset_metadata::アセットメタデータ, catalog::カタログ, id::アセットID};

const 項目最小長: usize = 38;

pub(super) fn 内容を読む(内容: &[u8]) -> Result<カタログ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 項目数 = 入力.件数(項目最小長)?;
    let mut id一覧 = HashSet::with_capacity(項目数);
    let mut カタログ = カタログ::空を作る();
    for _ in 0..項目数 {
        let id文字列 = 文字列を読む(&mut 入力)?.to_string();
        if !id一覧.insert(id文字列.clone()) {
            return Err(アセット実行時形式エラー::カタログID重複);
        }
        let id = アセットID::生成する(&id文字列).map_err(|_| アセット実行時形式エラー::不正な文字列)?;
        let 実行時パス = PathBuf::from(文字列を読む(&mut 入力)?);
        let 依存数 = 入力.件数(5)?;
        let mut 依存一覧 = Vec::with_capacity(依存数);
        for _ in 0..依存数 {
            依存一覧.push(PathBuf::from(文字列を読む(&mut 入力)?));
        }
        let メタデータ = アセットメタデータ {
            頂点数: 入力.u64()?,
            インデックス数: 入力.u64()?,
            テクスチャバイト数: 入力.u64()?,
        };
        カタログ.詳細を登録する(id, 実行時パス, 依存一覧, メタデータ);
    }
    入力.完了を検査する()?;
    Ok(カタログ)
}

fn 文字列を読む<'a>(入力: &mut 読取位置<'a>) -> Result<&'a str, アセット実行時形式エラー> {
    let 長さ = 入力.usize()?;
    std::str::from_utf8(入力.バイト列(長さ)?).map_err(|_| アセット実行時形式エラー::不正な文字列)
}
