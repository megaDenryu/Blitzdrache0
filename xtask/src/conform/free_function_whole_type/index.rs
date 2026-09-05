//! クレートの中で定義された型の名前から、その型が根になるモジュールの木を引く索引と、
//! その索引を使って1ファイルの自由関数を判定する操作。
//!
//! 同じ名前の型が別のモジュールで定義されることがあるため、1つの名前に複数の木を持つ。
//! 判定は「どれか1つの木がこのファイルを含むか」であり、含む木があれば親の型とみなす。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::parameter::丸ごと受け取る型の名前;
use super::signature::自由関数の署名一覧;
use crate::rust_module::モジュールの位置;
use crate::type_metrics::{ファイルの観測, 観測};

pub struct 親の型を丸ごと受け取る自由関数 {
    pub パス: PathBuf,
    pub 行番号: usize,
    pub 関数名: String,
    pub 型名: String,
}

pub struct 型の定義の索引 {
    木の表: BTreeMap<String, Vec<モジュールの位置>>,
}

impl 型の定義の索引 {
    pub fn 観測から生成する(ファイル別観測: &[ファイルの観測]) -> Self {
        let mut 木の表: BTreeMap<String, Vec<モジュールの位置>> = BTreeMap::new();
        for ファイル in ファイル別観測 {
            for 観測 in &ファイル.観測一覧 {
                if let 観測::型定義 { 型名, .. } = 観測 {
                    木の表
                        .entry(型名.clone())
                        .or_default()
                        .push(モジュールの位置::定義ファイルから生成する(&ファイル.パス));
                }
            }
        }
        Self { 木の表 }
    }

    pub fn 親の型を丸ごと受け取る自由関数を探す(
        &self,
        パス: &Path,
        内容: &str,
    ) -> Vec<親の型を丸ごと受け取る自由関数> {
        let mut 検出一覧 = Vec::new();
        for 署名 in 自由関数の署名一覧(内容) {
            for 引数 in &署名.引数一覧 {
                let Some(型名) = 丸ごと受け取る型の名前(引数).filter(|型名| self.親の型か(型名, パス)) else {
                    continue;
                };
                検出一覧.push(親の型を丸ごと受け取る自由関数 {
                    パス: パス.to_path_buf(),
                    行番号: 署名.行番号,
                    関数名: 署名.関数名.clone(),
                    型名,
                });
            }
        }
        検出一覧
    }

    fn 親の型か(&self, 型名: &str, パス: &Path) -> bool {
        self.木の表
            .get(型名)
            .is_some_and(|木一覧| 木一覧.iter().any(|木| 木.この木の中のファイルか(パス)))
    }
}
