//! 検証のログのファイル名。受け取るのは無し、返すのは`<枝の名前>_<先端の短いハッシュ>_<yyyyMMdd-HHmmss>`に
//! 拡張子を付けた綴りである。
//!
//! 枝と先端を道具がgitへ問い合わせて読むのは、呼び出し側に名前を書かせると書き忘れと取り違えを道具が止められない
//! ためである。枝の名前の`/`を`-`へ置き換えるのは、その綴りがパスの区切りとして解釈されるのを防ぐためである。

use std::process::Command;

use super::log_error::検証列の破れ;
use super::utc_moment::協定世界時の時刻;

const 枝の名前の問い合わせ: &str = "--abbrev-ref";
const 先端の短いハッシュの問い合わせ: &str = "--short";

pub struct 検証のログのファイル名 {
    綴り: String,
}

impl 検証のログのファイル名 {
    pub fn 枝と先端と時刻から組み立てる() -> Result<Self, 検証列の破れ> {
        let 枝の名前 = 枝または先端をgitから読む(枝の名前の問い合わせ)?.replace('/', "-");
        let 先端の短いハッシュ = 枝または先端をgitから読む(先端の短いハッシュの問い合わせ)?;
        let 時刻 = 協定世界時の時刻::いまを読む()?.綴り();
        Ok(Self {
            綴り: format!("{枝の名前}_{先端の短いハッシュ}_{時刻}.log"),
        })
    }

    pub fn 綴り(&self) -> &str {
        &self.綴り
    }
}

/// `git rev-parse <切り替え> HEAD`の答えを1行として読む。切り替えが枝の名前と先端の短いハッシュを選ぶ。
fn 枝または先端をgitから読む(切り替え: &'static str) -> Result<String, 検証列の破れ> {
    let 結果 = Command::new("git").args(["rev-parse", 切り替え, "HEAD"]).output().map_err(|誤り| {
        検証列の破れ::枝または先端を読めなかった {
            問い合わせ: 切り替え, 誤り
        }
    })?;
    if !結果.status.success() {
        return Err(検証列の破れ::枝または先端の問い合わせが失敗した {
            問い合わせ: 切り替え,
            標準エラー: String::from_utf8_lossy(&結果.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&結果.stdout).trim().to_string())
}
