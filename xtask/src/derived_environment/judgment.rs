//! 派生表現報告の全項目の判定。受け取るのは読み取った報告、返すのは判定の要約か最初に見つけた違反である。
//!
//! 見るのは7つである。validationの指摘・定数環境の解析解との一致・全テクセルの健全性・粗さを上げたときの
//! ばらつきの単調な減り・最詳細段が遠方環境の複製であること・面境界の連続性・代表テクセルのCPU正本との一致である。

mod analytic;
mod representative;
mod soundness;

use super::rows::報告;
use super::thresholds::検証層が有効な語;

pub(super) struct 判定 {
    pub(super) 代表テクセル数: usize,
    pub(super) 最大相対誤差: f64,
    pub(super) 定数環境の最大相対誤差: f64,
    pub(super) 面境界の角あたりの最大変化: f64,
    pub(super) 検証の告知: String,
}

pub(super) fn 全項目を検査する(報告: &報告) -> Result<判定, String> {
    let 検証の告知 = 検証を検査する(報告)?;
    let 定数環境の最大相対誤差 = analytic::定数環境を検査する(報告)?;
    soundness::健全性を検査する(報告)?;
    analytic::ばらつきの単調な減りを検査する(報告)?;
    soundness::最詳細段の複製を検査する(報告)?;
    let 面境界の角あたりの最大変化 = soundness::面境界を検査する(報告)?;
    let 最大相対誤差 = representative::代表テクセルを検査する(報告)?;
    Ok(判定 {
        代表テクセル数: 報告.代表.len() + 報告.反射率積分表代表.len(),
        最大相対誤差,
        定数環境の最大相対誤差,
        面境界の角あたりの最大変化,
        検証の告知,
    })
}

/// 層が無い機材の0件を有効な機材の0件と読み違えないよう、状況の語を告知へ載せる。
fn 検証を検査する(報告: &報告) -> Result<String, String> {
    let Some((状況, 件数)) = &報告.検証 else {
        return Err("報告に検証層の行が無い".to_string());
    };
    if *件数 != 0 {
        return Err(format!("validation層が{件数}件の指摘を出した(状況={状況})"));
    }
    if 状況 != 検証層が有効な語 {
        return Ok(format!("validation層は{状況}のため指摘0件は保証にならない"));
    }
    Ok("validation層は有効で指摘0件だった".to_string())
}
