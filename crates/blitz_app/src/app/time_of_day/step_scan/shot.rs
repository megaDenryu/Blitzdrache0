//! 1撮影ぶんの条件。担うのは「どの時刻へ時計を据え、どの区間で遠方環境を焼き、どのファイル名で書き出すか」だけである。
//!
//! 境界の下側と上側を判別共用体で持つのは、対の2枚がどちらの区間のものかをファイル名と報告の両方で
//! 取り違えないためである。真偽値で持つと、真がどちらを指すかが呼び出し側の記憶になる。

use blitz_engine::sky::太陽天頂区間;

use super::super::interval_override::太陽天頂区間の上書き;

/// 境界のどちら側の区間で焼いた撮影か。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 撮影の側 {
    下側の区間,
    上側の区間,
}

impl 撮影の側 {
    /// 書き出すファイル名に付ける語。パスはASCIIで保つ。
    fn ファイル名の語(self) -> &'static str {
        match self {
            Self::下側の区間 => "low",
            Self::上側の区間 => "high",
        }
    }
}

/// 1撮影の条件。時刻は対の2枚で同じであり、区間だけが違う。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::app) struct 撮影指定 {
    /// 時計を据える一日内秒。境界を最初に跨いだ時刻であり、対の2枚で同じ値である。
    pub(in crate::app) 一日内秒: f64,
    /// この撮影で遠方環境を焼く区間。
    pub(in crate::app) 区間: 太陽天頂区間,
    /// この撮影が属する境界の上側の区間識別。報告とファイル名が境界を名指しするのに使う。
    pub(in crate::app) 境界の上側の区間識別: u16,
    pub(super) 側: 撮影の側,
}

impl 撮影指定 {
    pub(in crate::app) fn 区間の上書き(&self) -> 太陽天頂区間の上書き {
        太陽天頂区間の上書き::指定の区間 {
            区間識別: self.区間.区間識別(),
        }
    }

    /// 書き出しのベース名に足す語。境界の識別と側だけで一意になる。
    pub(in crate::app) fn ファイル名の後置き(&self) -> String {
        format!("_{}_{}", self.境界の上側の区間識別, self.側.ファイル名の語())
    }
}
