//! `ファイル保管庫`の楽曲の保存往復と、置き場の一覧・名乗りの食い違いの拒みを確かめる。
//! 版ごとの読み込みの挙動は兄弟の`music_storage_version_migration`が持つ。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use editor_server::{トラックの種類, プロジェクト保管庫, 楽器};

#[test]
fn 保存前は無しを返し一覧も空である() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_none");
    assert!(保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().is_none());
    assert!(保管庫.楽曲の一覧を読む().unwrap().is_empty());
}

#[test]
fn 保存して読み戻すと同じ値になる() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_roundtrip");
    保管庫.楽曲を検証して保存する(crate::common::楽曲の例()).unwrap();
    assert_eq!(
        保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap(),
        Some(crate::common::楽曲の例())
    );
}

#[test]
fn 一覧は保存済みの名乗りを昇順で返す() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_list");
    for 綴り in ["ぬ の曲", "あ の曲", "た の曲"] {
        let mut 楽曲 = crate::common::楽曲の例();
        楽曲.名乗り = crate::common::名乗り(綴り);
        保管庫.楽曲を検証して保存する(楽曲).unwrap();
    }
    let 一覧 = 保管庫.楽曲の一覧を読む().unwrap();
    assert_eq!(
        一覧,
        vec![
            crate::common::名乗り("あ の曲"),
            crate::common::名乗り("た の曲"),
            crate::common::名乗り("ぬ の曲")
        ]
    );
}

#[test]
fn 検証に落ちる楽曲は保存せず正本も作らない() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_reject");
    let mut 不正な楽曲 = crate::common::楽曲の例();
    不正な楽曲.テンポ = 10;
    assert!(保管庫.楽曲を検証して保存する(不正な楽曲).is_err());
    assert!(保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().is_none());
}

#[test]
fn ファイル名と中の名乗りが食い違う正本は読みが拒む() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_name_mismatch");
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::create_dir_all(&置き場).unwrap();
    let 本文 = serde_json::to_string_pretty(&crate::common::楽曲の例()).unwrap();
    std::fs::write(置き場.join("別の名乗り.json"), 本文).unwrap();
    assert!(保管庫.楽曲を読む(&crate::common::名乗り("別の名乗り")).is_err());
}

#[test]
fn 伴奏のトラックへ弦楽合奏を置いた楽曲は保存の往復を通る() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_instrument_range_ok");
    let mut 楽曲 = crate::common::楽曲の例();
    楽曲.トラック構成[0].種類 = トラックの種類::伴奏;
    楽曲.トラック構成[0].楽器 = 楽器::弦楽合奏;
    保管庫.楽曲を検証して保存する(楽曲.clone()).unwrap();
    assert_eq!(保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap(), Some(楽曲));
}

#[test]
fn 打楽器の行を持つトラックへグランドピアノを置いた楽曲は保存を拒む() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_instrument_range_reject");
    let mut 楽曲 = crate::common::楽曲の例();
    楽曲.トラック構成[1].楽器 = 楽器::グランドピアノ;
    assert!(保管庫.楽曲を検証して保存する(楽曲).is_err());
    assert!(保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().is_none());
}

#[test]
fn 一覧はjson以外のファイルを飛ばす() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_list_skip");
    保管庫.楽曲を検証して保存する(crate::common::楽曲の例()).unwrap();
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::write(置き場.join("覚え書き.txt"), "楽曲ではない").unwrap();
    assert_eq!(保管庫.楽曲の一覧を読む().unwrap(), vec![crate::common::名乗り("試験の楽曲")]);
}
