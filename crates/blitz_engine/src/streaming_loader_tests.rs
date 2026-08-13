use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::{
    asset::runtime_scene_tests::静的シーンを作る, streaming_ledger_tests::候補集合を作る, アセットID, カタログ, シーンを実行時形式へ格納する,
    チャンク台帳, チャンク読込エラー, チャンク読込器, チャンク読込完了, チャンク読込設定, リセット世代, 実行時シーン読込エラー, 準備完了結果,
};

mod setting_tests;

#[test]
fn 版付きシーンをワーカーで読み台帳へ完了を接続する() {
    let シーン = 静的シーンを作る(vec![10, 20, 30, 255], 0.2);
    let Ok(バイト列) = シーンを実行時形式へ格納する(&シーン) else {
        panic!("試験用シーンを格納できなかった");
    };
    let (パス, id, カタログ) = 試験ファイルを作る("success", &バイト列);
    let 候補集合 = 候補集合を作る(0.0, 0);
    let チャンク = 候補集合[0].要求().座標();
    let mut 台帳 = チャンク台帳::空を作る();
    assert!(台帳.必要集合を反映する(&候補集合).is_ok());
    let 読込器 = 起動する();
    let 世代 = リセット世代::最初を作る();
    assert!(読込器.読込を要求する(チャンク, &id, &カタログ, 世代).is_ok());
    assert_eq!(台帳.読込を開始する(チャンク), Ok(()));
    let 完了 = 完了を待つ(&読込器);
    assert_eq!(完了.チャンク(), チャンク);
    // 完了は投入時の世代をそのまま持ち帰る。リセットの前後で同じ座標へ並んだ2件を呼出し側が見分ける唯一の手掛かりである。
    assert_eq!(完了.世代(), 世代);
    let Ok(成果) = 完了.結果() else {
        panic!("ワーカー読込が失敗した");
    };
    assert_eq!(成果.読込バイト数, バイト列.len());
    assert_eq!(成果.シーン.描画対象一覧().len(), 1);
    let Ok(ram実測バイト数) = u64::try_from(成果.読込バイト数) else {
        panic!("読込バイト数をu64へ変換できなかった");
    };
    assert_eq!(台帳.準備完了を反映する(チャンク, ram実測バイト数), Ok(準備完了結果::GPU転送待ち));
    assert!(std::fs::remove_file(パス).is_ok());
}

#[test]
fn 壊れた生成物と未登録idを型付きエラーにする() {
    let (パス, id, カタログ) = 試験ファイルを作る("broken", b"broken");
    let 読込器 = 起動する();
    let 世代 = リセット世代::最初を作る();
    assert!(読込器.読込を要求する(crate::チャンク座標::生成する(7, 0), &id, &カタログ, 世代).is_ok());
    assert!(matches!(完了を待つ(&読込器).結果(), Err(実行時シーン読込エラー::形式不正(_))));
    let Ok(未登録) = アセットID::生成する("missing") else {
        panic!("未登録試験IDを作れなかった");
    };
    assert!(matches!(
        読込器.読込を要求する(crate::チャンク座標::生成する(8, 0), &未登録, &カタログ, 世代),
        Err(チャンク読込エラー::カタログ未登録(_))
    ));
    assert!(std::fs::remove_file(パス).is_ok());
}

fn 起動する() -> チャンク読込器 {
    設定で起動する(チャンク読込設定::既定値())
}

fn 設定で起動する(設定: チャンク読込設定) -> チャンク読込器 {
    match チャンク読込器::起動する(設定) {
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
