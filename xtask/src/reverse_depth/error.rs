//! 逆Z検収の工程ごとの失敗。

use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum 逆Z検収エラー {
    引数が不正(String),
    検証用アセットを生成できなかった,
    構築が失敗した(crate::release_build::計測用の構築の破れ),
    描画検収が失敗した(crate::acceptance::検収エラー),
    由来を書けなかった { パス: PathBuf, 誤り: std::io::Error },
    対照の由来が無い(PathBuf),
    対照の由来を読めなかった { パス: PathBuf, 誤り: std::io::Error },
    対照の深度契約が違う { 期待: &'static str, 実際: String },
    深度のバイト長が4の倍数でない(usize),
    深度の画素数が違う { 期待: usize, 実際: usize },
    判定が不合格(String),
}

impl From<crate::release_build::計測用の構築の破れ> for 逆Z検収エラー {
    fn from(理由: crate::release_build::計測用の構築の破れ) -> Self {
        Self::構築が失敗した(理由)
    }
}

impl From<crate::acceptance::検収エラー> for 逆Z検収エラー {
    fn from(理由: crate::acceptance::検収エラー) -> Self {
        Self::描画検収が失敗した(理由)
    }
}

impl std::fmt::Display for 逆Z検収エラー {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::引数が不正(引数) => write!(
                書き手,
                "引数は--capture-reference、--compare-candidate、--print-planのいずれか1つである: {引数}"
            ),
            Self::検証用アセットを生成できなかった => write!(書き手, "検証用アセットを生成できなかった"),
            Self::構築が失敗した(理由) => write!(書き手, "{理由}"),
            Self::描画検収が失敗した(理由) => write!(書き手, "{理由}"),
            Self::由来を書けなかった { パス, 誤り } => write!(書き手, "{}へ由来を書けなかった: {誤り}", パス.display()),
            Self::対照の由来が無い(パス) => write!(書き手, "反転前に--capture-referenceで{}を作る必要がある", パス.display()),
            Self::対照の由来を読めなかった { パス, 誤り } => write!(書き手, "{}の対照の由来を読めなかった: {誤り}", パス.display()),
            Self::対照の深度契約が違う { 期待, 実際 } => write!(書き手, "対照の深度契約が違う(期待「{期待}」、実際「{実際}」)"),
            Self::深度のバイト長が4の倍数でない(長さ) => write!(書き手, "深度のバイト長{長さ}が単精度の4バイト単位でない"),
            Self::深度の画素数が違う { 期待, 実際 } => write!(書き手, "深度の画素数が色の画素数と違う(期待{期待}、実際{実際})"),
            Self::判定が不合格(理由) => write!(書き手, "{理由}"),
        }
    }
}

impl std::error::Error for 逆Z検収エラー {}
