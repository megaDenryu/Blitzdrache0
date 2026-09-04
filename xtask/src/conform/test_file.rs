//! 試験と例のためのファイルかの判定。受け取るのはパス、返すのは試験と例のためのファイルかどうかである。
//!
//! 2つの検査(ファイル名らしい綴りの重複・親の型を丸ごと受け取る自由関数)が同じ範囲を外すため、
//! 判定をここ1箇所に置く。試験の中で作る名前と補助の関数は、本番の正本も本番の構造も持たない。

use std::path::Path;

use super::allow_lint::パスがテストまたは例か;

/// ディレクトリと`_tests.rs`に加えて、モジュールの試験だけを収めた`tests.rs`も試験のためのファイルとみなす。
pub fn 試験のためのファイルか(パス: &Path) -> bool {
    パスがテストまたは例か(パス) || パス.file_name().is_some_and(|名前| 名前 == "tests.rs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 試験のためのファイルを見分ける() {
        assert!(試験のためのファイルか(
            Path::new("xtask/src/conform/free_function_whole_type/tests.rs")
        ));
        assert!(試験のためのファイルか(
            Path::new("crates/blitz_render/src/renderer/origin_tests.rs")
        ));
        assert!(試験のためのファイルか(Path::new("crates/editor_server/tests/common/mod.rs")));
        assert!(!試験のためのファイルか(Path::new("crates/blitz_app/src/app/mod.rs")));
    }
}
