//! 実行時形式で焼かれたファイル1本を指す型。パスを1回だけ持ち、そのファイルの中身を読み出す。
//!
//! 裸の`&Path`で受け取らないのは、読み込みの口へソースのパスや出力先のパスを渡しても型が通るためである。
//! 種別ごとの読み口(カタログ・シーン・チャンク目録・高さ場)はこの型を保持し、自分の形式検査だけを足す。
//!
//! 注意: blitz_engineがファイルの中身を読むのはこの型の中だけである。読み口が自分で`std::fs`を呼ぶと、
//! ファイルシステムへ触る地点が形式の数だけ散り、どの経路がどのパスを開くのかが呼び出し側の記憶になる。

mod read_error;

use std::path::{Path, PathBuf};

pub use read_error::実行時形式のファイルの読み取りの破れ;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 実行時形式のファイル(PathBuf);

impl 実行時形式のファイル {
    /// カタログの登録や起動指定のように、外からパスが分かっている場所で作る。
    pub fn パスから作る(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// このファイルの中身。blitz_engineがファイルの中身を読む唯一の口である。
    pub(crate) fn バイト列を読む(&self) -> Result<Vec<u8>, 実行時形式のファイルの読み取りの破れ> {
        std::fs::read(&self.0).map_err(|事由| 実行時形式のファイルの読み取りの破れ::生成する(self.0.clone(), 事由))
    }

    /// このファイルが並ぶディレクトリ。カタログが生成物の相対パスを解決する基準になる。
    /// 親を持たない綴りは今の場所に並ぶものとして扱う。
    pub(crate) fn 並ぶディレクトリ(&self) -> PathBuf {
        self.0.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."))
    }

    /// 更新の監視へ載せるためのパスの写し。生値へ戻る唯一の口である。
    pub(crate) fn パスの写しを作る(&self) -> PathBuf {
        self.0.clone()
    }
}
