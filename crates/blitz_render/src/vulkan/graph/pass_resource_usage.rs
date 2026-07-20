//! パス宣言からクロージャを取り除いた、バリア導出専用の軽量ビュー。
//! バリア導出関数を副作用なしにするために分離する（記録クロージャは比較・複製できないため）。

use super::handle::{バッファハンドル, 画像ハンドル};
use super::pass::パス宣言;
use super::usage::{バッファ用途, 画像用途};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct パスリソース使用 {
    pub(crate) 名前: &'static str,
    pub(crate) 読み画像: Vec<(画像ハンドル, 画像用途)>,
    pub(crate) 書き画像: Vec<(画像ハンドル, 画像用途)>,
    pub(crate) 読みバッファ: Vec<(バッファハンドル, バッファ用途)>,
    pub(crate) 書きバッファ: Vec<(バッファハンドル, バッファ用途)>,
}

impl<'a> From<&パス宣言<'a>> for パスリソース使用 {
    fn from(パス: &パス宣言<'a>) -> Self {
        Self {
            名前: パス.名前,
            読み画像: パス.読み画像.clone(),
            書き画像: パス.書き画像.clone(),
            読みバッファ: パス.読みバッファ.clone(),
            書きバッファ: パス.書きバッファ.clone(),
        }
    }
}
