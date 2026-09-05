//! エディター世界の16チャンクが、版2の実行時目録に保持した256メートルの一辺から大域原点を導くことを検査する。
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use blitz_asset_compiler::チャンク目録ソースを読み込む;
use blitz_engine::{チャンク目録, チャンク目録を実行時形式へ格納する, 実行時形式からチャンク目録を読む};

#[test]
fn 十六チャンクの原点を実行時目録の一辺から導出する() {
    let パス = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/editor_world/chunk_directory.txt");
    let ソース = チャンク目録ソースを読み込む(&パス).unwrap();
    let mut 目録 = チャンク目録::空を作る(ソース.一辺());
    for 項目 in ソース.項目一覧() {
        assert!(パス.parent().unwrap().join(項目.ソース相対パス()).is_file());
        目録.登録する(項目.チャンク(), 項目.アセット().clone()).unwrap();
    }
    let バイト列 = チャンク目録を実行時形式へ格納する(&目録).unwrap();
    let 復元 = 実行時形式からチャンク目録を読む(&バイト列).unwrap();
    assert_eq!(復元.件数(), 16);
    assert_eq!(復元.一辺().f32値(), 256.0);
    for (座標, _) in 復元.全登録を走査する() {
        let 原点 = 座標.大域原点を求める(復元.一辺().大域メートルへ変換する());
        assert_eq!(原点.x().値(), f64::from(座標.x()) * 256.0);
        assert_eq!(原点.z().値(), f64::from(座標.z()) * 256.0);
    }
}
