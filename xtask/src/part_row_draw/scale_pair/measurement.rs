//! 並び1つぶんの実測: 焼く工程が数えた勘定と、走らせたアプリが出したシーンパスの数と、書き出した絵の置き場。
//!
//! 4つを1つの型が持つのは、どれも同じ1つの並びのものであることを判定が前提にするためである。別々に返すと、
//! 件数の少ないほうの勘定と多いほうの計数を突き合わせる呼び方が書ける。

use std::path::PathBuf;

use super::super::row_target::検収する並び1つ;
use crate::part_row_draw::tally_line::焼いた並びの勘定;
use crate::report_parse::計数報告;

pub(in crate::part_row_draw) struct 並び1つの実測 {
    pub(super) 並び: 検収する並び1つ,
    pub(super) 勘定: 焼いた並びの勘定,
    pub(super) 計数: 計数報告,
    pub(super) 絵: PathBuf,
}

impl 並び1つの実測 {
    /// シーンパスが1フレームに積んだ描画発行の回数。
    pub(super) fn シーンパスの発行数(&self) -> u64 {
        self.計数.シーン.発行数
    }

    /// 可視判定へ登録された個体の総数。視錐台に写ったかどうかに依らないため、焼いた個体数と直に比べられる。
    pub(super) fn 群の個体の候補数(&self) -> u64 {
        self.計数.可視個体の選別.候補数
    }

    /// 群でない描画対象の数。シーンパスの候補数と可視個体の選別の候補数の差であり、この世界では地面の1だけである。
    pub(super) fn 群でない描画対象の数(&self) -> u64 {
        self.計数.シーン.候補数.saturating_sub(self.計数.可視個体の選別.候補数)
    }

    pub(in crate::part_row_draw) fn 要約の1行(&self) -> String {
        format!(
            "{}は発行数{}・部品の種類{}・材質スロット総和{}・個体{}体、絵は{}",
            self.並び.呼び名(),
            self.シーンパスの発行数(),
            self.勘定.部品の種類数,
            self.勘定.材質スロット数の総和,
            self.群の個体の候補数(),
            self.絵.display()
        )
    }
}
