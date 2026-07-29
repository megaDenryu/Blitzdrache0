//! 契約指摘: 適合検査が見つけた1件の破れ。担当するのは重大度・位置・内容・是正方法の4つ組の定義と、その表記である。
//! 是正方法を省略できない項目にするのは、この道具の主な読者がAIエージェントであり、次にどの操作をするかを決められない報告に価値がないためである。
//! 表記を同じファイルへ置くのは、指摘の文面が指摘の定義そのものであり、分けると私有フィールドの可視性を検査モジュール全体へ広げることになるためである。

use std::fmt;

use super::target::対象位置;

/// 違反は、焼けないか、焼けても作者の宣言と絵が食い違うことが確定するもの。
/// 警告は、宣言がローダーに読まれず捨てられるが、作者がその宣言を諦めていれば絵が成立するもの。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 重大度 {
    違反,
    警告,
}

pub struct 契約指摘 {
    重大度: 重大度,
    位置: 対象位置,
    内容: String,
    是正方法: String,
}

impl 契約指摘 {
    pub(super) fn 違反を作る(位置: 対象位置, 内容: impl Into<String>, 是正方法: impl Into<String>) -> Self {
        Self {
            重大度: 重大度::違反,
            位置,
            内容: 内容.into(),
            是正方法: 是正方法.into(),
        }
    }

    pub(super) fn 警告を作る(位置: 対象位置, 内容: impl Into<String>, 是正方法: impl Into<String>) -> Self {
        Self {
            重大度: 重大度::警告,
            位置,
            内容: 内容.into(),
            是正方法: 是正方法.into(),
        }
    }

    pub fn 重大度(&self) -> 重大度 {
        self.重大度
    }
}

impl fmt::Display for 契約指摘 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        let 見出し = match self.重大度 {
            重大度::違反 => "違反",
            重大度::警告 => "警告",
        };
        writeln!(出力, "[{見出し}] {}: {}", self.位置, self.内容)?;
        write!(出力, "        是正: {}", self.是正方法)
    }
}
