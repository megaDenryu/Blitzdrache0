//! 世界のソースディレクトリ: 1つの世界のソース一式が並ぶディレクトリを表す型。チャンク目録ソースのファイル名を持ち、
//! 直下のファイルの読み書きを行う。
//!
//! 直下のパスを組む口と読み書きの口を`pub(super)`に留めるのは、任意のファイル名を外から流せるようにしないためである。
//! 世界ごとの綴りは、その世界の置き場の型(`fox_tour_source_directory`)が持ち、この型を包んで役割別のメソッドで見せる。
//!
//! 置き場のパスは包んだ生成の出力ルートが1回だけ所有する。台帳の置き場として渡すときも借用で貸し、複製を作らない。

use std::path::{Path, PathBuf};

use super::world_directory_name::世界のディレクトリ名;
use crate::generation_ledger::生成の出力ルート;

/// どの世界のソースディレクトリにも同じ名前で置く、チャンク目録ソースのファイル名。
const チャンク目録ソースのファイル名: &str = "chunk_directory.txt";

#[repr(transparent)]
pub struct 世界のソースディレクトリ(生成の出力ルート);

impl 世界のソースディレクトリ {
    pub(super) fn ソースルートの下に作る(ソースルートのパス: &Path, 名前: 世界のディレクトリ名) -> Self {
        Self(生成の出力ルート::生成する(ソースルートのパス.join(名前.綴り())))
    }

    /// ディレクトリを作ってから返す。書き出す側は最初にこれを通り、読む側は通らない。
    pub(super) fn 作ってから開く(self) -> Result<Self, String> {
        self.0.ディレクトリを作る().map_err(|誤り| 誤り.to_string())?;
        Ok(self)
    }

    pub(super) fn チャンク目録ソースのパス(&self) -> PathBuf {
        self.0.直下のファイルのパス(チャンク目録ソースのファイル名)
    }

    pub(super) fn チャンク目録ソースへ書き込む(&self, バイト列: &[u8]) -> Result<(), String> {
        self.直下へ書き込む(チャンク目録ソースのファイル名, バイト列)
    }

    pub(super) fn 直下のファイルのパス(&self, ファイル名: &str) -> PathBuf {
        self.0.直下のファイルのパス(ファイル名)
    }

    /// 目録の1行のように、置き場からの相対パスとして与えられたソースのパス。綴りへ写し替えずに組む。
    pub(super) fn 直下の相対パスが指すファイル(&self, 相対パス: &Path) -> PathBuf {
        self.0.直下の相対パスが指すファイル(相対パス)
    }

    /// この置き場がファイルへ書く唯一の口。生値のパスがここより外へ出ない。
    pub(super) fn 直下へ書き込む(&self, ファイル名: &str, バイト列: &[u8]) -> Result<(), String> {
        let パス = self.直下のファイルのパス(ファイル名);
        std::fs::write(&パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
    }

    pub(super) fn 直下のファイルを綴りとして読む(&self, ファイル名: &str) -> Result<String, String> {
        let パス = self.直下のファイルのパス(ファイル名);
        std::fs::read_to_string(&パス).map_err(|誤り| format!("{}を読めなかった: {誤り}", パス.display()))
    }

    /// 生成台帳の置き場としてのこの置き場。台帳の読み書きは生成の出力ルートの側が担う。
    pub(super) fn 台帳の置き場(&self) -> &生成の出力ルート {
        &self.0
    }

    pub(super) fn 表示の綴り(&self) -> std::path::Display<'_> {
        self.0.表示の綴り()
    }
}
