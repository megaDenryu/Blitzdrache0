//! 発生源の画素集合から各画素までのチェビシェフ距離を求める工程。受け取るのは画面の寸法と発生源の印、
//! 返すのは画素ごとの距離である。
//!
//! チェビシェフ距離(縦横斜めのどれも1歩と数える距離)を採るのは、にじみの標本位置が縦横の軸に沿って
//! 並んでおり、横と縦の届く範囲が独立に同じだけ広がるためである。ユークリッド距離を使うと斜め方向の
//! 実際の到達を短く見積もることになる。
//!
//! 前向きと後ろ向きの2走査で厳密な距離が出る。各画素は自分より前に確定した4近傍だけを見ればよく、
//! 待ち行列を持たずに済む。発生源が1つも無い場合は全画素が到達不能として残り、上限との比較で必ず破れる。

const 到達不能: usize = usize::MAX;

pub(super) fn 発生源からのチェビシェフ距離(幅: usize, 高さ: usize, 発生源: &[bool]) -> Vec<usize> {
    let mut 距離: Vec<usize> = 発生源.iter().map(|源| if *源 { 0 } else { 到達不能 }).collect();
    for y in 0..高さ {
        for x in 0..幅 {
            let 最小 = 前の行と左の最小(&距離, 幅, x, y);
            縮める(&mut 距離, 幅, x, y, 最小);
        }
    }
    for y in (0..高さ).rev() {
        for x in (0..幅).rev() {
            let 最小 = 次の行と右の最小(&距離, 幅, 高さ, x, y);
            縮める(&mut 距離, 幅, x, y, 最小);
        }
    }
    距離
}

fn 前の行と左の最小(距離: &[usize], 幅: usize, x: usize, y: usize) -> usize {
    let mut 最小 = 到達不能;
    if y > 0 {
        最小 = 最小.min(行の左右を含む最小(距離, 幅, x, y - 1));
    }
    if x > 0 {
        最小 = 最小.min(距離[y * 幅 + x - 1]);
    }
    最小
}

fn 次の行と右の最小(距離: &[usize], 幅: usize, 高さ: usize, x: usize, y: usize) -> usize {
    let mut 最小 = 到達不能;
    if y + 1 < 高さ {
        最小 = 最小.min(行の左右を含む最小(距離, 幅, x, y + 1));
    }
    if x + 1 < 幅 {
        最小 = 最小.min(距離[y * 幅 + x + 1]);
    }
    最小
}

fn 行の左右を含む最小(距離: &[usize], 幅: usize, x: usize, 行: usize) -> usize {
    let 先頭 = 行 * 幅;
    let 左 = x.saturating_sub(1);
    let 右 = (x + 1).min(幅 - 1);
    距離[先頭 + 左..=先頭 + 右].iter().copied().min().unwrap_or(到達不能)
}

fn 縮める(距離: &mut [usize], 幅: usize, x: usize, y: usize, 近傍: usize) {
    if 近傍 == 到達不能 {
        return;
    }
    let 位置 = y * 幅 + x;
    if 近傍 + 1 < 距離[位置] {
        距離[位置] = 近傍 + 1;
    }
}
