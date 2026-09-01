//! 開発サーバーの待ち受け口。担当するのは、起動前に番号が空いているかを確かめ、
//! 塞がっていたら前回の残りを疑う手掛かりを日本語で出すことである。
//!
//! 番号の正本は`editor_web/vite.config.ts`の`server.port`であり、ここへ写しを持たない。
//! 写しを持つと、番号を変えたときに案内だけが古い値のまま残り、案内が嘘になる。

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener};

/// 空きを試すべき自分宛ての住所。viteは`localhost`を解決して待ち受けるため、実測ではIPv6側(`::1`)だけを
/// 掴む場合がある。IPv4側だけを試すと、塞がっているのに空いていると読み違えて案内が出ない。
const 自分宛ての住所一覧: [IpAddr; 2] = [IpAddr::V4(Ipv4Addr::LOCALHOST), IpAddr::V6(Ipv6Addr::LOCALHOST)];

use super::web_root::エディター画面の置き場;

pub(crate) struct 開発サーバーの待ち受け口 {
    番号: u16,
}

impl 開発サーバーの待ち受け口 {
    /// 設定の中の`port:`で始まる行から番号を読む。読めなければ`None`を返し、案内を諦める。
    /// 読めないことを既定値で埋めないのは、当てずっぽうの番号で案内を出すほうが有害なためである。
    pub(crate) fn vite設定から読み取る(置き場: &エディター画面の置き場) -> Option<Self> {
        let 設定 = std::fs::read_to_string(置き場.vite設定のパス()).ok()?;
        let 行 = 設定.lines().find(|行| 行.trim_start().starts_with("port:"))?;
        let 数字の並び: String = 行
            .trim_start()
            .trim_start_matches("port:")
            .trim_start()
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        数字の並び.parse().ok().map(|番号| Self { 番号 })
    }

    /// どちらの住所でも待ち受けを開けるなら空いている。片方でも開けなければ誰かが掴んでいる。
    fn 空いているか(&self) -> bool {
        自分宛ての住所一覧.iter().all(|住所| TcpListener::bind((*住所, self.番号)).is_ok())
    }

    /// 起動前に呼ぶ。塞がっているのは前回の`cargo xtask editor`の残りである見込みが高いため、
    /// 調べ方と止め方まで書く。他人のプロセスかもしれないため、こちらからは終わらせない。
    pub(crate) fn 塞がっているなら残りの掃除を案内する(&self) {
        if self.空いているか() {
            return;
        }
        let 番号 = self.番号;
        println!("開発サーバーの待ち受け口{番号}が既に使われている。前回の `cargo xtask editor` の開発サーバー(vite)が残っている見込みが高い。");
        println!("誰が掴んでいるかを調べる: powershell -Command \"Get-NetTCPConnection -LocalPort {番号} -State Listen\"");
        println!("前回の残りだと確かめたうえで終わらせる: taskkill /T /F /PID <番号>");
        println!("このまま起動を続けるが、開発サーバーは待ち受け口の衝突で終了する。");
    }
}
