//! 各点から近い点への拘束の対を選ぶ工程。受け取るのは生成器と位置の並びと本数の上限、返すのは重複の無い点の対である。
//! 点ごとの本数を生成器で決めるため、接続数は点ごとに違う。同じ対を2度張らないよう、小さい添字を先にして集合で畳む。
//! 距離の比較は全点どうしで行う(点の数の2乗の計算量)。計測の題材は千点の桁であり、生成は1度きりである。

use std::collections::BTreeSet;

use blitz_math::{ワールド, 位置, 決定的な値の生成器, 用途番号};

use super::super::point_index::点添字;

/// 本数を決める用途番号。散布(`scatter`)の1〜3と重ならない番号である。
const 本数の用途番号: u64 = 4;

/// 拘束で結ぶ2点。aはbより小さい添字である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct 点の対 {
    pub(super) a: 点添字,
    pub(super) b: 点添字,
}

pub(super) fn 近い点の対を選ぶ(
    生成器: &決定的な値の生成器, 位置一覧: &[位置<ワールド>], 上限: u32
) -> Vec<点の対> {
    let mut 対の集合 = BTreeSet::new();
    for (添字, 位置) in 位置一覧.iter().enumerate() {
        let 本数 = 本数を決める(生成器, 添字, 上限);
        for 相手 in 近い順の添字(位置一覧, 添字, 位置).into_iter().take(本数) {
            let (小, 大) = if 添字 < 相手 { (添字, 相手) } else { (相手, 添字) };
            対の集合.insert(点の対 {
                a: 添字へ(小),
                b: 添字へ(大),
            });
        }
    }
    対の集合.into_iter().collect()
}

/// 1から上限までの本数。生成器の値に上限を掛けて切り捨て、1を足す(上限を超えない)。
fn 本数を決める(生成器: &決定的な値の生成器, 添字: usize, 上限: u32) -> usize {
    let 値 = 生成器.零以上一未満(u64::try_from(添字).unwrap_or(0), 用途番号::番号から生成する(本数の用途番号));
    let 上限の実数 = f32::from(u16::try_from(上限).unwrap_or_else(|_| panic!("接続数の上限がu16に収まらない: {上限}")));
    let mut 本数 = 1u16;
    while u32::from(本数) < 上限 && 値 * 上限の実数 >= f32::from(本数) {
        本数 += 1;
    }
    usize::from(本数)
}

/// 自分以外の全点を距離の昇順に並べた添字。同じ距離は添字の昇順で決着する(決定性のため)。
fn 近い順の添字(位置一覧: &[位置<ワールド>], 自分: usize, 位置: &位置<ワールド>) -> Vec<usize> {
    let mut 候補: Vec<(f32, usize)> = 位置一覧
        .iter()
        .enumerate()
        .filter(|(添字, _)| *添字 != 自分)
        .map(|(添字, 相手)| ((*相手 - *位置).長さ().値(), 添字))
        .collect();
    候補.sort_by(|左, 右| 左.0.total_cmp(&右.0).then(左.1.cmp(&右.1)));
    候補.into_iter().map(|(_, 添字)| 添字).collect()
}

fn 添字へ(値: usize) -> 点添字 {
    点添字::生成する(u32::try_from(値).unwrap_or_else(|_| panic!("点の添字がu32に収まらない: {値}")))
}
