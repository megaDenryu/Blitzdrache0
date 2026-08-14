//! 遠景の固定構図を採る工程ごとの失敗。

use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum 遠景構図の検収エラー {
    引数が不正(String),
    構築が失敗した(crate::release_build::計測用の構築の破れ),
    描画検収が失敗した(crate::acceptance::検収エラー),
    由来を書けなかった { パス: PathBuf, 誤り: std::io::Error },
    再撮影が一致しない(&'static str),
    構図契約が失敗した(String),
    判定が失敗した(String),
}

impl From<crate::release_build::計測用の構築の破れ> for 遠景構図の検収エラー {
    fn from(理由: crate::release_build::計測用の構築の破れ) -> Self {
        Self::構築が失敗した(理由)
    }
}

impl From<crate::acceptance::検収エラー> for 遠景構図の検収エラー {
    fn from(理由: crate::acceptance::検収エラー) -> Self {
        Self::描画検収が失敗した(理由)
    }
}

impl std::fmt::Display for 遠景構図の検収エラー {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::引数が不正(引数) => write!(書き手, "distant-viewの採取・診断・計画・判定の引数を1つだけ指定する必要がある: {引数}"),
            Self::構築が失敗した(理由) => write!(書き手, "{理由}"),
            Self::描画検収が失敗した(理由) => write!(書き手, "{理由}"),
            Self::由来を書けなかった { パス, 誤り } => write!(書き手, "{}へ由来を書けなかった: {誤り}", パス.display()),
            Self::再撮影が一致しない(形式) => write!(書き手, "同じ条件で再撮影した{形式}がバイト一致しない"),
            Self::構図契約が失敗した(理由) => write!(書き手, "遠景の固定構図が設計仕様を満たさない: {理由}"),
            Self::判定が失敗した(理由) => write!(書き手, "遠景のA/B判定が失敗した: {理由}"),
        }
    }
}

impl std::error::Error for 遠景構図の検収エラー {}
