//! 掃除の対象: 消してよいビルドの中間データの置き場1つを表す型。役割の呼び名と置き場のパスを組で持つ。
//!
//! 呼び名とパスを裸の対で持ち回らないのは、どちらも文字列であり、取り違えても型が通るためである。
//! 取り違えると、消した置き場の呼び名が別の置き場のものになり、報告を読む人が何を失ったか分からない。
//!
//! 消してよいと言えるのは、失うものが「次の1回のビルドが差分でなく全部になること」だけの置き場に限る。
//! この型の値を作る場所は`collect`の1箇所であり、そこがその判断を持つ。

use std::path::{Path, PathBuf};

use super::error::掃除の破れ;
use super::occupied_size::占める容量;

pub(super) struct 掃除の対象 {
    役割の呼び名: String,
    パス: PathBuf,
}

impl 掃除の対象 {
    pub(super) fn 生成する(役割の呼び名: String, パス: PathBuf) -> Self {
        Self { 役割の呼び名, パス }
    }

    pub(super) fn 実在するか(&self) -> bool {
        self.パス.is_dir()
    }

    pub(super) fn 役割の呼び名(&self) -> &str {
        &self.役割の呼び名
    }

    pub(super) fn パス(&self) -> &Path {
        &self.パス
    }

    pub(super) fn 容量を測る(&self) -> Result<占める容量, 掃除の破れ> {
        占める容量::ディレクトリを測る(&self.パス)
    }

    /// 置き場を丸ごと消す。まだ無いことは破れではない。
    pub(super) fn 消す(&self) -> Result<(), 掃除の破れ> {
        match std::fs::remove_dir_all(&self.パス) {
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(誤り) => Err(掃除の破れ::ディレクトリを消せなかった {
                ディレクトリ: self.パス.clone(),
                誤り,
            }),
            Ok(()) => Ok(()),
        }
    }
}
