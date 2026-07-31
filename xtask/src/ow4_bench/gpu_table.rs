//! パス別GPU時間の表から、予算と突き合わせる量を導く工程。受け取るのは標準出力、返すのはシーン描画・シャドウ合計・
//! 空の定常合計・定常合計・更新時合計である。表の綴りは`crates/blitz_app/src/reports.rs`の出力と一致させている。
//! 定常合計から「シャドウ合計」の行を除くのは、その行が距離区分ごとの4行から導いた重複であり足すと二重計上になるためである。
//! 透過率生成と多重散乱生成を除くのは、その2区間が大気の変わったフレームだけ走る更新時の量だからである。
//! 区間の台帳は`sky-lut`が持つものをそのまま読む(同じ集合を2箇所で決めない)。

#[cfg(test)]
mod parse_tests;

use super::section_parse::区画の行一覧;
use crate::sky_lut::gpu_time::{定常の区間一覧, 更新時の区間一覧};

const 見出し: &str = "パス別GPU時間";
const シーン描画の区間名: &str = "シーン描画";
const シャドウ合計の区間名: &str = "シャドウ合計";

pub(super) struct GPU時間 {
    pub(super) シーン描画ms: f64,
    pub(super) シャドウ合計ms: f64,
    /// 空を描かない条件では空の区間が1本も立たないため`None`。0を返すと「空が速い」と読めてしまうため不在で表す。
    pub(super) 空の定常合計ms: Option<f64>,
    /// 毎フレーム走る区間だけの合計。60fpsの1フレームと比べる値である。
    pub(super) 定常合計ms: f64,
    /// 大気が変わったフレームだけ走る2区間の合計。空を描かない条件では`None`。
    pub(super) 更新時合計ms: Option<f64>,
}

pub(super) fn 取り出す(標準出力: &str) -> Result<GPU時間, String> {
    let 表 = 表を読む(標準出力)?;
    let 定常 = 表.iter().filter(|(名前, _)| 定常合計へ入れるか(名前));
    Ok(GPU時間 {
        シーン描画ms: 区間を参照する(&表, シーン描画の区間名)?,
        シャドウ合計ms: 区間を参照する(&表, シャドウ合計の区間名)?,
        空の定常合計ms: 区間群の合計(&表, &定常の区間一覧)?,
        定常合計ms: 定常.map(|(_, ミリ秒)| *ミリ秒).sum(),
        更新時合計ms: 区間群の合計(&表, &更新時の区間一覧)?,
    })
}

fn 定常合計へ入れるか(名前: &str) -> bool {
    名前 != シャドウ合計の区間名 && !更新時の区間一覧.contains(&名前)
}

fn 表を読む(標準出力: &str) -> Result<Vec<(String, f64)>, String> {
    let mut 表 = Vec::new();
    for 行 in 区画の行一覧(標準出力, 見出し)? {
        let Some((名前, 残り)) = 行.trim_start().split_once(':') else {
            continue;
        };
        let 値 = 残り.split_whitespace().next().ok_or_else(|| format!("区間{名前}の行に値が無い"))?;
        let ミリ秒 = 値.parse().map_err(|誤り| format!("区間{名前}の値を数として読めない({行}): {誤り}"))?;
        表.push((名前.to_string(), ミリ秒));
    }
    Ok(表)
}

fn 区間を参照する(表: &[(String, f64)], 区間名: &str) -> Result<f64, String> {
    表.iter()
        .find(|(名前, _)| 名前 == 区間名)
        .map(|(_, ミリ秒)| *ミリ秒)
        .ok_or_else(|| format!("{見出し}の表に区間{区間名}が無い"))
}

/// 名前で挙げた区間の合計。そろって立つか1本も立たないかのどちらかであり、一部だけ立つのは構成が想定と
/// 違うことを意味するため、残りを0として足さず失敗にする。
fn 区間群の合計(表: &[(String, f64)], 名前一覧: &[&str]) -> Result<Option<f64>, String> {
    let 値一覧: Vec<Option<f64>> = 名前一覧.iter().map(|区間名| 区間を参照する(表, 区間名).ok()).collect();
    if 値一覧.iter().all(Option::is_none) {
        return Ok(None);
    }
    if 値一覧.iter().any(Option::is_none) {
        return Err(format!("{見出し}の表に区間{名前一覧:?}の一部しか無い"));
    }
    Ok(Some(値一覧.into_iter().flatten().sum()))
}
