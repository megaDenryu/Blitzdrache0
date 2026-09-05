//! 結末の締めの検査。ログを開いた後に段の子プロセスの起動が失敗したとき、その本文が道具のログに残り、
//! ログの最後の行がログのパスになることを固定する。参照: PR #83のレビューの必須1
//!
//! 端末の最後の行も同じ行になることを、この検査はログの中身で固定する。出力係は同じバイト列を同じ順で
//! 端末とログの両方へ書くため、ログの並びが端末の並びと一致するからである。試験の中から自分の端末の
//! 書き込み先を差し替えることはできないため、片方を固定して両方を言う形を採る。

use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU32, Ordering};

use super::検証列の実行係;

/// 同じ試験の実行の中でログの名前がぶつからないようにする番号。
static 使い捨てのログの通し番号: AtomicU32 = AtomicU32::new(0);

/// この名前の実行ファイルはどの経路にも無いため、段の子プロセスの起動が必ず失敗する。
const 存在しない段の実行ファイル: &str = "blitzdrache0-この名前の実行ファイルは存在しない";

fn 使い捨てのログのパスを作る() -> PathBuf {
    let 番号 = 使い捨てのログの通し番号.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("blitzdrache0-検証列の締めの検査-{}-{番号}", std::process::id()))
}

#[test]
fn 起動できない段の破れの本文と最後のログのパスがログに残る() {
    let ログのパス = 使い捨てのログのパスを作る();
    let 実行係 = 検証列の実行係::ログを開いて作る(ログのパス.clone()).unwrap();
    let mut 命令 = Command::new(存在しない段の実行ファイル);
    let 結果 = 実行係
        .段を走らせて結果を告げる("存在しない段", "存在しない段", &mut 命令)
        .map(|_| ExitCode::SUCCESS);
    let 終了コード = 実行係.結末を締めて終了コードへ写す(結果);
    let ログの中身 = std::fs::read_to_string(&ログのパス).unwrap();
    drop(実行係);
    std::fs::remove_file(&ログのパス).unwrap();
    // ExitCodeは値どうしの比較を持たないため、デバッグの綴りで突き合わせる。
    assert_eq!(
        format!("{終了コード:?}"),
        format!("{:?}", ExitCode::FAILURE),
        "内部の破れを失敗として返していない"
    );
    assert!(
        ログの中身.contains("段の子プロセスを起動できなかった"),
        "ログを開いた後の破れの本文がログの外へ逃げている: {ログの中身}"
    );
    assert_eq!(
        ログの中身.lines().last().unwrap(),
        format!("[xtask] ログ: {}", ログのパス.display()),
        "最後の行がログのパスになっていない: {ログの中身}"
    );
}
