//! 版2で焼かれたカタログが最新版のメタデータへ変換されて読めることの検査。版2の書き出しは検査だけが持つため、材料もここで作る。
//! 版2の展開済みテクスチャバイト数と版3のテクスチャ格納バイト数は、全テクスチャが非圧縮だった版2では同じ値であるため、変換は恒等である。

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::super::super::{アセット形式版, 実行時アセットを格納する, 実行時アセット種別};
use crate::asset::runtime_format::実行時形式からカタログを読む;
use crate::asset::{アセットID, アセットメタデータ, カタログ};

fn カタログを作る() -> カタログ {
    let mut カタログ = カタログ::空を作る();
    カタログ.詳細を登録する(
        アセットID::生成する("village_chunk").unwrap(),
        PathBuf::from("village_chunk.blitzasset"),
        vec![PathBuf::from("assets/village/prop.gltf")],
        アセットメタデータ {
            頂点数: 24,
            インデックス数: 36,
            テクスチャ格納バイト数: 64,
            個体数: 8,
        },
    );
    カタログ
}

#[test]
fn 版2のカタログのテクスチャ格納バイト数をそのまま最新版へ写して読む() {
    let 内容 = super::write::内容を書く(&カタログを作る()).unwrap();
    let バイト列 = 実行時アセットを格納する(アセット形式版::V2, 実行時アセット種別::カタログ, &内容).unwrap();
    let 読み取り = 実行時形式からカタログを読む(&バイト列).unwrap();
    let id = アセットID::生成する("village_chunk").unwrap();
    let メタデータ = 読み取り.項目を参照する(&id).unwrap().メタデータ();
    assert_eq!(メタデータ.テクスチャ格納バイト数, 64);
    assert_eq!(メタデータ.個体数, 8);
}

#[test]
fn 版2と最新版で先頭のヘッダー形式版が異なり並びは同じである() {
    let 版2の内容 = super::write::内容を書く(&カタログを作る()).unwrap();
    let 最新版 = crate::asset::カタログを実行時形式へ格納する(&カタログを作る()).unwrap();
    assert_eq!(最新版[8..12], アセット形式版::V3.番号().to_le_bytes());
    assert_eq!(最新版[24..], 版2の内容);
}
