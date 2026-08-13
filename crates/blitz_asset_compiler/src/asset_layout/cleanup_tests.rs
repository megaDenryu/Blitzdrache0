//! 出力ルートの残置物清掃と全消去の検査。

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use blitz_engine::アセットID;

use super::runtime_output_root::実行時形式の出力ルート;

fn 一時の出力ルート(名前: &str) -> (PathBuf, 実行時形式の出力ルート) {
    let パス = std::env::temp_dir().join(format!("blitz_output_cleanup_{}_{}", std::process::id(), 名前));
    let _ = std::fs::remove_dir_all(&パス);
    (パス.clone(), 実行時形式の出力ルート::作る(パス).unwrap())
}

#[test]
fn 現行対象を残して残置物だけを削除する() {
    let (パス, ルート) = 一時の出力ルート("residual");
    let id = アセットID::生成する("kept").unwrap();
    ルート.実行時カタログを書き出す(b"catalog").unwrap();
    ルート.チャンク目録を書き出す(b"directory", 0).unwrap();
    ルート.アセットを書き出す(&id, b"asset").unwrap();
    std::fs::write(パス.join("residual.blitzasset"), b"old").unwrap();
    ルート.現行対象外を削除する(std::slice::from_ref(&id)).unwrap();
    assert!(パス.join("kept.blitzasset").is_file());
    assert!(パス.join("catalog.blitzcatalog").is_file());
    assert!(!パス.join("residual.blitzasset").exists());
    std::fs::remove_dir_all(パス).unwrap();
}

#[test]
fn 全消去は出力ルート自体を残して直下を空にする() {
    let (パス, ルート) = 一時の出力ルート("all");
    std::fs::create_dir_all(パス.join("old/sub")).unwrap();
    std::fs::write(パス.join("old/sub/file"), b"old").unwrap();
    std::fs::write(パス.join("old_file"), b"old").unwrap();
    ルート.生成物一式を削除する().unwrap();
    assert!(パス.is_dir());
    assert_eq!(std::fs::read_dir(&パス).unwrap().count(), 0);
    std::fs::remove_dir_all(パス).unwrap();
}
