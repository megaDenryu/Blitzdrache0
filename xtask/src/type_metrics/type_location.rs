//! 計測が1つの型を識別する鍵。定義ファイルのリポジトリからの相対パスと型名の組であり、
//! 同じ名前の型が別の場所に複数あっても別の型として数えるために置く。
//!
//! モジュールの経路つきの名前を鍵に採らないのは、パスからモジュールの経路を復元する規則
//! (`mod.rs`・`lib.rs`・`main.rs`・`#[path]`)を鍵の同一性が背負うことになり、その規則自体が次の欠陥の
//! 温床になるためである。行番号を鍵へ入れないのは、定義の上へ1行足すたびに台帳が壊れるためである。
//!
//! 綴りは、定義ファイルの相対パスに二重コロンと型名を続けた形である。`crates/blitz_app/src/cli/types.rs`の
//! 起動設定なら crates/blitz_app/src/cli/types.rs::起動設定 になる。ファイルと名前を二重コロンで分けるのは、
//! 読み手がどちらがどちらか判断できる形にするためであり、この綴りを決めるのはこの型の`Display`だけである。

use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct 型の所在 {
    定義ファイル: String, // リポジトリルートからの相対パス。区切りは斜線へ揃える
    型名: String,
}

impl 型の所在 {
    /// 走査したファイルのパスから組み立てる。走査で得るパスの区切り文字は実行環境で変わるため、
    /// 台帳の綴りと突き合わせられるよう斜線へ揃える。
    pub fn 走査したファイルから生成する(ファイル: &Path, 型名: &str) -> Self {
        Self {
            定義ファイル: ファイルの綴りへ揃える(ファイル),
            型名: 型名.to_string(),
        }
    }

    /// 台帳の1行から組み立てる。台帳は綴りを区画の根と根からの相対パスへ分けて持つため、繋ぐ規則をここへ閉じる。
    /// 繋ぎを呼び出し側に書かせると、走査の側と台帳の側で区切りの綴りが割れる余地が残る。
    pub fn 区画の根と相対パスから生成する(区画の根: &str, 根からのパス: &str, 型名: &str) -> Self {
        Self {
            定義ファイル: format!("{区画の根}/{根からのパス}"),
            型名: 型名.to_string(),
        }
    }

    /// 違反の報告が指す先。読み手が開くのはこの型の定義のファイルである。
    pub fn 定義ファイルのパス(&self) -> PathBuf {
        PathBuf::from(&self.定義ファイル)
    }

    /// 経路の解決が出したファイルの綴りと突き合わせるための綴り。区切りは斜線へ揃えてある。
    pub fn 定義ファイルの綴り(&self) -> &str {
        &self.定義ファイル
    }
}

/// 走査で得たパスを、台帳と経路の解決が使う斜線区切りの綴りへ揃える。実行環境で区切り文字が変わるためである。
pub fn ファイルの綴りへ揃える(ファイル: &Path) -> String {
    ファイル.to_string_lossy().replace('\\', "/")
}

impl fmt::Display for 型の所在 {
    fn fmt(&self, 書き手: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き手, "{}::{}", self.定義ファイル, self.型名)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 綴りはファイルと型名を二重コロンで分ける() {
        let 所在 = 型の所在::区画の根と相対パスから生成する("crates/blitz_app/src", "cli/types.rs", "起動設定");
        assert_eq!(所在.to_string(), "crates/blitz_app/src/cli/types.rs::起動設定");
    }

    #[test]
    fn 逆斜線の区切りでも台帳の綴りと一致する() {
        let 走査 = 型の所在::走査したファイルから生成する(Path::new(r"xtask\src\smoke\launch_setting.rs"), "起動設定");
        assert_eq!(
            走査,
            型の所在::区画の根と相対パスから生成する("xtask/src", "smoke/launch_setting.rs", "起動設定")
        );
    }

    #[test]
    fn 同じ名前でもファイルが違えば別の所在とみなす() {
        let 片方 = 型の所在::区画の根と相対パスから生成する("crates/blitz_app/src", "cli/types.rs", "起動設定");
        let もう片方 = 型の所在::区画の根と相対パスから生成する("xtask/src", "smoke/launch_setting.rs", "起動設定");
        assert_ne!(片方, もう片方);
    }
}
