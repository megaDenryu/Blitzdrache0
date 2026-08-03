//! `--indirect-probe <条件>`の解析。担当する工程は「1つの語を遠方環境の検収条件へ直す」ことであり、
//! 受け取るのは引数イテレータ、返すのは条件である。
//!
//! 知らない語を既定の条件へ落とさず失敗にするのは、綴りの誤りが「別の条件で通った検収」になるためである。

use std::slice::Iter;

use super::value_args::次の値を読む;
use super::起動引数エラー;
use crate::reports::indirect_probe::遠方環境の検収条件;

pub(super) fn 引数を処理する(引数: &mut Iter<String>) -> Result<遠方環境の検収条件, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--indirect-probe", 起動引数エラー::遠方環境の検収条件不正)?;
    遠方環境の検収条件::語から選ぶ(値.as_str()).ok_or_else(|| {
        起動引数エラー::遠方環境の検収条件不正(format!("diffuse・specular-level・specular-faceのどれでもない: {値}"))
    })
}
