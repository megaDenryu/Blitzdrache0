//! `--report-sky-pixel <横,縦;横,縦...>`の解析。担当する工程は「1つの語を画面画素位置の一覧へ直す」ことであり、
//! 受け取るのは引数イテレータ、返すのは検査したい画素の一覧である。
//!
//! 座標を呼び出し側から受け取るのは、どの画素が空を写しているかを決める材料(構図と地形の高さ)を持つのが
//! 検収側の`cargo xtask sky-time`であり、アプリはその画素について答えるだけでよいからである。

use std::slice::Iter;

use super::value_args::次の値を読む;
use super::起動引数エラー;

/// 読み戻し画像の中の1画素の位置。左上を原点に、横は右へ、縦は下へ数える。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 画面画素位置 {
    pub(crate) 横: u32,
    pub(crate) 縦: u32,
}

pub(super) fn report_sky_pixel引数を処理する(引数: &mut Iter<String>) -> Result<Vec<画面画素位置>, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--report-sky-pixel", 起動引数エラー::空代表画素不正)?;
    let mut 一覧 = Vec::new();
    for 組 in 値.split(';').filter(|語| !語.is_empty()) {
        一覧.push(一組を読む(組)?);
    }
    if 一覧.is_empty() {
        return Err(起動引数エラー::空代表画素不正(format!("画素が1つも指定されていない: {値}")));
    }
    Ok(一覧)
}

fn 一組を読む(組: &str) -> Result<画面画素位置, 起動引数エラー> {
    let 誤り = || 起動引数エラー::空代表画素不正(format!("横,縦の形でない: {組}"));
    let (横, 縦) = 組.split_once(',').ok_or_else(誤り)?;
    Ok(画面画素位置 {
        横: 横.trim().parse::<u32>().map_err(|_| 誤り())?,
        縦: 縦.trim().parse::<u32>().map_err(|_| 誤り())?,
    })
}
