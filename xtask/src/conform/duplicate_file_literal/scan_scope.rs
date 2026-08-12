//! この検査が見るファイルの範囲を所有する。受け取るのは無し、返すのは走査するファイルの一覧である。
//!
//! 試験のためのファイルを外すのは、試験の入力として作る名前が正本を持たないためである。
//! 自分で検査の対象を名指しする台帳は走査には含め、数え方の側で外す(`self_reference`)。走査から丸ごと落とすと、
//! そのファイルが本番のファイル名を持ったときにも見えなくなる。

use std::path::{Path, PathBuf};

use crate::conform::allow_lint::パスがテストまたは例か;
use crate::conform::error::規約検査の破れ;
use crate::file_scan;

const 走査するディレクトリ一覧: [&str; 2] = ["crates", "xtask/src"];
const 走査する拡張子一覧: [&str; 1] = ["rs"];

pub(super) fn 走査するファイル一覧を集める() -> Result<Vec<PathBuf>, 規約検査の破れ> {
    let 全ファイル = file_scan::対象ファイル一覧を集める(&走査するディレクトリ一覧, &走査する拡張子一覧)?;
    Ok(全ファイル.into_iter().filter(|パス| !試験のためのファイルか(パス)).collect())
}

/// ディレクトリと`_tests.rs`に加えて、モジュールの試験だけを収めた`tests.rs`も試験のためのファイルとみなす。
fn 試験のためのファイルか(パス: &Path) -> bool {
    パスがテストまたは例か(パス) || パス.file_name().is_some_and(|名前| 名前 == "tests.rs")
}
