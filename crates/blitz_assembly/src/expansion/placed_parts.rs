//! 据えた部品の列: 展開の途中で既に配置が決まった部品を、据えた順に保つ型。
//!
//! 可変の列を型の中へ閉じるのは、展開の途中の半端な状態(まだ全部の部品が据わっていない列)を
//! 呼び出し側へ見せないためである。外から見えるのは、番号で1件引くことと、最後に配置表へ畳むことだけである。

use blitz_engine::個体配置;

use super::super::part::部品ID;
use super::instruction::据えた部品の番号;
use super::placement_table::部品ごとの配置表;

/// 据わった部品1件。どの部品がどこに置かれたかの組である。
pub(super) struct 据えた部品 {
    識別子: 部品ID,
    配置: 個体配置,
}

impl 据えた部品 {
    pub(super) fn 識別子(&self) -> &部品ID {
        &self.識別子
    }

    pub(super) fn 配置(&self) -> 個体配置 {
        self.配置
    }
}

pub(super) struct 据えた部品の列 {
    一覧: Vec<据えた部品>,
}

impl 据えた部品の列 {
    pub(super) fn 根から始める(根の識別子: 部品ID, 根の配置: 個体配置) -> Self {
        Self {
            一覧: vec![据えた部品 {
                識別子: 根の識別子,
                配置: 根の配置,
            }],
        }
    }

    /// まだ据わっていない番号を指されたらNoneを返す。指示が自分より後の部品を親に指す形をここで止める。
    pub(super) fn 番号で引く(&self, 番号: 据えた部品の番号) -> Option<&据えた部品> {
        self.一覧.get(番号.添字())
    }

    pub(super) fn 据えた件数(&self) -> usize {
        self.一覧.len()
    }

    pub(super) fn 部品を1件据える(&mut self, 識別子: 部品ID, 配置: 個体配置) {
        self.一覧.push(据えた部品 { 識別子, 配置 });
    }

    /// 据えた順の列を、部品ごとにまとめた配置表へ畳む。ここから先は据えた順が見えなくなる。
    pub(super) fn 配置表へ畳む(self) -> 部品ごとの配置表 {
        let mut 表 = 部品ごとの配置表::空から始める();
        for 部品 in self.一覧 {
            表.配置を1件足す(部品.識別子, 部品.配置);
        }
        表
    }
}
