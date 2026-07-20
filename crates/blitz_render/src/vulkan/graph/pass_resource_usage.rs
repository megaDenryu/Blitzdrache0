//! パス宣言からクロージャを取り除いた、バリア導出専用の軽量ビュー。
//! バリア導出関数を副作用なしにするために分離する（記録クロージャは比較・複製できないため）。

use super::handle::画像ハンドル;
use super::pass::パス宣言;
use super::usage::画像用途;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct パスリソース使用 {
    pub(crate) 名前: &'static str,
    pub(crate) 読み: Vec<(画像ハンドル, 画像用途)>,
    pub(crate) 書き: Vec<(画像ハンドル, 画像用途)>,
}

impl<'a> From<&パス宣言<'a>> for パスリソース使用 {
    fn from(パス: &パス宣言<'a>) -> Self {
        Self { 名前: パス.名前, 読み: パス.読み.clone(), 書き: パス.書き.clone() }
    }
}
