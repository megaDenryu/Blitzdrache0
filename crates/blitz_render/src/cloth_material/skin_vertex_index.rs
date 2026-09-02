//! スキン済み頂点バッファの1頂点を指す添字。布粒子添字や目標拘束添字と同じu32のまま配ると取り違えても型が通るため、別の型で包む。
//! 頂点数との照合はレンダラーの生成(スキニング資源の頂点数を知る時点)が行い、生値へ落とすのは目標の更新対応のバイト列化だけである。

use std::fmt;

/// スキン済み頂点の添字。並びはスキンメッシュ素材の頂点の順である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct スキン頂点添字(u32);

impl スキン頂点添字 {
    /// スキンメッシュ素材の頂点の並びの位置から作る。
    pub const fn 生成する(値: u32) -> Self {
        Self(値)
    }

    /// GPUのバイト列へ書く境界向けの生値。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// CPU側の頂点一覧を引くための配列添字。
    pub fn 配列添字(self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("スキン頂点添字{}がusizeに収まらない", self.0))
    }

    /// 頂点数の範囲内を指しているか。
    pub fn 頂点数の範囲内か(self, 頂点数: u32) -> bool {
        self.0 < 頂点数
    }
}

impl fmt::Display for スキン頂点添字 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.0)
    }
}
