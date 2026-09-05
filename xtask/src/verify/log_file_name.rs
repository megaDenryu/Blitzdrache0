//! 検証のログのファイル名。受け取るのは無し、返すのは`<ブランチの名前>_<先端の短いハッシュ>_<yyyyMMdd-HHmmss>`に
//! 拡張子を付けた綴りである。切り離した状態ではブランチの名前が無いため、先端の短いハッシュから始まる綴りになる。
//!
//! ブランチと先端を道具がgitへ問い合わせて読むのは、呼び出し側に名前を書かせると書き忘れと取り違えを道具が止められない
//! ためである。ブランチの名前の`/`を`-`へ置き換えるのは、その綴りがパスの区切りとして解釈されるのを防ぐためである。

use std::process::Command;

use super::log_error::検証列の破れ;
use super::utc_moment::協定世界時の時刻;

const ブランチの名前の問い合わせ: &str = "--abbrev-ref";
const 先端の短いハッシュの問い合わせ: &str = "--short";

/// 切り離した状態のときにgitがブランチの名前として返す綴り。ブランチの名前ではないため、前置きに使わない。
const 切り離した状態の答え: &str = "HEAD";

pub struct 検証のログのファイル名 {
    綴り: String,
}

impl 検証のログのファイル名 {
    pub fn ブランチと先端と時刻から組み立てる() -> Result<Self, 検証列の破れ> {
        let ブランチの前置き = ブランチの名前を前置きへ写す(&ブランチまたは先端をgitから読む(ブランチの名前の問い合わせ)?);
        let 先端の短いハッシュ = ブランチまたは先端をgitから読む(先端の短いハッシュの問い合わせ)?;
        let 時刻 = 協定世界時の時刻::いまを読む()?.綴り();
        Ok(Self {
            綴り: format!("{ブランチの前置き}{先端の短いハッシュ}_{時刻}.log"),
        })
    }

    pub fn 綴り(&self) -> &str {
        &self.綴り
    }
}

/// `git rev-parse <切り替え> HEAD`の答えを1行として読む。切り替えがブランチの名前と先端の短いハッシュを選ぶ。
fn ブランチまたは先端をgitから読む(切り替え: &'static str) -> Result<String, 検証列の破れ> {
    let 結果 = Command::new("git").args(["rev-parse", 切り替え, "HEAD"]).output().map_err(|誤り| {
        検証列の破れ::ブランチまたは先端を読めなかった {
            問い合わせ: 切り替え, 誤り
        }
    })?;
    if !結果.status.success() {
        return Err(検証列の破れ::ブランチまたは先端の問い合わせが失敗した {
            問い合わせ: 切り替え,
            標準エラー: String::from_utf8_lossy(&結果.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&結果.stdout).trim().to_string())
}

/// gitが返したブランチの名前を、ファイル名の前置きへ写す。切り離した状態では前置きを空にする。
/// ビルド専用のチェックアウトは依頼された先端を切り離して置くため、`HEAD`という綴りが名前に残ると
/// どのブランチの検証だったのか読めない名前が並ぶ。
fn ブランチの名前を前置きへ写す(gitの答え: &str) -> String {
    if gitの答え == 切り離した状態の答え {
        return String::new();
    }
    format!("{}_", gitの答え.replace('/', "-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 切り離した状態ではブランチの前置きを付けない() {
        assert_eq!(ブランチの名前を前置きへ写す("HEAD"), "");
    }

    #[test]
    fn ブランチの名前の斜線を横棒へ置き換えて前置きにする() {
        assert_eq!(ブランチの名前を前置きへ写す("feat/verify-writes-log"), "feat-verify-writes-log_");
    }
}
