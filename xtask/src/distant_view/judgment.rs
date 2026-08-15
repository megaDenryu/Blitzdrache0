//! 工程Dのreference/candidateを2段で比較する入口。担当するのは「どの対へどの厳しさを課すか」だけであり、
//! 個々の条件は各工程が持つ。受け取るのは採取の置き場、返すのは2段の要約か最初に破れた条件である。
//!
//! 2段に分けるのは、近傍チャンクの幾何画素の不変を後処理込みでは原理的に課せないためである。光のにじみは
//! 画面内の演算子であり、遠景が地平の帯で空を地形へ置き換えるとその周囲の画素が量子化1段ぶん暗くなる。
//! 台は遠景の輪郭よりはるかに広く、輪郭の免除を広げても覆えない。そこで陰影そのものの不変は後処理を
//! 組まない対で厳密に課し、実際に出荷される後処理ありの対へは滲みとして説明できる形に収まっていることを課す。
//! 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点4

mod bloom_footprint;
mod color_difference;
mod contour;
mod gap;
#[cfg(test)]
mod gap_tests;
mod geometry_contract;
mod image_pair;
mod pixel_class;
mod shadow_stage;
mod shadow_visibility;

use std::path::Path;

use bloom_footprint::にじみの足跡;
use color_difference::許可域外の色差;
use image_pair::検収画像;
use pixel_class::画素の区分;

use super::provenance::採取の旗;

/// 影の距離区分の再配分の検査点。遠景の検査点と分けて呼ぶのは、空中遠近・遠景・影の変化を同じ比較へ混ぜないためである。
pub(super) fn 影を判定する(置き場: &Path) -> Result<String, String> {
    shadow_stage::判定する(置き場)
}

pub(super) fn 判定する(置き場: &Path) -> Result<String, String> {
    let 主判定 = 主判定を課す(置き場)?;
    let 有界検査 = 有界検査を課す(置き場)?;
    Ok(format!("主判定(後処理なし)は{主判定}、有界検査(後処理あり)は{有界検査}"))
}

/// 陰影そのものの不変を課す段。滲みが混ざらないため、近傍の色差は1画素も許さない。
fn 主判定を課す(置き場: &Path) -> Result<String, String> {
    let (対照, 候補, 区分) = 対を読む(置き場, "reference_no_post", "candidate_no_post", false)?;
    let 幾何 = geometry_contract::課す(&対照, &候補, &区分)?;
    let 色 = 許可域外の色差::集める(&対照, &候補, &区分).一画素も無いことを課す(&区分)?;
    Ok(format!("{幾何}・{色}"))
}

/// 出荷される構成のまま課す段。差が残ること自体は認め、滲みとして説明できる形に収まっていることを見る。
fn 有界検査を課す(置き場: &Path) -> Result<String, String> {
    let (対照, 候補, 区分) = 対を読む(置き場, "reference", "candidate", true)?;
    let 幾何 = geometry_contract::課す(&対照, &候補, &区分)?;
    let 発生源: Vec<bool> = (0..区分.画素数()).map(|添字| 区分.遠景が背景を置き換えたか(添字)).collect();
    let 足跡 = にじみの足跡::求める(対照.幅, 対照.高さ, &発生源);
    let 色 = 許可域外の色差::集める(&対照, &候補, &区分).有界であることを課す(&足跡)?;
    Ok(format!("{幾何}・{色}"))
}

/// 絵を読む前に採取の旗を照合する。段が要る構成と違う絵を突き合わせても、絵からはその食い違いが分からない。
fn 対を読む(
    置き場: &Path, 対照名: &str, 候補名: &str, 後処理を使うか: bool
) -> Result<(検収画像, 検収画像, 画素の区分), String> {
    for 名前 in [対照名, 候補名] {
        let 旗 = 採取の旗::由来ファイルから読む(置き場, 名前)?;
        if 旗.後処理を使うか() != 後処理を使うか {
            return Err(format!(
                "{名前}は後処理{}で採ってあり、この段が要る後処理{}の絵ではない",
                入切(旗.後処理を使うか()),
                入切(後処理を使うか)
            ));
        }
    }
    let 対照 = 検収画像::読む(置き場, 対照名)?;
    let 候補 = 検収画像::読む(置き場, 候補名)?;
    対照.同じ寸法を課す(&候補)?;
    let 区分 = 画素の区分::求める(&対照, &候補);
    Ok((対照, 候補, 区分))
}

fn 入切(使うか: bool) -> &'static str {
    if 使うか { "あり" } else { "なし" }
}
