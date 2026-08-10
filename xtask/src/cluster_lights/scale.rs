//! 光の件数を振って夜の世界を測る工程。受け取るのは出力先、返すのは件数ごとの結果の並びである。
//!
//! 同じ件数を3回走らせるのは、単発の実行から実行間の分散と実在する差を区別できないためである
//! (参照: `_doc/計画/評価軸.md`「3.4 計測の再現性」)。3回のうち中央の値を代表にする。
//!
//! 件数を1・8・32・64に取るのは、1件が第3段階までの既定と同じ形であり、64件が世界が宣言できる上限だからである。
//! あいだの2点は、件数を倍々に振ったときGPU時間が件数に比例するのか頭打ちになるのかを読むための点である。

use std::path::Path;

use super::assignment::割り当ての統計;
use super::gpu_time::区間の中央値;
use super::{assignment, gpu_time, run, world};

/// 振る光の件数。
pub(super) const 光の件数一覧: [usize; 4] = [1, 8, 32, 64];

/// 同じ件数を何回走らせるか。
pub(super) const 反復回数: usize = 3;

pub(super) struct 件数ごとの結果 {
    pub(super) 光の件数: usize,
    pub(super) 選別ms: f64,
    pub(super) シーン描画ms: f64,
    pub(super) 割り当て: 割り当ての統計,
}

pub(super) fn 測る(出力先: &Path) -> Result<Vec<件数ごとの結果>, String> {
    光の件数一覧.iter().map(|件数| 一件数を測る(出力先, *件数)).collect()
}

fn 一件数を測る(出力先: &Path, 光の件数: usize) -> Result<件数ごとの結果, String> {
    let 件数文字列 = 光の件数.to_string();
    let mut 選別一覧 = Vec::with_capacity(反復回数);
    let mut シーン描画一覧 = Vec::with_capacity(反復回数);
    let mut 最後の標準出力 = String::new();
    for 回 in 1..=反復回数 {
        let 書き出し先 = 出力先.join(format!("night_x{光の件数}_{回}"));
        let 結果 = run::走らせる(&run::描画条件 {
            シーン名: world::夜のシーン,
            アセットルート: world::夜のアセットルート,
            書き出し先: &書き出し先,
            追加引数: &[
                "--sky",
                "--time-of-day",
                world::夜の一日内秒,
                "--local-light-count",
                &件数文字列,
                "--report-gpu-times",
                "--report-cluster-assignment",
            ],
        })?;
        let 区間の中央値 { 選別ms, シーン描画ms } = gpu_time::取り出す(&結果.標準出力)?;
        選別一覧.push(選別ms);
        シーン描画一覧.push(シーン描画ms);
        最後の標準出力 = 結果.標準出力;
    }
    let 割り当て = assignment::取り出す(&最後の標準出力)?;
    宣言された件数を確かめる(&割り当て, 光の件数)?;
    Ok(件数ごとの結果 {
        光の件数,
        選別ms: 中央の値(&mut 選別一覧),
        シーン描画ms: 中央の値(&mut シーン描画一覧),
        割り当て,
    })
}

/// 指定した件数がそのままGPUの宣言件数になっているかを確かめる。ここが食い違うと、件数を振ったつもりの
/// 計測が別の件数の繰り返しになる。
fn 宣言された件数を確かめる(割り当て: &割り当ての統計, 光の件数: usize) -> Result<(), String> {
    let 宣言 = 割り当て.宣言光数;
    if 宣言 == u64::try_from(光の件数).unwrap_or(u64::MAX) {
        return Ok(());
    }
    Err(format!("指定した光の件数{光の件数}に対しGPUの宣言件数が{宣言}だった"))
}

/// 反復のうち中央の値。反復回数は奇数であるため、並べ替えた真ん中が1つに決まる。
fn 中央の値(一覧: &mut [f64]) -> f64 {
    一覧.sort_by(f64::total_cmp);
    一覧.get(一覧.len() / 2).copied().unwrap_or(f64::NAN)
}
