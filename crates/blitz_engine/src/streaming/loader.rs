//! 有界要求キューで実行時シーンをワーカースレッドへ読み込む。

mod error;
mod result;
mod worker;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, TrySendError};
use std::thread::JoinHandle;

pub use error::チャンク読込エラー;
pub use result::{チャンク読込完了, チャンク読込成果};
use worker::読込ジョブ;

pub struct チャンク読込器 {
    要求送信: SyncSender<読込ジョブ>,
    完了受信: Option<Receiver<チャンク読込完了>>,
    停止: Arc<AtomicBool>,
    スレッド: Option<JoinHandle<()>>,
}

impl チャンク読込器 {
    pub fn 起動する() -> Result<Self, チャンク読込エラー> {
        worker::起動する()
    }

    pub fn 読込を要求する(
        &self,
        チャンク: crate::チャンク座標,
        アセット: &crate::アセットID,
        カタログ: &crate::カタログ,
    ) -> Result<(), チャンク読込エラー> {
        let パス = カタログ
            .パスを引く(アセット)
            .ok_or_else(|| チャンク読込エラー::カタログ未登録(アセット.clone()))?
            .to_path_buf();
        match self.要求送信.try_send(読込ジョブ { チャンク, パス }) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(_)) => Err(チャンク読込エラー::要求キュー満杯),
            Err(TrySendError::Disconnected(_)) => Err(チャンク読込エラー::ワーカー停止),
        }
    }

    pub fn 完了を取り出す(&self) -> Result<Option<チャンク読込完了>, チャンク読込エラー> {
        let 受信 = self.完了受信.as_ref().ok_or(チャンク読込エラー::ワーカー停止)?;
        match 受信.try_recv() {
            Ok(完了) => Ok(Some(完了)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(チャンク読込エラー::ワーカー停止),
        }
    }
}

impl Drop for チャンク読込器 {
    fn drop(&mut self) {
        self.完了受信.take();
        self.停止.store(true, std::sync::atomic::Ordering::Release);
        if let Some(スレッド) = self.スレッド.take() {
            let _終了結果 = スレッド.join();
        }
    }
}
