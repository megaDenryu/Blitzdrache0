//! ファイルのパスから Rust のモジュールパスを導く純粋な関数。受け取るのはパス、返すのは `クレート名::a::b` の綴りである。
//! `crates` の直下のクレート `名前` について、`src` の直下の `lib.rs`・`main.rs` は `名前`、`src` の下の `a/b.rs` は `名前::a::b`、`a/mod.rs` は `名前::a` である。
//! 試験のファイル(`tests.rs`・`*_tests.rs`)も同じ規則で自分のモジュールになる。
//! 保証範囲の外: 同じファイルの中の inline module(`mod a { ... }`)で同名の型を分けることと、`#[path = "..."] mod` によるファイルとモジュールの不一致は、この推定では区別できない。

use std::path::{Component, Path};

/// パスのモジュールパス。`crates` の下に無いパスは空である。
pub fn モジュールパス(パス: &Path) -> String {
    let mut 部品一覧 = パス.components().filter_map(|部品| match 部品 {
        Component::Normal(名前) => 名前.to_str(),
        _ => None,
    });
    部品一覧.find(|部品| *部品 == "crates");
    let Some(クレート) = 部品一覧.next() else {
        return String::new();
    };
    let mut 区切り一覧 = vec![クレート.to_string()];
    for 部品 in 部品一覧.skip_while(|部品| *部品 == "src") {
        let 名前 = 部品.strip_suffix(".rs").unwrap_or(部品);
        if !matches!(名前, "lib" | "main" | "mod") {
            区切り一覧.push(名前.to_string());
        }
    }
    区切り一覧.join("::")
}

/// モジュールパスの親。最上位(クレート)の親はそのクレート自身である。
pub fn 親のモジュールパス(モジュールパス: &str) -> String {
    モジュールパス.rsplit_once("::").map_or(モジュールパス.to_string(), |(親, _)| 親.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn モジュールパス_libとmodは区切りにならず入れ子は二重コロンで繋ぐ() {
        assert_eq!(モジュールパス(Path::new("crates/a/src/lib.rs")), "a");
        assert_eq!(モジュールパス(Path::new("crates/a/src/main.rs")), "a");
        assert_eq!(モジュールパス(Path::new("crates/a/src/x/y.rs")), "a::x::y");
        assert_eq!(モジュールパス(Path::new("crates/a/src/x/mod.rs")), "a::x");
        assert_eq!(モジュールパス(Path::new("C:/devs/repo/crates/a/src/tests/x_tests.rs")), "a::tests::x_tests");
        assert_eq!(モジュールパス(Path::new("xtask/src/x.rs")), "");
    }

    #[test]
    fn 親のモジュールパス_最上位の親は自分自身() {
        assert_eq!(親のモジュールパス("a::x::y"), "a::x");
        assert_eq!(親のモジュールパス("a"), "a");
    }
}
