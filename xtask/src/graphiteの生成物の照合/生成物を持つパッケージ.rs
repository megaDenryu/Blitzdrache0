//! Graphiteの生成物を持つパッケージの一覧と、照合の対象との突き合わせ。受け取るのは走査した原文の一覧、返すのは照合の対象に無いパッケージの並びである。
//!
//! 照合の対象はパッケージの宣言の依存の行から選ぶため、書き方の読み落としで対象が0件のまま成功しうる。それを止めるため、
//! `cargo xtask conform` が見出しと生成元の宣言の結び付きで生成物と認めたファイル(`Graphiteの生成物の一覧`)を持つパッケージが、
//! 対象の一覧に全部入っていることを確かめる。生成物の見分けは conform と同じ型に任せ、ここでは判定しない。
//! 生成物の属するパッケージは、パスの中で最初に現れる `src` の手前までとする。生成物は生成元の宣言が `include!` で取り込むため、
//! パッケージの `src` の下に置かれるからである。ファイルシステムには問い合わせない。

use std::path::{Path, PathBuf};

use crate::conform::graphiteのコード::Graphiteの生成物の一覧;
use crate::conform::走査した原文の一覧::{斜線で揃えたパス, 走査した原文の一覧};

/// ソースの根のディレクトリの名前。この手前までがパッケージのディレクトリである。
const ソースの根: &str = "src";

/// Graphiteの生成物を1件以上持つパッケージのディレクトリの並び。
pub(super) struct 生成物を持つパッケージの一覧 {
    パッケージ一覧: Vec<PathBuf>,
}

impl 生成物を持つパッケージの一覧 {
    pub(super) fn 原文一覧から組む(原文一覧: &走査した原文の一覧) -> Self {
        let mut パッケージ一覧: Vec<PathBuf> = Graphiteの生成物の一覧::原文一覧から見分ける(原文一覧).生成物一覧().iter().filter_map(|生成物| 属するパッケージ(生成物)).collect();
        パッケージ一覧.sort();
        パッケージ一覧.dedup();
        Self { パッケージ一覧 }
    }

    /// 生成物を持つのに照合の対象の一覧に無いパッケージ。空なら、生成物を持つパッケージは全部照合される。
    pub(super) fn 照合の対象に無いパッケージ一覧(&self, 対象一覧: &[PathBuf]) -> Vec<PathBuf> {
        let 対象の表記一覧: Vec<斜線で揃えたパス> = 対象一覧.iter().map(|対象| 斜線で揃えたパス::生成する(対象)).collect();
        self.パッケージ一覧.iter().filter(|パッケージ| !対象の表記一覧.contains(&斜線で揃えたパス::生成する(パッケージ))).cloned().collect()
    }

    pub(super) fn 件数(&self) -> usize {
        self.パッケージ一覧.len()
    }
}

// パスの中で最初に現れる `src` の手前までを、そのファイルが属するパッケージのディレクトリとする。`src` を含まないパスは無しである。
fn 属するパッケージ(生成物: &Path) -> Option<PathBuf> {
    let 部品一覧: Vec<_> = 生成物.components().collect();
    let 根の位置 = 部品一覧.iter().position(|部品| 部品.as_os_str() == ソースの根)?;
    Some(部品一覧[..根の位置].iter().collect())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::生成物を持つパッケージの一覧;
    use crate::conform::走査した原文の一覧::走査した原文の一覧;

    fn 題材の原文一覧() -> 走査した原文の一覧 {
        let 宣言 = "graphite::dynamic_graph_schema! {\n    generated = \"generated/仮の図式の生成物.rs\";\n    schema 図式 { node 地点; }\n}\n";
        let 生成物 = "// このファイルは Graphite が生成したため手編集しないこと。\n// 生成元: src/仮の図式の宣言.rs:1\npub struct 地点Id;\n";
        走査した原文の一覧::生成する(vec![
            (PathBuf::from("crates/仮の経路網/src/仮の図式の宣言.rs"), 宣言.to_string()),
            (PathBuf::from("crates/仮の経路網/src/generated/仮の図式の生成物.rs"), 生成物.to_string()),
            (PathBuf::from("crates/仮の別の網/src/lib.rs"), "pub struct 地点;\n".to_string()),
        ])
    }

    #[test]
    fn 生成物を持つパッケージが照合の対象に無ければ名指す() {
        let 一覧 = 生成物を持つパッケージの一覧::原文一覧から組む(&題材の原文一覧());
        assert_eq!(一覧.件数(), 1, "生成物を持つパッケージは仮の経路網の1件だけである");
        assert_eq!(一覧.照合の対象に無いパッケージ一覧(&[]), vec![PathBuf::from("crates/仮の経路網")]);
        assert_eq!(一覧.照合の対象に無いパッケージ一覧(&[PathBuf::from("crates/仮の別の網")]), vec![PathBuf::from("crates/仮の経路網")]);
    }

    #[test]
    fn 生成物を持つパッケージが照合の対象に在れば何も名指さない() {
        let 一覧 = 生成物を持つパッケージの一覧::原文一覧から組む(&題材の原文一覧());
        assert!(一覧.照合の対象に無いパッケージ一覧(&[PathBuf::from(r"crates\仮の経路網")]).is_empty(), "区切りの違う同じパッケージを別のパッケージと見た");
    }
}
