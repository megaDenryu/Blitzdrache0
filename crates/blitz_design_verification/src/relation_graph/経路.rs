//! 関係の辺だけを辿る経路の問い。グラフの組み立てと異なり、問い合わせごとに探索する。

use std::collections::HashSet;

use super::設計関係グラフ;
use crate::{設計概念への参照, 設計関係の種類};

impl 設計関係グラフ {
    /// その関係の種類の辺だけを辿って主語から目的語へ達する経路が在るか。幅優先で辿り、同じ節点を2度は辿らない。
    /// グラフの検証器が「AからBへ依存の経路が存在しない」を問うために要る。
    pub fn 経路が在るか(&self, 主語: &設計概念への参照, 種類の集合: &[設計関係の種類], 目的語: &設計概念への参照) -> bool {
        // 済みの鍵を参照そのものにするのは、種類の違う2つの端点が別の辺を持つためである。
        let mut 済み: HashSet<設計概念への参照> = HashSet::new();
        let mut 待ち = vec![主語.clone()];
        済み.insert(主語.clone());
        while let Some(いま) = 待ち.pop() {
            for 種類 in 種類の集合 {
                for 次 in self.目的語一覧(&いま, *種類) {
                    if 次 == 目的語 {
                        return true;
                    }
                    if 済み.insert(次.clone()) {
                        待ち.push(次.clone());
                    }
                }
            }
        }
        false
    }
}
