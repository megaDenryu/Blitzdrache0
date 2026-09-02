//! `--cloth-xpbd-reference-shape <綴り>`が選ぶ、XPBDの参照比較の布の敷き方と固定の形。垂直に吊るした平らな布は面の内側の力しか受けず
//! 曲げ拘束が働かないため、曲げを突き合わせる題材は水平に敷いた布を縁や一点で固定して垂れ落ちさせる(Issue #38「検証シーン」の面)。
//! 二辺の固定に上端の行と右端の列を選ぶのは、掴みの枠が下端の左端にあり、左端の列を固定すると同じ粒子を2本の目標拘束が持つためである。

use super::argument_error::起動引数エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 参照比較の題材の形 {
    垂直に吊るして上端の行を固定,
    水平に敷いて上端の行を固定,
    水平に敷いて上端の行と右端の列を固定,
    水平に敷いて上端の左の一点を固定,
}

impl 参照比較の題材の形 {
    /// `--cloth-xpbd-reference <コンプライアンス>`だけを指定した起動が採る形。実装順3から続く吊るし布の題材である。
    pub(crate) fn 既定() -> Self {
        Self::垂直に吊るして上端の行を固定
    }

    pub(crate) fn 綴りから解析する(綴り: &str) -> Result<Self, 起動引数エラー> {
        match 綴り {
            "vertical-top-row" => Ok(Self::垂直に吊るして上端の行を固定),
            "horizontal-top-row" => Ok(Self::水平に敷いて上端の行を固定),
            "horizontal-two-edges" => Ok(Self::水平に敷いて上端の行と右端の列を固定),
            "horizontal-one-point" => Ok(Self::水平に敷いて上端の左の一点を固定),
            _ => Err(起動引数エラー::参照比較の題材の形不正(format!(
                "vertical-top-row / horizontal-top-row / horizontal-two-edges / horizontal-one-point のいずれかでない({綴り})"
            ))),
        }
    }

    /// 報告の行へ書く綴り。`綴りから解析する`の逆である。
    pub(crate) fn 綴り(self) -> &'static str {
        match self {
            Self::垂直に吊るして上端の行を固定 => "vertical-top-row",
            Self::水平に敷いて上端の行を固定 => "horizontal-top-row",
            Self::水平に敷いて上端の行と右端の列を固定 => "horizontal-two-edges",
            Self::水平に敷いて上端の左の一点を固定 => "horizontal-one-point",
        }
    }
}
