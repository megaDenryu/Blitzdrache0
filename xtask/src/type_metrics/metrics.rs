//! 型ごとの計測値と、ファイル別の観測からの集計。
//! 型名はモジュールを跨いで素の名前で集計するため、同名の別型があれば合算される。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::observation::観測;

pub struct 型計測 {
    pub 型名: String,
    pub 定義ファイル: Option<PathBuf>,
    pub フィールド数: usize,
    pub 実装ファイル一覧: BTreeSet<PathBuf>,
    pub メソッド総数: usize,
}

impl 型計測 {
    fn 新規(型名: &str) -> Self {
        Self {
            型名: 型名.to_string(),
            定義ファイル: None,
            フィールド数: 0,
            実装ファイル一覧: BTreeSet::new(),
            メソッド総数: 0,
        }
    }

    /// 降順に並べるための比較鍵。implの分散ファイル数を最優先し、次にフィールド数、最後にメソッド総数で比べる。
    /// 分散ファイル数を先頭に置くのは、privateフィールドへ触れるファイルの数がそのまま責務の広がりを表すためである。
    pub fn 比較鍵(&self) -> (usize, usize, usize) {
        (self.実装ファイル一覧.len(), self.フィールド数, self.メソッド総数)
    }
}

pub fn 集計する(ファイル別観測: &[(PathBuf, Vec<観測>)]) -> Vec<型計測> {
    let mut 表: BTreeMap<String, 型計測> = BTreeMap::new();
    for (パス, 観測一覧) in ファイル別観測 {
        for 観測 in 観測一覧 {
            let 計測 = 表.entry(観測.型名().to_string()).or_insert_with(|| 型計測::新規(観測.型名()));
            取り込む(計測, パス, 観測);
        }
    }
    let mut 一覧: Vec<型計測> = 表.into_values().collect();
    一覧.sort_by(|左, 右| 右.比較鍵().cmp(&左.比較鍵()).then_with(|| 左.型名.cmp(&右.型名)));
    一覧
}

fn 取り込む(計測: &mut 型計測, パス: &Path, 観測: &観測) {
    match 観測 {
        観測::型定義 { フィールド数, .. } => {
            計測.定義ファイル = Some(パス.to_path_buf());
            計測.フィールド数 = *フィールド数;
        }
        観測::実装ブロック { メソッド数, .. } => {
            計測.実装ファイル一覧.insert(パス.to_path_buf());
            計測.メソッド総数 += *メソッド数;
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 定義(型名: &str, フィールド数: usize) -> 観測 {
        let 型名 = 型名.to_string();
        観測::型定義 { 型名, フィールド数 }
    }

    fn 実装(型名: &str, メソッド数: usize) -> 観測 {
        let 型名 = 型名.to_string();
        観測::実装ブロック { 型名, メソッド数 }
    }

    #[test]
    fn 複数ファイルのimplを合算して降順に並べる() {
        let 観測 = vec![
            (PathBuf::from("a.rs"), vec![定義("大", 3), 実装("大", 2)]),
            (PathBuf::from("b.rs"), vec![実装("大", 1), 定義("小", 1)]),
        ];
        let 一覧 = 集計する(&観測);
        assert_eq!(一覧[0].型名, "大");
        assert_eq!(一覧[0].実装ファイル一覧.len(), 2);
        assert_eq!(一覧[0].メソッド総数, 3);
        assert_eq!(一覧[0].定義ファイル, Some(PathBuf::from("a.rs")));
        assert_eq!(一覧[1].型名, "小");
    }
}
