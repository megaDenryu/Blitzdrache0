//! 対象外にした生成物とは、ある検査がGraphiteの生成物と認めて対象から外したファイルの一覧を、外した検査の名前と一緒に持つ報告の値のことである。
//! 検査器は対象から外した入力の件数を報告する規約であり、利用者は「違反0件」を全部を検査した結果と読むため、外した件数と名前を並べて見せる。
//! 参照: `.claude/global-reference/全プロジェクト共通CLAUDE.md`「検査器は解析できなかった入力を黙って対象から外してはならない」。

use std::fmt;
use std::path::PathBuf;

pub struct 対象外にした生成物 {
    外した検査: &'static str,
    パス一覧: Vec<PathBuf>,
}

impl 対象外にした生成物 {
    pub const fn 生成する(外した検査: &'static str, パス一覧: Vec<PathBuf>) -> Self {
        Self { 外した検査, パス一覧 }
    }

    /// 外したファイルの一覧。試験が、外したファイルを名前で確かめるための口である。
    #[cfg(test)]
    pub fn パス一覧(&self) -> &[PathBuf] {
        &self.パス一覧
    }
}

impl fmt::Display for 対象外にした生成物 {
    /// 1行目に件数と外した検査を、続く行に外したファイルを1件ずつ書く。0件でも1行目は書き、外した生成物が無かったことを見せる。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[xtask] conform: Graphiteの生成物{}件を「{}」の対象から外した(先頭の見出しと生成元の宣言が結び付いたファイルだけを外す)",
            self.パス一覧.len(),
            self.外した検査
        )?;
        for パス in &self.パス一覧 {
            write!(f, "\n対象外: {}", パス.display())?;
        }
        Ok(())
    }
}
