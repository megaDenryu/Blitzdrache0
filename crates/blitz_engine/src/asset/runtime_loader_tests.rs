use std::path::PathBuf;

use super::{
    runtime_scene_tests::静的シーンを作る, アセットID, カタログ, シーンを実行時形式へ格納する, 実行時シーンを読み込む, 実行時シーン読込エラー,
};

#[test]
fn カタログの実行時形式を読み生成物だけを監視対象にする() {
    let mut 期待 = 静的シーンを作る(vec![10, 20, 30, 255], 0.2);
    let Ok(バイト列) = シーンを実行時形式へ格納する(&期待) else {
        panic!("試験用シーンを実行時形式へ格納できなかった");
    };
    let パス = PathBuf::from("target").join(format!("runtime_loader_test_{}.blitzasset", std::process::id()));
    if let Err(誤り) = std::fs::create_dir_all("target") {
        panic!("試験用ディレクトリを作れなかった: {誤り}");
    }
    if let Err(誤り) = std::fs::write(&パス, バイト列) {
        panic!("試験用実行時アセットを書けなかった: {誤り}");
    }
    let Ok(id) = アセットID::生成する("runtime_loader_test") else {
        panic!("試験用アセットIDを作れなかった");
    };
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), パス.clone());
    let 結果 = 実行時シーンを読み込む(&カタログ, &id);
    let 削除結果 = std::fs::remove_file(&パス);
    assert!(削除結果.is_ok(), "試験用実行時アセットを削除できなかった");

    期待.参照ファイル一覧.push(パス);
    assert_eq!(結果.map_err(|誤り| 誤り.to_string()), Ok(期待));
}

#[test]
fn 未登録idを型付きエラーにする() {
    let Ok(id) = アセットID::生成する("missing") else {
        panic!("試験用アセットIDを作れなかった");
    };
    let 結果 = 実行時シーンを読み込む(&カタログ::空を作る(), &id);
    assert!(matches!(結果, Err(実行時シーン読込エラー::カタログ未登録(_))));
}
