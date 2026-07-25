use std::path::PathBuf;
use std::time::{Duration, Instant};

use blitz_math::{大域メートル, 大域ワールド位置};

use crate::{
    asset::runtime_scene_tests::静的シーンを作る, アセットID, カタログ, シーンを実行時形式へ格納する, チャンク台帳, チャンク格子, チャンク読込エラー,
    チャンク読込器, チャンク読込完了, 実行時シーン読込エラー, 準備完了結果,
};

#[test]
fn 版付きシーンをワーカーで読み台帳へ完了を接続する() {
    let シーン = 静的シーンを作る(vec![10, 20, 30, 255], 0.2);
    let Ok(バイト列) = シーンを実行時形式へ格納する(&シーン) else {
        panic!("試験用シーンを格納できなかった");
    };
    let (パス, id, カタログ) = 試験ファイルを作る("success", &バイト列);
    let 必要集合 = 必要集合を作る();
    let チャンク = 必要集合[0].座標();
    let mut 台帳 = チャンク台帳::空を作る();
    assert!(台帳.必要集合を反映する(&必要集合).is_ok());
    let 読込器 = 起動する();
    assert!(読込器.読込を要求する(チャンク, &id, &カタログ).is_ok());
    assert_eq!(台帳.読込を開始する(チャンク), Ok(()));
    let 完了 = 完了を待つ(&読込器);
    assert_eq!(完了.チャンク(), チャンク);
    let Ok(成果) = 完了.結果() else {
        panic!("ワーカー読込が失敗した");
    };
    assert_eq!(成果.読込バイト数, バイト列.len());
    assert_eq!(成果.シーン.描画対象一覧().len(), 1);
    assert_eq!(台帳.準備完了を反映する(チャンク), Ok(準備完了結果::GPU転送待ち));
    assert!(std::fs::remove_file(パス).is_ok());
}

#[test]
fn 壊れた生成物と未登録idを型付きエラーにする() {
    let (パス, id, カタログ) = 試験ファイルを作る("broken", b"broken");
    let 読込器 = 起動する();
    assert!(読込器.読込を要求する(crate::チャンク座標::生成する(7, 0), &id, &カタログ).is_ok());
    assert!(matches!(完了を待つ(&読込器).結果(), Err(実行時シーン読込エラー::形式不正(_))));
    let Ok(未登録) = アセットID::生成する("missing") else {
        panic!("未登録試験IDを作れなかった");
    };
    assert!(matches!(
        読込器.読込を要求する(crate::チャンク座標::生成する(8, 0), &未登録, &カタログ),
        Err(チャンク読込エラー::カタログ未登録(_))
    ));
    assert!(std::fs::remove_file(パス).is_ok());
}

fn 起動する() -> チャンク読込器 {
    match チャンク読込器::起動する() {
        Ok(読込器) => 読込器,
        Err(誤り) => panic!("試験用ワーカーを起動できなかった: {誤り}"),
    }
}

fn 完了を待つ(読込器: &チャンク読込器) -> チャンク読込完了 {
    let 期限 = Instant::now() + Duration::from_secs(2);
    loop {
        match 読込器.完了を取り出す() {
            Ok(Some(完了)) => return 完了,
            Ok(None) if Instant::now() < 期限 => std::thread::yield_now(),
            Ok(None) => panic!("チャンク読込が期限内に完了しなかった"),
            Err(誤り) => panic!("チャンク読込完了を受信できなかった: {誤り}"),
        }
    }
}

fn 試験ファイルを作る(名前: &str, バイト列: &[u8]) -> (PathBuf, アセットID, カタログ) {
    let パス = PathBuf::from("target").join(format!("chunk_loader_{名前}_{}.blitzasset", std::process::id()));
    if let Err(誤り) = std::fs::write(&パス, バイト列) {
        panic!("試験用生成物を書けなかった: {誤り}");
    }
    let id = match アセットID::生成する(名前) {
        Ok(id) => id,
        Err(誤り) => panic!("試験用IDを作れなかった: {誤り}"),
    };
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), パス.clone());
    (パス, id, カタログ)
}

fn 必要集合を作る() -> Vec<crate::チャンク要求> {
    let Ok(格子) = チャンク格子::生成する(大域メートル::生成する(100.0)) else {
        panic!("試験用格子を作れなかった");
    };
    let 位置 = 大域ワールド位置::生成する(大域メートル::生成する(0.0), 大域メートル::生成する(0.0), 大域メートル::生成する(0.0));
    match 格子.必要集合を計算する(位置, 0) {
        Ok(集合) => 集合,
        Err(誤り) => panic!("試験用必要集合を作れなかった: {誤り}"),
    }
}
