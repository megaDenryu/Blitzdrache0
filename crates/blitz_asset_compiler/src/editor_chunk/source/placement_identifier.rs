//! 配置識別子: エディターがチャンクへ置いた配置1件へ付けた文字列。配置1件とは建物1棟または散布個体1本のことで
//! あり、どちらもこの型で指す。担当するのは、文字列の正準化と、正準でない値が型として存在しないことの保証である。
//!
//! 正準化(前後の空白を落とす)をこの型の生成へ閉じるのは、ソースを読む地点から下流のすべてが同じ文字列で同じ配置を
//! 指すためである。生の文字列で重複を判定し、焼く工程だけが正準化した文字列を正本にすると、ソース検査を通った
//! `" 建物 "`と`"建物"`が物理形状の重複としてコンパイルの後段で落ちる。読み取りは必ずこの生成を通る。

use serde::{Deserialize, Deserializer};

/// エディターがチャンクへ置いた配置1件(建物または散布個体)へ付けた、正準化済みの文字列。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct 配置識別子(String);

/// 配置識別子の生成が拒む理由。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum 配置識別子のエラー {
    /// 文字列が空白だけであり、前後の空白を落とすと何も残らない。
    #[error("配置識別子が空白だけであり、前後の空白を落とすと空になる")]
    正準化すると空になる,
}

impl 配置識別子 {
    /// 前後の空白を落とした文字列だけを持つ。落とした結果が空なら型付きエラーで拒む。
    pub(crate) fn 文字列を正準化して生成する(文字列: &str) -> Result<Self, 配置識別子のエラー> {
        let 正準の文字列 = 文字列.trim();
        if 正準の文字列.is_empty() {
            return Err(配置識別子のエラー::正準化すると空になる);
        }
        Ok(Self(正準の文字列.to_string()))
    }

    /// 包んでいる正準の文字列そのもの。重複の判定と、下流の種・描画・物理がこの値を読む。
    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for 配置識別子 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        出力.write_str(self.文字列())
    }
}

impl<'de> Deserialize<'de> for 配置識別子 {
    fn deserialize<入力元: Deserializer<'de>>(入力元: 入力元) -> Result<Self, 入力元::Error> {
        let 文字列 = String::deserialize(入力元)?;
        Self::文字列を正準化して生成する(&文字列).map_err(serde::de::Error::custom)
    }
}
