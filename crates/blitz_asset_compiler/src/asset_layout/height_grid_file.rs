//! 高さ格子のファイル1本を表す型。パスを1回だけ持ち、揃っているかの確認・内容ハッシュの算出・書き出し・読み込みを
//! 同じパスの上で行う。
//!
//! チャンク1つにつきこの型を1回だけ作るのは、同じパスを操作のたびに組み直さないためである。
//! 書き出す側(生成器)と読み込む側(実行時形式のコンパイラ)が同じ型を通ることで、置き場の知識が2箇所へ割れない。

use std::path::PathBuf;

use crate::generation_ledger::{内容ハッシュ, 生成台帳エラー};
use crate::height_grid::{高さ格子, 高さ格子を読み込む, 高さ格子エラー};

#[repr(transparent)]
pub struct 高さ格子のファイル(PathBuf);

impl 高さ格子のファイル {
    /// 目録の1行のように、外からパスが分かっている場所で作る。置き場の型を通さない構築の口はこの1つである。
    pub fn パスから作る(パス: PathBuf) -> Self {
        Self(パス)
    }

    pub fn 揃っているか(&self) -> bool {
        self.0.is_file()
    }

    pub fn 内容ハッシュを求める(&self) -> Result<Option<内容ハッシュ>, 生成台帳エラー> {
        内容ハッシュ::ファイルがあれば中身から求める(&self.0)
    }

    pub fn 読み込む(&self) -> Result<高さ格子, 高さ格子エラー> {
        高さ格子を読み込む(&self.0)
    }

    /// カタログのソース依存一覧へ載せるためのパスの写し。生値へ戻す唯一の口である。
    pub fn パスの写しを作る(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn 書き出す(&self, バイト列: &[u8]) -> Result<(), String> {
        std::fs::write(&self.0, バイト列).map_err(|誤り| format!("{}: {誤り}", self.0.display()))
    }
}
