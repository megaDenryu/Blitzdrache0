//! `ファイル保管庫`の楽曲の保存往復と、置き場の一覧・名乗りの食い違いの拒みを確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use editor_server::プロジェクト保管庫;

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
fn 一覧はjson以外のファイルを飛ばす() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_list_skip");
    保管庫.楽曲を検証して保存する(crate::common::楽曲の例()).unwrap();
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::write(置き場.join("覚え書き.txt"), "楽曲ではない").unwrap();
    assert_eq!(保管庫.楽曲の一覧を読む().unwrap(), vec![crate::common::名乗り("試験の楽曲")]);
}

#[test]
fn 形式版の欄を持たない旧版の正本は現在の形へ写して読める() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_legacy_version");
    let mut 旧版のjson = serde_json::to_value(crate::common::楽曲の例()).unwrap();
    assert!(旧版のjson.as_object_mut().unwrap().remove("形式版").is_some());
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::create_dir_all(&置き場).unwrap();
    std::fs::write(置き場.join("試験の楽曲.json"), serde_json::to_string_pretty(&旧版のjson).unwrap()).unwrap();

    let 読んだ楽曲 = 保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().unwrap();
    assert_eq!(読んだ楽曲.形式版, editor_server::楽曲の現在の形式版);
    assert_eq!(読んだ楽曲, crate::common::楽曲の例());
}

#[test]
fn 現在より新しい形式版の正本は読みが拒む() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_future_version");
    let mut 新しい版のjson = serde_json::to_value(crate::common::楽曲の例()).unwrap();
    新しい版のjson["形式版"] = serde_json::json!(editor_server::楽曲の現在の形式版 + 1);
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::create_dir_all(&置き場).unwrap();
    std::fs::write(置き場.join("試験の楽曲.json"), serde_json::to_string_pretty(&新しい版のjson).unwrap()).unwrap();

    assert!(保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).is_err());
}
