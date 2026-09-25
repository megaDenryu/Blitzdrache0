//! 斜線で揃えたパスとは、パスの `.` と `..` を解き、区切りを斜線へ揃えてつないだ表記のことである。
//! 2つのパスが同じファイルを指すかを、実行環境の区切りと宣言の中の表記の書き方に左右されずに比べるために使う。
//! ファイルシステムには問い合わせない。シンボリックリンクと大文字小文字の違いは同じファイルと見ない。

use std::fmt;
use std::path::{Component, Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 斜線で揃えたパス(String);

impl 斜線で揃えたパス {
    pub fn 生成する(パス: &Path) -> Self {
        let mut 部品一覧: Vec<String> = Vec::new();
        for 部品 in パス.components() {
            match 部品 {
                Component::CurDir => {}
                Component::ParentDir => {
                    部品一覧.pop();
                }
                _ => 部品一覧.push(部品.as_os_str().to_string_lossy().replace('\\', "/")),
            }
        }
        Self(部品一覧.join("/"))
    }
}

impl fmt::Display for 斜線で揃えたパス {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
