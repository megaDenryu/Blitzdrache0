//! 1本のワーカーで要求順を保ち、ファイル読込と形式検査を行う。

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};

use crate::{asset::実行時シーンファイルを読み込む, チャンク座標};

use super::{result::チャンク読込成果, チャンク読込エラー, チャンク読込器, チャンク読込完了};

const 要求上限: usize = 64;
const 完了上限: usize = 4;

pub(super) struct 読込ジョブ {
    pub(super) チャンク: チャンク座標,
    pub(super) パス: PathBuf,
}

pub(super) fn 起動する() -> Result<チャンク読込器, チャンク読込エラー> {
    let (要求送信, 要求受信) = mpsc::sync_channel::<読込ジョブ>(要求上限);
    let (完了送信, 完了受信) = mpsc::sync_channel(完了上限);
    let 停止 = Arc::new(AtomicBool::new(false));
    let ワーカー停止 = Arc::clone(&停止);
    let スレッド = std::thread::Builder::new()
        .name("blitz-chunk-loader".to_string())
        .spawn(move || {
            while !ワーカー停止.load(Ordering::Acquire) {
                match 要求受信.recv_timeout(Duration::from_millis(10)) {
                    Ok(ジョブ) => {
                        let 完了 = 読み込む(ジョブ);
                        if 完了送信.send(完了).is_err() {
                            break;
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        })
        .map_err(|誤り| チャンク読込エラー::ワーカー起動失敗(誤り.to_string()))?;
    Ok(チャンク読込器 {
        要求送信,
        完了受信: Some(完了受信),
        停止,
        スレッド: Some(スレッド),
    })
}

fn 読み込む(ジョブ: 読込ジョブ) -> チャンク読込完了 {
    let 開始 = Instant::now();
    let 結果 = 実行時シーンファイルを読み込む(&ジョブ.パス).map(|(シーン, 読込バイト数)| チャンク読込成果 {
        シーン,
        読込バイト数,
        所要時間: 開始.elapsed(),
    });
    チャンク読込完了::生成する(ジョブ.チャンク, 結果)
}
