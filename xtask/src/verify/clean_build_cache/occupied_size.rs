//! 占める容量: ディスクの上で1つの置き場が占めるバイト数を表す値オブジェクト。
//!
//! 裸の`u64`で持たないのは、件数・秒数と同じ整数であり、取り違えても型が通るためである。
//! 人へ見せる単位への換算をこの型の`Display`が持ち、呼び出し側は換算の式を1つも書かない。
//!
//! 換算を整数の演算だけで書くのは、`as`による数値変換をこのリポジトリの規約が禁じているためである。
//! 小数第2位までを求めるのに剰余を100倍してから割る形にすると、浮動小数点を1度も通らない。

use std::path::Path;

use super::error::掃除の破れ;

const 単位の刻み: u64 = 1024;
const 単位の綴り一覧: [&str; 5] = ["バイト", "KB", "MB", "GB", "TB"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(super) struct 占める容量(u64);

impl 占める容量 {
    pub(super) const fn 無し() -> Self {
        Self(0)
    }

    /// ディレクトリの下にあるファイルのバイト数を全部足す。辿れないファイルは0として数えず、読み取りの破れを返す。
    pub(super) fn ディレクトリを測る(ディレクトリ: &Path) -> Result<Self, 掃除の破れ> {
        if !ディレクトリ.is_dir() {
            return Ok(Self::無し());
        }
        let 読めなかった = |誤り| 掃除の破れ::ディレクトリを読めなかった {
            ディレクトリ: ディレクトリ.to_path_buf(),
            誤り,
        };
        let mut 合計 = 0u64;
        for エントリ結果 in std::fs::read_dir(ディレクトリ).map_err(読めなかった)? {
            let エントリ = エントリ結果.map_err(読めなかった)?;
            let パス = エントリ.path();
            let 種別 = エントリ.file_type().map_err(読めなかった)?;
            if 種別.is_symlink() {
                continue; // 印だけを数え、指す先は数えない。指す先が木の外にあると二重に数えるためである
            }
            if 種別.is_dir() {
                合計 = 合計.saturating_add(Self::ディレクトリを測る(&パス)?.0);
                continue;
            }
            合計 = 合計.saturating_add(エントリ.metadata().map_err(読めなかった)?.len());
        }
        Ok(Self(合計))
    }

    pub(super) fn 足す(self, 他: Self) -> Self {
        Self(self.0.saturating_add(他.0))
    }
}

impl std::fmt::Display for 占める容量 {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut 位取り = 1u64;
        let mut 単位の添字 = 0;
        while 単位の添字 + 1 < 単位の綴り一覧.len() && self.0 / 位取り >= 単位の刻み {
            位取り *= 単位の刻み;
            単位の添字 += 1;
        }
        let 単位 = 単位の綴り一覧[単位の添字];
        if 単位の添字 == 0 {
            return write!(書き手, "{}{単位}", self.0);
        }
        let 整数部 = self.0 / 位取り;
        let 小数部 = self.0 % 位取り * 100 / 位取り;
        write!(書き手, "{整数部}.{小数部:02}{単位}")
    }
}
