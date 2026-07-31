//! パス別GPU時間の表から、予算と突き合わせる4つの量を導く工程。受け取るのは標準出力、返すのは
//! シーン描画・シャドウ合計・空の定常合計・GPU合計である。表の綴りは`crates/blitz_app/src/reports.rs`の出力と一致させている。
//! GPU合計から「シャドウ合計」の行を除くのは、その行が距離区分ごとの4行から導いた重複であり、足すと二重計上になるためである。
//! 空の定常合計へ入れる区間の台帳は`sky-lut`が持つものをそのまま読む(同じ集合を2箇所で決めない)。

use super::section_parse::区画の行一覧;

const 見出し: &str = "パス別GPU時間";
const シーン描画の区間名: &str = "シーン描画";
/// 距離区分ごとの4行から導いた合計の行。予算(2.0msは4距離区分の合計)とそのまま突き合わせるために報告が足している。
const シャドウ合計の区間名: &str = "シャドウ合計";

pub(super) struct GPU時間 {
    pub(super) シーン描画ms: f64,
    pub(super) シャドウ合計ms: f64,
    /// 空を描かない条件では空の区間が1本も立たないため`None`。0を返すと「空が速い」と読めてしまうため不在で表す。
    pub(super) 空の定常合計ms: Option<f64>,
    pub(super) 合計ms: f64,
}

pub(super) fn 取り出す(標準出力: &str) -> Result<GPU時間, String> {
    let 表 = 表を読む(標準出力)?;
    let 合計ms = 表
        .iter()
        .filter(|(名前, _)| 名前 != シャドウ合計の区間名)
        .map(|(_, ミリ秒)| *ミリ秒)
        .sum();
    Ok(GPU時間 {
        シーン描画ms: 区間を参照する(&表, シーン描画の区間名)?,
        シャドウ合計ms: 区間を参照する(&表, シャドウ合計の区間名)?,
        空の定常合計ms: 空の定常合計を求める(&表)?,
        合計ms,
    })
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

/// 空の定常区間は空を描く条件でそろって立ち、描かない条件では1本も立たない。一部だけが立つのは
/// 空パスの構成が想定と違うことを意味するため、残りを0として足さず失敗にする。
fn 空の定常合計を求める(表: &[(String, f64)]) -> Result<Option<f64>, String> {
    let 値一覧: Vec<Option<f64>> = crate::sky_lut::gpu_time::定常の区間一覧
        .iter()
        .map(|区間名| 区間を参照する(表, 区間名).ok())
        .collect();
    if 値一覧.iter().all(Option::is_none) {
        return Ok(None);
    }
    if 値一覧.iter().any(Option::is_none) {
        return Err(format!("{見出し}の表に空の定常区間の一部しか無い"));
    }
    Ok(Some(値一覧.into_iter().flatten().sum()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const 空あり: &str = "パス別GPU時間(移動平均、60フレーム窓):\n  シーン描画: 1.0000 ms\n  シャドウ距離区分0: 0.5000 ms\n  シャドウ距離区分1: 0.2500 ms\n  スカイビュー生成: 0.0200 ms\n  空中遠近生成: 0.0300 ms\n  空中遠近合成: 0.0200 ms\n  空: 0.0100 ms\n  シャドウ合計: 0.7500 ms\n";
    const 空なし: &str = "パス別GPU時間:\n  シーン描画: 1.0 ms\n  シャドウ距離区分0: 0.5 ms\n  シャドウ合計: 0.5 ms\n";

    /// 合計は距離区分ごとの行を足し、導出である「シャドウ合計」の行を二重に足さない。
    #[test]
    fn 合計はシャドウ合計の行を除く() {
        let 時間 = 取り出す(空あり).unwrap();
        assert!((時間.合計ms - 1.83).abs() < 1.0e-9, "{}", 時間.合計ms);
        assert!((時間.シャドウ合計ms - 0.75).abs() < 1.0e-9);
        assert!((時間.空の定常合計ms.unwrap() - 0.08).abs() < 1.0e-9);
    }

    /// 空を描かない条件は空の不在を返し、0ミリ秒とは区別する。
    #[test]
    fn 空の区間が1本も無ければ不在を返す() {
        assert_eq!(取り出す(空なし).unwrap().空の定常合計ms, None);
    }

    #[test]
    fn 空の区間が一部しか無ければ失敗にする() {
        let 一部 = "パス別GPU時間:\n  シーン描画: 1.0 ms\n  空: 0.01 ms\n  シャドウ合計: 0.5 ms\n";
        assert!(取り出す(一部).is_err());
    }
}
