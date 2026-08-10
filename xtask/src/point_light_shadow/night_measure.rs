//! 夜の世界を影付き0件・1件・2件の3条件で撮り、領域ごとの平均輝度と点光源の影の計器を採る工程。
//! 受け取るのは出力先、返すのは領域ごとの3つの平均輝度と条件ごとの計器と影付き2件の絵のパスである。判定は1つも持たない。
//!
//! 件数を0から2へ1件ずつ足すのは、どの領域がどの灯で暗くなるかを分けて読むためである。2件を一度に足すと、
//! 2つの影が同時に現れ、どちらの灯がどちらの領域を暗くしたのかを絵から決められない。

use std::path::{Path, PathBuf};

use crate::multi_light_world::{run, world};

use super::instrument::{点光源の影の計器, 計器を取り出す};
use super::night_region::判定領域の一覧;
use super::region::矩形の平均輝度を採る;

pub(super) struct 領域ごとの夜の平均輝度 {
    pub(super) 名前: &'static str,
    pub(super) 影付き0件: f64,
    pub(super) 影付き1件: f64,
    pub(super) 影付き2件: f64,
}

pub(super) struct 夜の測り {
    pub(super) 領域一覧: Vec<領域ごとの夜の平均輝度>,
    pub(super) 影付き2件の計器: 点光源の影の計器,
    pub(super) 絵: PathBuf,
}

pub(super) fn 夜を影付きの件数3つで撮る(出力先: &Path) -> Result<夜の測り, String> {
    let (零件, _, _) = 一条件を撮る(出力先, 0)?;
    let (一件, _, _) = 一条件を撮る(出力先, 1)?;
    let (二件, 二件の計器, 絵) = 一条件を撮る(出力先, 2)?;
    let 三条件 = 零件.into_iter().zip(一件).zip(二件);
    let 領域一覧 = 判定領域の一覧
        .iter()
        .zip(三条件)
        .map(|(領域, ((影付き0件, 影付き1件), 影付き2件))| 領域ごとの夜の平均輝度 {
            名前: 領域.名前,
            影付き0件,
            影付き1件,
            影付き2件,
        })
        .collect();
    Ok(夜の測り {
        領域一覧,
        影付き2件の計器: 二件の計器,
        絵,
    })
}

fn 一条件を撮る(出力先: &Path, 影付きの件数: usize) -> Result<(Vec<f64>, 点光源の影の計器, PathBuf), String> {
    let 件数文字列 = 影付きの件数.to_string();
    let 書き出し先 = 出力先.join(format!("night_shadow_x{影付きの件数}"));
    let 結果 = run::走らせる(&run::描画条件 {
        シーン名: world::夜のシーン,
        アセットルート: world::夜のアセットルート,
        枚数: world::絵の枚数,
        書き出し先: &書き出し先,
        追加引数: &[
            "--sky",
            "--time-of-day",
            world::夜の一日内秒,
            "--point-light-shadow-count",
            &件数文字列,
            "--report-draw-issue",
            "--report-memory",
        ],
    })?;
    let 平均輝度一覧 = 判定領域の一覧
        .iter()
        .map(|領域| 矩形の平均輝度を採る(&結果, &領域.矩形))
        .collect::<Result<Vec<f64>, String>>()?;
    let 計器 = 計器を取り出す(&結果.標準出力, 影付きの件数)?;
    Ok((平均輝度一覧, 計器, crate::raw_png::変換する(&書き出し先)?))
}
