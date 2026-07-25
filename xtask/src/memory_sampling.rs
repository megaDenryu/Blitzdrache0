//! 子プロセスを起動し、終了するまでRAM・VRAMの標本を周期採取して推移と要約を標準出力へ出す。
//! 呼出し側は実行ファイル・引数・採取間隔・制限時間を渡すだけでよく、採取の手順と打ち切りの判断はここが持つ。

mod sample;

use std::process::{Child, Command, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) struct 採取条件<'a> {
    pub(crate) 実行ファイル: &'a str,
    pub(crate) 引数一覧: &'a [&'a str],
    pub(crate) 採取間隔: Duration,
    /// これを超えても子プロセスが終わらなければ失敗として打ち切る。
    pub(crate) 制限時間: Duration,
}

enum 一巡結果 {
    継続,
    子が終了した(ExitStatus),
    採取に失敗した,
}

/// 子プロセスの標準出力は継承する。アプリ自身の終了時レポートを呼出し側の画面へそのまま出すためである。
pub(crate) fn 実行しながら採取する(条件: &採取条件<'_>) -> bool {
    let Ok(mut 子) = Command::new(条件.実行ファイル).args(条件.引数一覧).spawn() else {
        eprintln!("[xtask] {}の起動に失敗した", 条件.実行ファイル);
        return false;
    };
    let 開始 = Instant::now();
    let mut 標本一覧 = Vec::new();
    println!("経過秒,ワーキングセットMiB,プライベートMiB,専用VRAMMiB");

    let 終了状態 = loop {
        thread::sleep(条件.採取間隔);
        if 開始.elapsed() >= 条件.制限時間 {
            eprintln!("[xtask] 子プロセスが制限時間を超えたため終了する");
            return 打ち切る(&mut 子);
        }
        match 一巡する(&mut 子, 開始, &mut 標本一覧) {
            一巡結果::継続 => {}
            一巡結果::子が終了した(状態) => break 状態,
            一巡結果::採取に失敗した => return 打ち切る(&mut 子),
        }
    };

    let 要約成功 = sample::要約を表示する(&標本一覧);
    終了状態.success() && 要約成功
}

/// 採取はPowerShell呼び出しを伴い数百ミリ秒かかるため、採取の前後で子プロセスの生存を確かめる。
/// 採取中に終了していた標本は、消えたプロセスを読んだ値になるため捨てる。
fn 一巡する(子: &mut Child, 開始: Instant, 標本一覧: &mut Vec<sample::メモリ標本>) -> 一巡結果 {
    match 子.try_wait() {
        Ok(Some(状態)) => return 一巡結果::子が終了した(状態),
        Ok(None) => {}
        Err(誤り) => {
            eprintln!("[xtask] 子プロセス状態の取得に失敗: {誤り}");
            return 一巡結果::採取に失敗した;
        }
    }
    let 標本 = sample::取得する(子.id(), 開始.elapsed());
    match 子.try_wait() {
        Ok(Some(状態)) => 一巡結果::子が終了した(状態),
        Ok(None) => {
            if let Some(標本) = 標本 {
                標本.表示する();
                標本一覧.push(標本);
            }
            一巡結果::継続
        }
        Err(誤り) => {
            eprintln!("[xtask] 採取後の子プロセス状態取得に失敗: {誤り}");
            一巡結果::採取に失敗した
        }
    }
}

fn 打ち切る(子: &mut Child) -> bool {
    let _強制終了結果 = 子.kill();
    let _待機結果 = 子.wait();
    false
}
