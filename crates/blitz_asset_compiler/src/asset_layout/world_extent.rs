//! 世界の広がり: 原点を中心として東西と南北へ並べるチャンク数を表す値オブジェクト。

use thiserror::Error;

use super::世界のディレクトリ名;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 世界の広がり {
    東西チャンク数: u16,
    南北チャンク数: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum 世界の広がりエラー {
    #[error("世界の東西チャンク数は1以上でなければならない")]
    東西チャンク数が零,
    #[error("世界の南北チャンク数は1以上でなければならない")]
    南北チャンク数が零,
}

impl 世界の広がり {
    pub const fn 生成する(東西チャンク数: u16, 南北チャンク数: u16) -> Result<Self, 世界の広がりエラー> {
        if 東西チャンク数 == 0 {
            Err(世界の広がりエラー::東西チャンク数が零)
        } else if 南北チャンク数 == 0 {
            Err(世界の広がりエラー::南北チャンク数が零)
        } else {
            Ok(Self {
                東西チャンク数,
                南北チャンク数,
            })
        }
    }

    pub const fn 場所巡りの既定値() -> Self {
        Self {
            東西チャンク数: 3,
            南北チャンク数: 3,
        }
    }

    /// 大規模世界の広がりの正本。8km×10kmを一辺100メートルのチャンクで割った値である。
    /// 引数を省いた生成と、絵の対照を焼く検収の両方がこの1つを読む。写しを持つと、片方だけが別の世界を焼く。
    pub const fn 大規模世界の既定値() -> Self {
        Self {
            東西チャンク数: 80,
            南北チャンク数: 100,
        }
    }

    pub const fn 東西チャンク数(self) -> u16 {
        self.東西チャンク数
    }

    pub const fn 南北チャンク数(self) -> u16 {
        self.南北チャンク数
    }

    pub fn チャンク総数(self) -> usize {
        usize::from(self.東西チャンク数) * usize::from(self.南北チャンク数)
    }

    pub fn 東の最小座標(self) -> i32 {
        -(i32::from(self.東西チャンク数) / 2)
    }

    pub fn 南の最小座標(self) -> i32 {
        -(i32::from(self.南北チャンク数) / 2)
    }

    pub fn 東の最大座標(self) -> i32 {
        self.東の最小座標() + i32::from(self.東西チャンク数) - 1
    }

    pub fn 南の最大座標(self) -> i32 {
        self.南の最小座標() + i32::from(self.南北チャンク数) - 1
    }

    pub fn 焼き方の指定を組み立てる(self, 世界名: 世界のディレクトリ名) -> String {
        format!(
            "{} east_chunks={} south_chunks={}",
            世界名.綴りを見せる(),
            self.東西チャンク数,
            self.南北チャンク数
        )
    }
}
