//! コマンドを選んだ後の引数の聞き取り。担当するのは、引数定義の並びを1件ずつ人へ尋ね、答えを
//! コマンド行へ足す語の並びへ組み立てることである。1行の受け取りは`line_reader`が、入力1行から
//! 答えを読む規則は`answer_reading`が、画面へ出す文面は`prompt_text`が持つ。
//!
//! 入力の口を型が保つのは、必須の引数の聞き直しと、位置の引数を省いた後の飛ばしを、標準入力を
//! 使わずに試験できるようにするためである。

mod answer_reading;
mod line_reader;
mod prompt_text;
#[cfg(test)]
mod tests;

use crate::command_ui::command_catalog::引数定義;

use answer_reading::{引数の答え, 答えを読む};
pub(super) use line_reader::標準入力の読み手;
use line_reader::行の読み手;

/// 引数の定義を順に尋ね、コマンド行へ足す語の並びを組み立てる型。
/// 位置で意味が決まる引数を1つ省いたら後ろの位置がずれるため、省いたことをこの型が覚えて以降を飛ばす。
pub(super) struct 引数フォーム<読み手> {
    読み手: 読み手,
    位置の引数を省いたか: bool,
}

impl<読み手: 行の読み手> 引数フォーム<読み手> {
    pub(super) fn 生成する(読み手: 読み手) -> Self {
        Self {
            読み手,
            位置の引数を省いたか: false,
        }
    }

    /// 定義の並びの順に尋ね、コマンド行へ足す語の並びを返す。入力が閉じたらそこまでの語を返す。
    pub(super) fn 語一覧を尋ねる(&mut self, 定義一覧: &[引数定義]) -> Vec<String> {
        let mut 語一覧 = Vec::new();
        for 定義 in 定義一覧.iter().copied() {
            if self.位置の引数を省いたか && 定義.位置で意味が決まるか() {
                println!("{}", prompt_text::飛ばしたことの案内(定義));
                continue;
            }
            let Some(語) = self.引数1件を尋ねる(定義) else {
                break;
            };
            if 語.is_empty() && 定義.位置で意味が決まるか() {
                self.位置の引数を省いたか = true;
            }
            語一覧.extend(語);
        }
        語一覧
    }

    /// 1件の引数を、答えが定まるまで繰り返し尋ねる。入力が閉じたら無しを返す。
    fn 引数1件を尋ねる(&mut self, 定義: 引数定義) -> Option<Vec<String>> {
        loop {
            println!();
            println!("{}", prompt_text::案内文を組み立てる(定義));
            let 行 = self.読み手.一行受け取る()?;
            match 答えを読む(定義, &行) {
                引数の答え::語へ写した(語一覧) => return Some(語一覧),
                引数の答え::聞き直す(理由) => println!("  {理由}"),
            }
        }
    }
}
