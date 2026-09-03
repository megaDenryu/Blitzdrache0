//! 起動した子プロセス1本と、その子孫までを含む木。担当するのは、生死の確認と木ごとの終了である。
//! `Child`を呼び出し側へ直に配らないのは、終わらせ方(Windowsでは子孫が道連れにならない)を
//! この型の中の1箇所へ閉じるためである。

use std::process::{Child, ExitStatus};

use super::process_id::プロセス番号;

pub(crate) struct 子プロセスの木 {
    本体: Child,
}

impl 子プロセスの木 {
    pub(crate) fn 起動済みの子から作る(本体: Child) -> Self {
        Self { 本体 }
    }

    pub(crate) fn 番号(&self) -> プロセス番号 {
        プロセス番号::起動済みの子から得る(&self.本体)
    }

    /// 既に終わっていればその終了状態を返し、まだ動いていれば`None`を返す。
    /// 待ち合わせの問い合わせ自体が失敗した場合も`None`として扱い、次の巡回で改めて問う。
    pub(crate) fn 終わっていれば終了状態を返す(&mut self) -> Option<ExitStatus> {
        self.本体.try_wait().ok().flatten()
    }

    /// 境界: 道連れの束へ加えるために、Windowsが子へ割り当てた取っ手(HANDLE)の番地を貸す。
    /// 生の値へ戻るのはこの1箇所である。符号付きの範囲へ収まらない番地は`None`を返すが、
    /// 取っ手の番地は小さな値であり、実際に起こることは無い。
    #[cfg(windows)]
    pub(crate) fn 子への取っ手の番地(&self) -> Option<isize> {
        use std::os::windows::io::AsRawHandle;
        isize::try_from(self.本体.as_raw_handle().addr()).ok()
    }

    /// 子孫まで終わらせたうえで、子の終了を見送って後片付けを済ませる。
    /// 見送りを省くとWindowsでプロセスの記録が残り続けるため、終わらせたあとに必ず待つ。
    pub(crate) fn 木ごと終わらせて見送る(&mut self) {
        if cfg!(windows) {
            self.番号().この番号を根とする木を終わらせる();
        } else {
            let _ = self.本体.kill();
        }
        let _ = self.本体.wait();
    }
}
