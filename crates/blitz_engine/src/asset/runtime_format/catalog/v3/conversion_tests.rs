//! 版3から最新版への変換検査。

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::super::super::{アセット形式版, 実行時アセットを格納する, 実行時アセット種別};
use crate::asset::{アセットID, アセットメタデータ, カタログ, 世界の由来, 実行時形式からカタログを読む};

#[test]
fn 版3を由来なし高さ場標本数0の最新版へ変換する() {
    let mut カタログ = カタログ::空を作る();
    let id = アセットID::生成する("height_field").unwrap();
    カタログ.詳細を登録する(id.clone(), PathBuf::from("height.blitzasset"), Vec::new(), アセットメタデータ::default());
    let 内容 = super::write::カタログ内容を書く(&カタログ).unwrap();
    let バイト列 = 実行時アセットを格納する(アセット形式版::V3, 実行時アセット種別::カタログ, &内容).unwrap();
    let 読み取り = 実行時形式からカタログを読む(&バイト列).unwrap();
    assert_eq!(読み取り.世界の由来(), 世界の由来::生成による由来を持たない);
    assert_eq!(読み取り.項目を参照する(&id).unwrap().メタデータ().高さ場の標本数.値(), 0);
}
