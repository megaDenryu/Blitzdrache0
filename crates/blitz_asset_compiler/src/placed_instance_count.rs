//! 置いた個体の数: 焼く工程が実際に群へ置いた個体の数。実行時カタログの容量メタデータが数える個体数とは
//! 別の地点で数えた値である。
//!
//! 別の地点で数えるのは、カタログの個体数がシーンへ載った描画対象だけを数えるためである。配置は作られたのに
//! 群の描画対象がシーンへ載らなかった欠陥は、載った側だけを数えているかぎり2つの数が同じ1つの計算から出るため、
//! 突き合わせても捕まらない。配置列がインスタンス群になる直前で数えることで、載せ漏れが2つの数の食い違いとして現れる。

mod by_kind;

pub use by_kind::{散らした種類の名前, 種類ごとの置いた個体の数};

use crate::error::アセットコンパイルエラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct 置いた個体の数(u64);

impl 置いた個体の数 {
    /// 1体も置かなかったこと。群を1つも作らない工程がこの値を返す。
    pub fn 零() -> Self {
        Self(0)
    }

    pub(crate) fn 配置列の長さから生成する(長さ: usize) -> Result<Self, アセットコンパイルエラー> {
        let 数 = u64::try_from(長さ).map_err(|_| blitz_engine::アセット実行時形式エラー::長さ表現不能)?;
        Ok(Self(数))
    }

    pub fn 足す(self, 他: Self) -> Result<Self, アセットコンパイルエラー> {
        let 合計 = self.0.checked_add(他.0).ok_or(blitz_engine::アセット実行時形式エラー::長さ表現不能)?;
        Ok(Self(合計))
    }

    pub fn 値(self) -> u64 {
        self.0
    }
}
