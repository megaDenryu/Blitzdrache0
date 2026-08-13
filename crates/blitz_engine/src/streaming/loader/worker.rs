//! 1本のワーカーで要求順を保ち、ファイル読込と形式検査を行う。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::asset::実行時シーンのファイル;
use crate::streaming::reset_generation::リセット世代;
use crate::チャンク座標;

use super::{result::チャンク読込成果, チャンク読込エラー, チャンク読込器, チャンク読込完了};
use crate::streaming::loader_settings::チャンク読込設定;

pub(super) struct 読込ジョブ {
    pub(super) チャンク: チャンク座標,
    pub(super) ファイル: 実行時シーンのファイル,
    /// 投入した時点のリセット世代。完了通知がそのまま持ち帰り、呼出し側が古い世代の完了を退ける根拠にする。
    pub(super) 世代: リセット世代,
}

pub(super) fn 起動する(設定: チャンク読込設定) -> Result<チャンク読込器, チャンク読込エラー> {
    let (要求送信, 要求受信) = mpsc::sync_channel::<読込ジョブ>(設定.要求キュー容量());
    let (完了送信, 完了受信) = mpsc::sync_channel(設定.完了キュー容量());
    let 停止 = Arc::new(AtomicBool::new(false));
    let 共有要求受信 = Arc::new(Mutex::new(要求受信));
    let mut スレッド一覧 = Vec::with_capacity(設定.ワーカー本数());
    for 番号 in 0..設定.ワーカー本数() {
        スレッド一覧.push(ワーカーを起動する(
            番号,
            Arc::clone(&共有要求受信),
            完了送信.clone(),
            Arc::clone(&停止),
        )?);
    }
    Ok(チャンク読込器 {
        要求送信,
        完了受信: Some(完了受信),
        停止,
        スレッド一覧,
    })
}

fn ワーカーを起動する(
    番号: usize,
    要求受信: Arc<Mutex<mpsc::Receiver<読込ジョブ>>>,
    完了送信: mpsc::SyncSender<チャンク読込完了>,
    停止: Arc<AtomicBool>,
) -> Result<std::thread::JoinHandle<()>, チャンク読込エラー> {
    std::thread::Builder::new()
        .name(format!("blitz-chunk-loader-{番号}"))
        .spawn(move || ワーカーを走らせる(&要求受信, &完了送信, &停止))
        .map_err(|誤り| チャンク読込エラー::ワーカー起動失敗(誤り.to_string()))
}

fn ワーカーを走らせる(
    要求受信: &Mutex<mpsc::Receiver<読込ジョブ>>, 完了送信: &mpsc::SyncSender<チャンク読込完了>, 停止: &AtomicBool
) {
    while !停止.load(Ordering::Acquire) {
        let Ok(受信) = 要求受信.lock() else { break };
        let 結果 = 受信.recv_timeout(Duration::from_millis(10));
        drop(受信);
        match 結果 {
            Ok(ジョブ) => {
                if 完了送信.send(読み込む(ジョブ)).is_err() {
                    break;
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn 読み込む(ジョブ: 読込ジョブ) -> チャンク読込完了 {
    let 開始 = Instant::now();
    let 結果 = ジョブ
        .ファイル
        .読み込んで読み込んだバイト数も返す()
        .map(|(シーン, 読込バイト数)| チャンク読込成果 {
            シーン,
            読込バイト数,
            所要時間: 開始.elapsed(),
        });
    チャンク読込完了::生成する(ジョブ.チャンク, ジョブ.世代, 結果)
}
