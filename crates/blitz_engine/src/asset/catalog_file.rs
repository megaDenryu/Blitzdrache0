//! 版付きカタログのファイル1本を指す型。読むと、生成物の相対パスを自分の並ぶディレクトリで解決した
//! カタログになる。
//!
//! 相対パスの解決をこの型が持つのは、基準がそのカタログの並ぶ場所そのものだからである。基準を読み手が
//! 覚える形にすると、置き場を移した実行でカタログだけが見つかって生成物が見つからない。

use std::path::PathBuf;

use super::catalog::カタログ;
use super::catalog_load_error::実行時カタログ読込エラー;
use super::runtime_file::実行時形式のファイル;
use super::runtime_format::実行時形式からカタログを読む;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 実行時カタログのファイル(実行時形式のファイル);

impl 実行時カタログのファイル {
    pub fn パスから作る(パス: PathBuf) -> Self {
        Self(実行時形式のファイル::パスから作る(パス))
    }

    pub fn 読み込む(&self) -> Result<カタログ, 実行時カタログ読込エラー> {
        let 格納値 = 実行時形式からカタログを読む(&self.0.バイト列を読む()?)?;
        let 基準 = self.0.並ぶディレクトリ();
        let mut 解決済み = カタログ::空を作る();
        for (id, 項目) in 格納値.全項目を走査する() {
            let 実行時パス = if 項目.実行時パス().is_absolute() {
                項目.実行時パス().to_path_buf()
            } else {
                基準.join(項目.実行時パス())
            };
            解決済み.詳細を登録する(id.clone(), 実行時パス, 項目.ソース依存一覧().to_vec(), 項目.メタデータ());
        }
        Ok(解決済み)
    }

    pub fn 表示の綴り(&self) -> std::path::Display<'_> {
        self.0.表示の綴り()
    }
}
