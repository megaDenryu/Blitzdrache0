//! 版1で焼かれたカタログが最新版のメタデータへ変換されて読めることの検査。版1の書き出しは検査だけが持つため、材料もここで作る。

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::super::super::{アセット形式版, 実行時アセットを格納する, 実行時アセット種別};
use crate::asset::runtime_format::実行時形式からカタログを読む;
use crate::asset::{アセットID, アセットメタデータ, カタログ};

fn カタログを作る(個体数: u64) -> カタログ {
    let mut カタログ = カタログ::空を作る();
    カタログ.詳細を登録する(
        アセットID::生成する("vegetation_chunk").unwrap(),
        PathBuf::from("vegetation_chunk.blitzasset"),
        vec![PathBuf::from("assets/vegetation_world/archetype.gltf")],
        アセットメタデータ {
            頂点数: 24,
            インデックス数: 36,
            テクスチャバイト数: 4,
            個体数,
        },
    );
    カタログ
}

#[test]
fn 版1のカタログを個体数0として最新版へ変換して読む() {
    let 内容 = super::write::内容を書く(&カタログを作る(64)).unwrap();
    let バイト列 = 実行時アセットを格納する(アセット形式版::V1, 実行時アセット種別::カタログ, &内容).unwrap();
    let 読み取り = 実行時形式からカタログを読む(&バイト列).unwrap();
    let id = アセットID::生成する("vegetation_chunk").unwrap();
    assert_eq!(読み取り.項目を引く(&id).unwrap().メタデータ().個体数, 0);
}

#[test]
fn 最新版は個体数を往復する() {
    let バイト列 = crate::asset::カタログを実行時形式へ格納する(&カタログを作る(64)).unwrap();
    assert_eq!(バイト列[8..12], アセット形式版::V2.番号().to_le_bytes());
    let 読み取り = 実行時形式からカタログを読む(&バイト列).unwrap();
    let id = アセットID::生成する("vegetation_chunk").unwrap();
    assert_eq!(読み取り.項目を引く(&id).unwrap().メタデータ().個体数, 64);
}
