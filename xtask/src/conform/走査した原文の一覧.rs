//! 走査した原文の一覧とは、検査や抽出が走査して読んだファイルのパスと原文の対を読んだ順に並べ、パスで原文を探す口を持つ値のことである。
//!
//! 規約の検査(1ファイル単位の検査・設計オントロジーの構文検査)と設計関係の抽出が、それぞれ自分の読んだ原文をこの型で持つ。
//! Graphiteの生成物の見分けは、見出しが名乗る生成元の原文をパスで探すため、どの呼び出し元もこの型を渡す。
//! パスで探す処理を呼び出し元ごとに書くと、区切りの揃え方と探し方が呼び出し元ごとに分かれ、同じパスを片方だけが見つける食い違いが起きる。
//!
//! 探すときはパスを斜線で揃えたパスへ直して比べる。走査が返すパスの区切りは実行環境で変わり、宣言の中の表記は斜線で書かれるためである。
//! 同じ斜線で揃えたパスを持つ対が2つ在るときは、先に並んだ対を答える。

#[path = "走査した原文の一覧/斜線の表記.rs"]
mod 斜線の表記;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use 斜線の表記::斜線で揃えたパス;

pub struct 走査した原文の一覧 {
    並び: Vec<(PathBuf, String)>,
    位置の索引: HashMap<斜線で揃えたパス, usize>, // 斜線で揃えたパスから並びの添字を引く
}

impl 走査した原文の一覧 {
    pub fn 生成する(並び: Vec<(PathBuf, String)>) -> Self {
        let mut 位置の索引 = HashMap::with_capacity(並び.len());
        for (添字, (パス, _)) in 並び.iter().enumerate() {
            位置の索引.entry(斜線で揃えたパス::生成する(パス)).or_insert(添字);
        }
        Self { 並び, 位置の索引 }
    }

    /// そのパスの原文。走査していないパスなら無しである。
    pub fn パスで探す(&self, パス: &Path) -> Option<&str> {
        let 添字 = self.位置の索引.get(&斜線で揃えたパス::生成する(パス))?;
        self.並び.get(*添字).map(|(_, 原文)| 原文.as_str())
    }

    /// パスと原文の対を読んだ順に並べる。
    pub fn 並び(&self) -> impl Iterator<Item = (&Path, &str)> {
        self.並び.iter().map(|(パス, 原文)| (パス.as_path(), 原文.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::走査した原文の一覧;

    fn 一覧を組む(対一覧: &[(&str, &str)]) -> 走査した原文の一覧 {
        走査した原文の一覧::生成する(対一覧.iter().map(|(パス, 原文)| (PathBuf::from(パス), (*原文).to_string())).collect())
    }

    #[test]
    fn 区切りと点の違うパスでも同じファイルの原文を探す() {
        let 一覧 = 一覧を組む(&[("crates/甲/src/lib.rs", "甲の原文"), ("crates/乙/src/lib.rs", "乙の原文")]);
        assert_eq!(一覧.パスで探す(Path::new("crates/甲/src/lib.rs")), Some("甲の原文"));
        assert_eq!(一覧.パスで探す(Path::new(r"crates\乙\src\lib.rs")), Some("乙の原文"));
        assert_eq!(一覧.パスで探す(Path::new("crates/甲/src/generated/../lib.rs")), Some("甲の原文"));
        assert_eq!(一覧.パスで探す(Path::new("crates/丙/src/lib.rs")), None);
    }

    #[test]
    fn 同じファイルが2度並んだら先の原文を答え並びは読んだ順を保つ() {
        let 一覧 = 一覧を組む(&[("src/a.rs", "先"), ("src/b.rs", "間"), (r"src\a.rs", "後")]);
        assert_eq!(一覧.パスで探す(Path::new("src/a.rs")), Some("先"));
        assert_eq!(一覧.並び().map(|(_, 原文)| 原文).collect::<Vec<_>>(), vec!["先", "間", "後"]);
    }
}
