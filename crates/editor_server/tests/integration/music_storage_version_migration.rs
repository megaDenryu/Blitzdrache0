//! `ファイル保管庫`が楽曲の版ごとの正本(形式版を持たない旧版・形式版1・現在の形式版2・未対応の新しい版)を
//! 読むときの挙動を確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

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
fn 形式版1の正本はパターンへ既定の小節数を補って読める() {
    let (一時, 保管庫) = crate::common::保管庫を作る("music_format_version_1");
    let mut 形式版1のjson = serde_json::to_value(crate::common::楽曲の例()).unwrap();
    形式版1のjson["形式版"] = serde_json::json!(1);
    for パターン in 形式版1のjson["パターン一覧"].as_array_mut().unwrap() {
        パターン.as_object_mut().unwrap().remove("小節数").unwrap();
    }
    let 置き場 = 一時.ルート().join("editor_data").join("楽曲");
    std::fs::create_dir_all(&置き場).unwrap();
    std::fs::write(置き場.join("試験の楽曲.json"), serde_json::to_string_pretty(&形式版1のjson).unwrap()).unwrap();

    let 読んだ楽曲 = 保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().unwrap();
    assert_eq!(読んだ楽曲.形式版, editor_server::楽曲の現在の形式版);
    assert_eq!(読んだ楽曲.パターン一覧[0].小節数, editor_server::新しいパターンの既定の小節数);
    assert_eq!(読んだ楽曲, crate::common::楽曲の例());
}

#[test]
fn 現在の形式版2の正本はそのまま読める() {
    let (_一時, 保管庫) = crate::common::保管庫を作る("music_format_version_2");
    保管庫.楽曲を検証して保存する(crate::common::楽曲の例()).unwrap();
    let 読んだ楽曲 = 保管庫.楽曲を読む(&crate::common::名乗り("試験の楽曲")).unwrap().unwrap();
    assert_eq!(読んだ楽曲.形式版, 2);
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
