//! xtaskが開くプロジェクトルート。担当するのは、`--project <ルート>`の引数から開くプロジェクトのルートを
//! 決めることと、配下の置き場を組み立てる口へそのパスを貸すことだけである。
//!
//! 生の`PathBuf`で持ち回らないのは、リポジトリルートとまったく同じ姿をしており、取り違えて渡しても型が通るためである。
//! カタログの書き出しはこの2つを両方受け取るため、型で分けないと引数の順を入れ替えた実行が黙って別の格子を読む。
//!
//! `editor_server`の同名の型と役割は同じだが、xtaskは依存の白リストで`editor_server`へ依存できないため、
//! ここへ同じ役割の型を1つ置く。引数の解決の規則の正本はあちらであり、こちらは規則を写している。
//! 参照: `crates/editor_server/src/project_root.rs`

use std::path::{Path, PathBuf};

/// プロジェクトルートとは、いま開いている1つのゲームプロジェクトのルートディレクトリのことである。
pub(crate) struct プロジェクトルート(PathBuf);

impl プロジェクトルート {
    /// `--project <ルート>`の次の引数を開くルートにする。指定が無ければ既定ルート(リポジトリルート)を使う。
    pub(crate) fn 引数から解く(引数一覧: &[String], 既定ルート: &Path) -> Self {
        let 指定パス = 引数一覧
            .iter()
            .position(|引数| 引数 == "--project")
            .and_then(|添字| 引数一覧.get(添字 + 1));
        Self(指定パス.map_or_else(|| 既定ルート.to_path_buf(), PathBuf::from))
    }

    /// 境界: 建物の格子の置き場を組み立てるためにパスを貸す。生のパスへ戻るのはこの1箇所である。
    pub(crate) fn パス(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectの指定が無ければ既定ルートを使う() {
        assert_eq!(プロジェクトルート::引数から解く(&[], Path::new("/repo")).パス(), Path::new("/repo"));
    }

    #[test]
    fn projectの次の引数をプロジェクトルートにする() {
        let 引数一覧 = vec!["--project".to_string(), "/game".to_string()];
        assert_eq!(プロジェクトルート::引数から解く(&引数一覧, Path::new("/repo")).パス(), Path::new("/game"));
    }
}
