//! 計測が1つの型を識別する鍵。定義ファイルのリポジトリからの相対パスと型名の組であり、
//! 同じ名前の型が別の場所に複数あっても別の型として数えるために置く。
//!
//! モジュールの経路つきの名前を鍵に採らないのは、パスからモジュールの経路を復元する規則
//! (`mod.rs`・`lib.rs`・`main.rs`・`#[path]`)を計測が持つことになり、その規則自体が次の欠陥の
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
            定義ファイル: ファイル.to_string_lossy().replace('\\', "/"),
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

    /// 同じ型名の複数の定義のうち、あるimplブロックがどれに属するかを測る近さ。
    pub fn 実装ブロックのファイルへの近さ(&self, 実装ブロックのファイル: &Path) -> 実装ブロックへの近さ {
        let 相手 = 実装ブロックのファイル.to_string_lossy().replace('\\', "/");
        実装ブロックへの近さ {
            同じファイルか: 相手 == self.定義ファイル,
            ディレクトリの一致成分数: ディレクトリの一致成分数(&self.定義ファイル, &相手),
        }
    }
}

/// 1つの定義が、あるimplブロックのファイルからどれだけ近いかを表す順序。同じファイルにある定義を最も近いとし、
/// 次にディレクトリの成分を先頭から突き合わせた一致数で比べる。Rustのinherent implは定義と同じクレートにしか
/// 置けず、実際には定義と同じモジュールの木の中に置かれるため、ディレクトリの共有の深さがモジュールの木の近さになる。
/// 導出した順序はフィールドの宣言順で比べるため、宣言の並びがそのまま優先の順である。
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct 実装ブロックへの近さ {
    同じファイルか: bool,
    ディレクトリの一致成分数: usize,
}

fn ディレクトリの一致成分数(左: &str, 右: &str) -> usize {
    let ディレクトリ成分 = |綴り: &str| -> Vec<String> {
        let mut 成分: Vec<String> = 綴り.split('/').map(str::to_string).collect();
        成分.pop();
        成分
    };
    ディレクトリ成分(左)
        .iter()
        .zip(ディレクトリ成分(右))
        .take_while(|(左, 右)| **左 == *右)
        .count()
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

    #[test]
    fn 近さは同じファイルとディレクトリの共有の深さで決まる() {
        let 所在 = 型の所在::区画の根と相対パスから生成する("crates/blitz_collision/src", "triangle/sweep_solver.rs", "求解");
        let 近さ = |ファイル: &str| 所在.実装ブロックのファイルへの近さ(Path::new(ファイル));
        let 同じファイル = 近さ("crates/blitz_collision/src/triangle/sweep_solver.rs");
        let 同じディレクトリ = 近さ("crates/blitz_collision/src/triangle/sweep_face.rs");
        let 隣のディレクトリ = 近さ("crates/blitz_collision/src/height_field/sweep.rs");
        assert!(同じファイル > 同じディレクトリ);
        assert!(同じディレクトリ > 隣のディレクトリ);
    }
}
