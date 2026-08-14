//! 候補の背景を4連結成分へ分け、画面外の空と繋がらない先読み縁の穴だけを数える。

use std::collections::VecDeque;

use super::image_pair::検収画像;

pub(super) fn 閉じた先読み縁の空隙画素を数える(対照: &検収画像, 候補: &検収画像) -> usize {
    let mut 訪問済み = vec![false; 候補.画素数()];
    let mut 空隙画素 = 0;
    for 始点 in 0..候補.画素数() {
        if 訪問済み[始点] || 候補.深度[始点] != 0.0 {
            continue;
        }
        let 成分 = 背景成分を辿る(対照, 候補, 始点, &mut 訪問済み);
        if !成分.画面端へ接する && 成分.近傍あり && 成分.遠景あり {
            空隙画素 += 成分.画素数;
        }
    }
    空隙画素
}

struct 背景成分 {
    画素数: usize,
    画面端へ接する: bool,
    近傍あり: bool,
    遠景あり: bool,
}

fn 背景成分を辿る(対照: &検収画像, 候補: &検収画像, 始点: usize, 訪問済み: &mut [bool]) -> 背景成分 {
    let mut 待ち = VecDeque::from([始点]);
    訪問済み[始点] = true;
    let mut 結果 = 背景成分 {
        画素数: 0,
        画面端へ接する: false,
        近傍あり: false,
        遠景あり: false,
    };
    while let Some(添字) = 待ち.pop_front() {
        結果.画素数 += 1;
        let (x, y) = (添字 % 候補.幅, 添字 / 候補.幅);
        結果.画面端へ接する |= x == 0 || y == 0 || x + 1 == 候補.幅 || y + 1 == 候補.高さ;
        for 隣 in 四近傍(候補.幅, 候補.高さ, x, y) {
            if 候補.深度[隣] == 0.0 {
                if !訪問済み[隣] {
                    訪問済み[隣] = true;
                    待ち.push_back(隣);
                }
            } else if 対照.深度[隣] > 0.0 {
                結果.近傍あり = true;
            } else {
                結果.遠景あり = true;
            }
        }
    }
    結果
}

fn 四近傍(幅: usize, 高さ: usize, x: usize, y: usize) -> impl Iterator<Item = usize> {
    let mut 一覧 = [None; 4];
    一覧[0] = (x > 0).then_some(y * 幅 + x.saturating_sub(1));
    一覧[1] = (x + 1 < 幅).then_some(y * 幅 + x + 1);
    一覧[2] = (y > 0).then_some(y.saturating_sub(1) * 幅 + x);
    一覧[3] = (y + 1 < 高さ).then_some((y + 1) * 幅 + x);
    一覧.into_iter().flatten()
}
