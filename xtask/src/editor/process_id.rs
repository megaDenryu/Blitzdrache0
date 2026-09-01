//! OSが割り当てたプロセス番号と、その番号を根とする木の終わらせ方。
//! 番号だけで終わらせられる形にしてあるのは、Ctrl+Cの割り込みが`Child`を持ち込めない
//! 別のスレッドで走るためである(参照: `interrupt.rs`)。

use std::process::{Child, Command};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct プロセス番号(u32);

impl プロセス番号 {
    pub(crate) fn 起動済みの子から得る(子: &Child) -> Self {
        Self(子.id())
    }

    /// 注意: Windowsでは親を終わらせても子孫は道連れにならない。`npm.cmd`は`cmd.exe`→`node`(npm)→`cmd.exe`→`node`(vite)
    /// と連なるため、親だけを終わらせるとviteが待ち受け口を掴んだまま残り、次の起動が衝突で落ちる。
    /// 木ごと終わらせる標準の道具は`taskkill /T`だけであるため、外部プログラムとして呼ぶ。
    /// 出力を捨てるのは、道具が出す英語の行を検収の読み手へ流さないためである。
    pub(crate) fn この番号を根とする木を終わらせる(self) {
        let 番号の綴り = self.0.to_string();
        let 結果 = if cfg!(windows) {
            Command::new("taskkill").args(["/T", "/F", "/PID", &番号の綴り]).output()
        } else {
            // Unixには木ごと終わらせる標準の道具が無いため、番号1つを終わらせる。
            Command::new("kill").args(["-TERM", &番号の綴り]).output()
        };
        if let Err(原因) = 結果 {
            println!("プロセス{番号の綴り}を終わらせる道具を呼べない: {原因}");
        }
    }
}
