//! 掃除の対象: 消してよいビルドの中間データの置き場1つを表す型。役割と置き場のパスを組で持つ。
//!
//! 役割とパスを裸の対で持ち回らないのは、報告の1行が役割とパスの両方を名指すためである。組で持てば、
//! 役割だけを別の対象のものと取り違えた行を書けない。
//!
//! 消してよいと言えるのは、失うものが「次の1回のビルドが差分でなく全部になること」だけの置き場に限る。
//! この型の値を作る場所は`collect`の1箇所であり、そこがその判断を持つ。

use std::path::{Path, PathBuf};

use super::error::掃除の破れ;
use super::occupied_size::占める容量;
use super::role::掃除の対象の役割;

pub(super) struct 掃除の対象 {
    役割: 掃除の対象の役割,
    パス: PathBuf,
}

impl 掃除の対象 {
    pub(super) fn 生成する(役割: 掃除の対象の役割, パス: PathBuf) -> Self {
        Self { 役割, パス }
    }

    pub(super) fn 実在するか(&self) -> bool {
        self.パス.is_dir()
    }

    pub(super) fn 役割(&self) -> 掃除の対象の役割 {
        self.役割
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
