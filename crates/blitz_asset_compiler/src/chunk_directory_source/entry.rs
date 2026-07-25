//! チャンク目録ソースの1行が表す、チャンク座標とアセットIDとソースファイルの対応。
//! 座標をファイル名から逆算せず行の中に明記させるため、ファイル名規約への暗黙の依存が生まれない。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, チャンク座標};

use crate::error::アセットコンパイルエラー;

pub struct チャンク目録ソース項目 {
    チャンク: チャンク座標,
    アセット: アセットID,
    ソース相対パス: PathBuf,
}

impl チャンク目録ソース項目 {
    /// 1行は空白区切りの4欄、x、z、アセットID、目録ファイルの配置先からのソース相対パスである。
    pub(super) fn 行から解析する(行番号: usize, 行: &str) -> Result<Self, アセットコンパイルエラー> {
        let 欄一覧: Vec<&str> = 行.split_whitespace().collect();
        let [x文字列, z文字列, id文字列, パス文字列] = 欄一覧.as_slice() else {
            return Err(行不正(行番号, 行));
        };
        let (Ok(x), Ok(z)) = (x文字列.parse::<i32>(), z文字列.parse::<i32>()) else {
            return Err(行不正(行番号, 行));
        };
        let Ok(アセット) = アセットID::生成する(id文字列) else {
            return Err(行不正(行番号, 行));
        };
        Ok(Self {
            チャンク: チャンク座標::生成する(x, z),
            アセット,
            ソース相対パス: PathBuf::from(パス文字列),
        })
    }

    pub fn チャンク(&self) -> チャンク座標 {
        self.チャンク
    }

    pub fn アセット(&self) -> &アセットID {
        &self.アセット
    }

    pub fn ソース相対パス(&self) -> &Path {
        &self.ソース相対パス
    }
}

fn 行不正(行番号: usize, 行: &str) -> アセットコンパイルエラー {
    アセットコンパイルエラー::チャンク目録ソース行不正 {
        行番号,
        内容: 行.to_string(),
    }
}
