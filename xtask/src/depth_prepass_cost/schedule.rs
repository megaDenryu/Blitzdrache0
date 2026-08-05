//! どの条件をどの順で起動するかを決める工程。受け取るものは無く、返すのは周回の位置を付けた条件の並びである。
//!
//! 起動の順をラテン方陣の回転(ABC・BCA・CAB)にするのは、周回の中の温まりとドリフトが条件に交絡するためである。
//! 毎周ABCの順で回すと、Aは常に冷えた状態で、Cは常に温まった状態で測ることになり、条件の差と順序の効果を分けられない。
//! 回転させると各条件が先頭・2番目・3番目をちょうど1回ずつ取るため、順序の効果が3条件へ等しく配られる。
//!
//! 周回番号と順序位置を生標本へ残すのは、順序の効果が残っていないかを後から読み手が確かめられるようにするためである。

#[cfg(test)]
mod schedule_tests;

/// その実行が使う深度プリパスの方式。値はblitz_appの`--depth-prepass`へそのまま渡る語である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 方式 {
    積まない,
    色は同値以下,
    色は等値,
}

impl 方式 {
    pub(super) const fn 起動指定の語(self) -> &'static str {
        match self {
            Self::積まない => "none",
            Self::色は同値以下 => "less-equal",
            Self::色は等値 => "equal",
        }
    }

    pub(super) const fn 深度プリパスを積むか(self) -> bool {
        match self {
            Self::積まない => false,
            Self::色は同値以下 | Self::色は等値 => true,
        }
    }
}

pub(super) struct 実行条件 {
    pub(super) 名前: &'static str,
    pub(super) 方式: 方式,
}

/// 測る3条件。同値性の検収もこの並びをそのまま使う。
pub(super) const 三条件: [実行条件; 3] = [
    実行条件 {
        名前: "A",
        方式: 方式::積まない,
    },
    実行条件 {
        名前: "B",
        方式: 方式::色は同値以下,
    },
    実行条件 {
        名前: "C",
        方式: 方式::色は等値,
    },
];

/// 回す周回の数。条件の数と同じにするのは、各条件が全部の順序位置をちょうど1回ずつ取るためである。
pub(super) const 周回数: usize = 三条件.len();

/// 1回の起動が周回のどこに置かれたか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 周回の位置 {
    pub(super) 周回番号: usize,
    pub(super) 順序位置: usize,
}

/// 起動する順の並び。周回ごとに開始条件を1つずらす。
pub(super) fn 起動の並び() -> Vec<(周回の位置, &'static 実行条件)> {
    let mut 並び = Vec::with_capacity(周回数 * 三条件.len());
    for 周回番号 in 0..周回数 {
        for 順序位置 in 0..三条件.len() {
            let 添字 = (周回番号 + 順序位置) % 三条件.len();
            並び.push((周回の位置 { 周回番号, 順序位置 }, &三条件[添字]));
        }
    }
    並び
}
