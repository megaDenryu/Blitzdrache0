//! 派生表現報告の全文から、判定に要る行だけを取り出す工程。受け取るのは終了時報告、返すのは行の値を組んだ報告である。
//!
//! 行の形が並ぶ場所をここが単独で持つのは、これがblitz_appとxtaskの間の機械可読な契約そのものだからである。
//! 語の並びに依存せず鍵で参照するのは、報告へ項目を足したときに読み取り側が壊れないようにするためである。
//! 行の値の取り出しは遠方環境の検収が持つ`field`を共有する。
//!
//! 行の種類ごとに見出しと読み方を持つのは、判定が見る3つの切り口と同じ切り口だからである。
//! 解析解と突き合わせる行は`analytic_rows`、健全性と連続性を見る行は`soundness_rows`、
//! 代表テクセルの行は`representative_rows`が持つ。

mod analytic_rows;
mod representative_rows;
mod soundness_rows;

use crate::acceptance::{検収エラー, 終了時報告};
use crate::distant_environment::parse::field::{整数値, 語句値};
use crate::report_heading::報告の見出し;

use super::rows::報告;

const 検証層の見出し: 報告の見出し = 報告の見出し::定数から生成する("検証層");

pub(super) fn 報告を取り出す(終了時の報告: &終了時報告) -> Result<報告, 検収エラー> {
    Ok(報告 {
        検証: 検証を読む(終了時の報告)?,
        定数環境拡散: analytic_rows::拡散の一覧を読む(終了時の報告)?,
        定数環境鏡面: analytic_rows::鏡面の一覧を読む(終了時の報告)?,
        統計: soundness_rows::統計の一覧を読む(終了時の報告)?,
        分散: soundness_rows::ばらつきの一覧を読む(終了時の報告)?,
        最詳細段の不一致: soundness_rows::最詳細段の不一致を読む(終了時の報告)?,
        面境界: soundness_rows::面境界の一覧を読む(終了時の報告)?,
        代表: representative_rows::派生表現の代表の一覧を読む(終了時の報告)?,
        反射率積分表代表: representative_rows::反射率積分表の代表の一覧を読む(終了時の報告)?,
    })
}

/// 行が無いことを失敗と読み替えず、無いことを返す。層が無い機材で走らせた実行を判定が名指すためである。
fn 検証を読む(終了時の報告: &終了時報告) -> Result<Option<(String, usize)>, 検収エラー> {
    let 行一覧 = 終了時の報告.見出しで始まる行の一覧(&検証層の見出し);
    let Some(行) = 行一覧.first() else {
        return Ok(None);
    };
    Ok(Some((語句値(行, "状況")?, 整数値(行, "件数")?)))
}
