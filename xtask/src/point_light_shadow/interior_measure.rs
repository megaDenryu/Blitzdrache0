//! 屋内の世界を影付き3件・影なし・灯を消した対照の3条件で撮り、領域ごとの平均輝度を採る工程。
//! 受け取るのは出力先、返すのは領域ごとの3つの平均輝度と書き出した2枚の絵のパスである。判定は1つも持たない。
//!
//! 対照を「影付きのまま灯を0件にした絵」でなく「灯を0件にした絵」で撮るのは、点光源の寄与がちょうど0の絵が要るためである。
//! 外側の領域が対照と一致することが、壁を抜けていた漏光がちょうど0になったことの反証になる。

use std::path::PathBuf;

use crate::acceptance::{判定の破れ, 描画検収の実行環境, 検収の実行名, 検収エラー};
use crate::multi_light_world::{run, world};

use super::interior_region::屋内の判定領域の一覧;
use super::region::矩形の平均輝度を採る;

pub(super) struct 屋内の領域ごとの平均輝度 {
    pub(super) 名前: &'static str,
    pub(super) 影付き3件: f64,
    pub(super) 影なし: f64,
    pub(super) 灯を消した対照: f64,
}

pub(super) struct 屋内の測り {
    pub(super) 領域一覧: Vec<屋内の領域ごとの平均輝度>,
    pub(super) 影付きの絵: PathBuf,
    pub(super) 影なしの絵: PathBuf,
}

pub(super) fn 屋内を3条件で撮る(実行環境: &描画検収の実行環境) -> Result<屋内の測り, 検収エラー> {
    let (影付き, 影付きの絵) = 屋内を一条件で撮る(実行環境, "hut_shadow_lit", &["--point-light-shadow-count", "3"])?;
    let (影なし, 影なしの絵) = 屋内を一条件で撮る(実行環境, "hut_shadow_none", &["--point-light-shadow-count", "0"])?;
    let (対照, _) = 屋内を一条件で撮る(実行環境, "hut_unlit", &["--local-light-count", "0"])?;
    let 三条件 = 影付き.into_iter().zip(影なし).zip(対照);
    let 領域一覧 = 屋内の判定領域の一覧
        .iter()
        .zip(三条件)
        .map(|(領域, ((影付き3件, 影なし), 灯を消した対照))| 屋内の領域ごとの平均輝度 {
            名前: 領域.名前,
            影付き3件,
            影なし,
            灯を消した対照,
        })
        .collect();
    Ok(屋内の測り {
        領域一覧,
        影付きの絵,
        影なしの絵,
    })
}

fn 屋内を一条件で撮る(
    実行環境: &描画検収の実行環境, 名前: &str, 追加の選択肢: &[&str]
) -> Result<(Vec<f64>, PathBuf), 検収エラー> {
    let 結果 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(名前)?,
        &run::起動指定を組み立てる(world::屋内のシーン, world::絵の枚数, 追加の選択肢),
    )?;
    let 平均輝度一覧 = 屋内の判定領域の一覧
        .iter()
        .map(|領域| 矩形の平均輝度を採る(結果.画像(), &領域.矩形))
        .collect::<Result<Vec<f64>, 判定の破れ>>()?;
    Ok((平均輝度一覧, 結果.書き出し先().目視用の絵へ変換する()?))
}
