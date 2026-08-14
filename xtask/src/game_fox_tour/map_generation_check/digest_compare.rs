//! 2つの生成ルートから得た相対パスと内容ハッシュを突き合わせる。

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::super::error::場所巡りの通しの検収エラー;
use crate::acceptance::判定の名前;

pub(super) fn 畳んだ値を突き合わせる(
    一度目: &BTreeMap<PathBuf, u64>,
    二度目: &BTreeMap<PathBuf, u64>,
    何の突き合わせか: &str,
) -> Result<usize, 場所巡りの通しの検収エラー> {
    判定名を組む(&format!("{何の突き合わせか}の2つの生成のファイルの本数")).一致を課す(二度目.len(), 一度目.len())?;
    for (相対パス, 一度目の値) in 一度目 {
        let 場所 = format!("{何の突き合わせか}の2つ目の生成の{}", 相対パス.display());
        let 二度目の値 = 二度目.get(相対パス).ok_or_else(|| 判定名を組む(&場所).あるはずのものが無い破れ())?;
        判定名を組む(&format!("{場所}の中身の畳んだ値")).一致を課す(*二度目の値, *一度目の値)?;
    }
    Ok(一度目.len())
}

fn 判定名を組む(綴り: &str) -> 判定の名前 {
    判定の名前::組み立てた綴りから生成する(綴り.to_string())
}
