//! 由来ファイルから読み出した採取の旗の1行。所有するのは読み出し元のパスと旗の行の本文であり、
//! 欄を読む操作をこの型のメソッドにする。受け取るのは欄の前置き、返すのは読み取った入切である。
//!
//! パスを本文と一緒に持つのは、欄が読めなかったときに「どのファイルを採り直せばよいか」を破れが名指すためである。
//! 本文だけを自由関数へ配ると、破れの文面がどの採取の話なのかを言えなくなる。

use std::path::{Path, PathBuf};

use super::on_off::入切の綴りを読む;
use super::旗の行の前置き;
use crate::distant_view::error::採取の読み取りの破れ;

pub(super) struct 採取の旗の行 {
    パス: PathBuf,
    本文: String,
}

impl 採取の旗の行 {
    pub(super) fn 由来ファイルから読み出す(置き場: &Path, 名前: &str) -> Result<Self, 採取の読み取りの破れ> {
        let パス = 置き場.join(format!("{名前}.txt"));
        let 内容 = std::fs::read_to_string(&パス)
            .map_err(|誤り| 採取の読み取りの破れ::ファイルを読めなかった {
                パス: パス.clone(), 誤り
            })?;
        let 本文 = 内容
            .lines()
            .find_map(|行| 行.strip_prefix(旗の行の前置き))
            .ok_or_else(|| 採取の読み取りの破れ::採取の旗の行が無い { パス: パス.clone() })?
            .to_string();
        Ok(Self { パス, 本文 })
    }

    pub(super) fn 入切を読む(&self, 前置き: &'static str) -> Result<bool, 採取の読み取りの破れ> {
        let 綴り = self.本文.split_whitespace().find_map(|語| 語.strip_prefix(前置き)).ok_or_else(|| {
            採取の読み取りの破れ::採取の旗に欄が無い {
                パス: self.パス.clone(),
                欄: 前置き,
            }
        })?;
        入切の綴りを読む(綴り).ok_or_else(|| 採取の読み取りの破れ::採取の旗の欄が入でも切でもない {
            パス: self.パス.clone(),
            欄: 前置き,
            綴り: 綴り.to_string(),
        })
    }

    /// 欄が無ければ記録が無いものとして返す。欄を足す前に採った由来がそれであり、その欄を実際に要る段だけが
    /// 失敗にできるようにする。欄はあるのに綴りが不正なら、記録が無いのと同じには扱わず失敗にする。
    pub(super) fn 記録があれば入切を読む(&self, 前置き: &'static str) -> Result<Option<bool>, 採取の読み取りの破れ> {
        if !self.本文.split_whitespace().any(|語| 語.starts_with(前置き)) {
            return Ok(None);
        }
        self.入切を読む(前置き).map(Some)
    }
}
