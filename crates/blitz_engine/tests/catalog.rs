//! カタログの登録・引き・未登録の検証。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, カタログ};

fn 試験用id(名前: &str) -> アセットID {
    match アセットID::生成する(名前) {
        Ok(id) => id,
        Err(誤り) => panic!("試験用IDの生成に失敗した: {誤り}"),
    }
}

#[test]
fn 登録したパスを引ける() {
    let id = 試験用id("quad");
    let パス = PathBuf::from("assets/smoke/quad.gltf");

    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), パス.clone());

    assert_eq!(カタログ.パスを引く(&id), Some(パス.as_path()));
}

#[test]
fn 未登録のidはnoneを返す() {
    let id = 試験用id("未登録");
    let カタログ = カタログ::空を作る();

    assert_eq!(カタログ.パスを引く(&id), None);
}

#[test]
fn 全登録を走査できる() {
    let id = 試験用id("quad");
    let パス = PathBuf::from("assets/smoke/quad.gltf");

    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), パス.clone());

    let 走査結果: Vec<(&アセットID, &Path)> = カタログ.全登録を走査する().collect();
    assert_eq!(走査結果, vec![(&id, パス.as_path())]);
}
