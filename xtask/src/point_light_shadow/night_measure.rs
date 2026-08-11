//! 夜の世界を影付き0件・1件・2件の3条件で撮り、領域ごとの平均輝度と点光源の影の計器を採る工程。
//! 受け取るのは出力先、返すのは領域ごとの3つの平均輝度と条件ごとの計器と影付き2件の絵のパスである。判定は1つも持たない。
//!
//! 件数を0から2へ1件ずつ足すのは、どの領域がどの灯で暗くなるかを分けて読むためである。2件を一度に足すと、
//! 2つの影が同時に現れ、どちらの灯がどちらの領域を暗くしたのかを絵から決められない。

use std::path::PathBuf;

use crate::acceptance::{描画検収の実行環境, 検収の実行名};
use crate::multi_light_world::{run, world};

use super::instrument::{点光源の影の計器, 点光源の影の計器を取り出す};
use super::night_region::夜の判定領域の一覧;
use super::region::矩形の平均輝度を採る;

pub(super) struct 夜の領域ごとの平均輝度 {
    pub(super) 名前: &'static str,
    pub(super) 影付き0件: f64,
    pub(super) 影付き1件: f64,
    pub(super) 影付き2件: f64,
}

pub(super) struct 夜の測り {
    pub(super) 領域一覧: Vec<夜の領域ごとの平均輝度>,
    pub(super) 影付き2件の計器: 点光源の影の計器,
    pub(super) 絵: PathBuf,
}

pub(super) fn 夜の世界を影付きの件数3つで撮る(実行環境: &描画検収の実行環境) -> Result<夜の測り, String> {
    let (零件, _, _) = 夜を一条件で撮る(実行環境, 0)?;
    let (一件, _, _) = 夜を一条件で撮る(実行環境, 1)?;
    let (二件, 二件の計器, 絵) = 夜を一条件で撮る(実行環境, 2)?;
    let 三条件 = 零件.into_iter().zip(一件).zip(二件);
    let 領域一覧 = 夜の判定領域の一覧
        .iter()
        .zip(三条件)
        .map(|(領域, ((影付き0件, 影付き1件), 影付き2件))| 夜の領域ごとの平均輝度 {
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

fn 夜を一条件で撮る(
    実行環境: &描画検収の実行環境, 影付きの件数: usize
) -> Result<(Vec<f64>, 点光源の影の計器, PathBuf), String> {
    let 件数文字列 = 影付きの件数.to_string();
    let 実行名の綴り = format!("night_shadow_x{影付きの件数}");
    let 結果 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(&実行名の綴り)?,
        &run::起動指定を組み立てる(
            world::夜のシーン,
            world::絵の枚数,
            &[
                "--sky",
                "--time-of-day",
                world::夜の一日内秒,
                "--point-light-shadow-count",
                &件数文字列,
                "--report-draw-issue",
                "--report-memory",
            ],
        ),
    )?;
    let 平均輝度一覧 = 夜の判定領域の一覧
        .iter()
        .map(|領域| 矩形の平均輝度を採る(結果.画像(), &領域.矩形))
        .collect::<Result<Vec<f64>, String>>()?;
    let 計器 = 点光源の影の計器を取り出す(結果.報告(), 影付きの件数)?;
    Ok((平均輝度一覧, 計器, 結果.書き出し先().目視用の絵へ変換する()?))
}
