//! 到達可能な状態一覧とは、初期状態から遷移を辿って実際に到達できる状態の集合のことである。
//!
//! Rustの型として組める状態と、アプリケーションが実際に到達する状態を区別するために置く。
//! モデルの遷移の後状態に現れるだけで初期状態から辿り着けない状態は、この一覧へ入らない。
//! 参照: `_doc/設計/設計オントロジー.md` 3.5.8。

use std::collections::{HashSet, VecDeque};

use super::transition::遷移;
use crate::design_model::設計概念の識別子;

/// 初期状態から辿り着ける状態を、辿った順に持つ一覧。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct 到達可能な状態一覧 {
    到達順の一覧: Vec<設計概念の識別子>,
}

impl 到達可能な状態一覧 {
    /// 初期状態から幅優先で辿って組む。同じ状態を2度は辿らない。
    pub(super) fn 幅優先で組む(初期状態一覧: &[設計概念の識別子], 遷移一覧: &[遷移]) -> Self {
        let mut 済み: HashSet<設計概念の識別子> = 初期状態一覧.iter().cloned().collect();
        let mut 待ち: VecDeque<設計概念の識別子> = 初期状態一覧.iter().cloned().collect();
        let mut 到達順の一覧 = Vec::new();
        while let Some(いま) = 待ち.pop_front() {
            for 候補 in 遷移一覧.iter().filter(|候補| 候補.前状態() == &いま) {
                if 済み.insert(候補.後状態().clone()) {
                    待ち.push_back(候補.後状態().clone());
                }
            }
            到達順の一覧.push(いま);
        }
        Self { 到達順の一覧 }
    }

    /// その状態へ到達できるか。振る舞いの原子命題`到達可能である`がこの問いに落ちる。
    pub fn 含むか(&self, 状態: &設計概念の識別子) -> bool {
        self.到達順の一覧.iter().any(|到達した| 到達した == 状態)
    }

    /// 到達できる状態の件数。有限全数の証拠が見た対象の件数としてこの数を書く。
    pub fn 件数(&self) -> usize {
        self.到達順の一覧.len()
    }

    /// 辿った順の一覧。全称の量化がこの並びを読むため、同じモデルなら常に同じ並びになる。
    pub fn 到達順の一覧(&self) -> &[設計概念の識別子] {
        &self.到達順の一覧
    }
}
