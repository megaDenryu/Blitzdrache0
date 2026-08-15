//! 遠景の固定構図を採る工程ごとの失敗。
//!
//! 採取を読む段の破れ(器の破れ)と、絵から採った数へ課した判定の破れを別の枝で持つのは、直す側が見る場所が
//! 違うためである。前者はファイルを採り直せば直り、後者は絵に写った陰影そのものの退行である。
//! 1つの型へ束ねるのは、この入口が構築と採取と読み取りと判定を一続きに通すためである。
//!
//! 人が読む1文へ写すのは`display`が持つ。

mod artifact_read_break;
mod display;

use std::path::PathBuf;

pub(in crate::distant_view) use artifact_read_break::採取の読み取りの破れ;

use crate::acceptance::判定の破れ;

#[derive(Debug)]
pub(super) enum 遠景構図の検収エラー {
    引数が不正(String),
    構築が失敗した(crate::release_build::計測用の構築の破れ),
    描画検収が失敗した(crate::acceptance::検収エラー),
    由来を書けなかった {
        パス: PathBuf,
        誤り: std::io::Error,
    },
    再撮影が一致しない(&'static str),
    構図契約が失敗した(String),
    /// 散布の対照を焼く工程の破れ。絵を採る前の段であるため、採取の読み取りの破れとは別の枝で持つ。
    対照の焼き付けが失敗した(crate::game_fox_tour::error::場所巡りの通しの検収エラー),
    /// 採った絵と由来のファイルを読む段の破れ。9つの前提を`採取の読み取りの破れ`が数え上げる。
    採取を読めなかった(採取の読み取りの破れ),
    /// 絵から採った数へ課した判定の破れ。破れの形を`判定の破れ`が数え上げる。
    判定が破れた(判定の破れ),
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

impl From<採取の読み取りの破れ> for 遠景構図の検収エラー {
    fn from(破れ: 採取の読み取りの破れ) -> Self {
        Self::採取を読めなかった(破れ)
    }
}

/// 判定の破れを遠景の検収エラーへ載せる境界。判定の側は自分がこの入口の一部であることを知らずに破れだけを返す。
impl From<判定の破れ> for 遠景構図の検収エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}

impl std::error::Error for 遠景構図の検収エラー {}
