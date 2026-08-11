//! 子プロセスを起動し、終了するまでRAM・VRAMの標本を周期採取して推移と要約を標準出力へ出す。
//! 呼出し側は起こし方・引数・採取間隔・制限時間を渡すだけでよく、採取の手順と打ち切りの判断はここが持つ。

mod sample;

use std::path::Path;
use std::process::{Child, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) use sample::メモリ最大;

pub(crate) struct 採取条件<'a> {
    pub(crate) 起こし方: crate::acceptance::アプリの起こし方,
    pub(crate) 引数一覧: &'a [&'a str],
    pub(crate) 採取間隔: Duration,
    /// これを超えても子プロセスが終わらなければ失敗として打ち切る。
    pub(crate) 制限時間: Duration,
    /// 子プロセスの標準出力の行き先。`None`なら呼出し側の画面へ継承する。パスを渡すと、同じ実行の
    /// 終了時報告を後から読める。パイプでなくファイルにするのは、採取のあいだ誰も読まないパイプが埋まると
    /// 子プロセスが書き込みで止まるためである。
    pub(crate) 標準出力先: Option<&'a Path>,
}

enum 一巡結果 {
    継続,
    子が終了した(ExitStatus),
    採取に失敗した,
}

/// 戻り値は実行中に観測した最大値であり、子プロセスが失敗したか標本を1つも採れなかったときは`None`である。
pub(crate) fn 実行しながら採取する(条件: &採取条件<'_>) -> Option<メモリ最大> {
    let Ok(mut 子) = 起動する(条件) else {
        eprintln!("[xtask] {}の起動に失敗した", 条件.起こし方.表示の綴り());
        return None;
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

    let 最大 = sample::要約を表示して最大を返す(&標本一覧);
    if 終了状態.success() { 最大 } else { None }
}

fn 起動する(条件: &採取条件<'_>) -> std::io::Result<Child> {
    let mut コマンド = 条件.起こし方.コマンドを作る();
    コマンド.args(条件.引数一覧);
    if let Some(出力先) = 条件.標準出力先 {
        コマンド.stdout(Stdio::from(std::fs::File::create(出力先)?));
    }
    コマンド.spawn()
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

fn 打ち切る(子: &mut Child) -> Option<メモリ最大> {
    let _強制終了結果 = 子.kill();
    let _待機結果 = 子.wait();
    None
}
