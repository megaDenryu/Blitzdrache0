//! 不正allow検査: unwrap_used等のlint緩和を、テスト以外で使っていないか確認する。
//! 注意: 検出パターンの綴りをこのファイルに連続して書くと、conformが自分自身を
//! 違反として検出してしまうため、分割リテラルの連結で回避する。

use std::path::{Component, Path};

use super::violation::違反;

const 検出パターン一覧: [&str; 3] = [
    concat!("allow(clippy::unwrap_us", "ed"),
    concat!("allow(clippy::expect_us", "ed"),
    concat!("allow(clippy::as_conversio", "ns"),
];

pub fn 不正allowを含むか(行: &str) -> Option<&'static str> {
    検出パターン一覧.iter().find(|パターン| 行.contains(*パターン)).copied()
}

pub fn パスがテストまたは例か(パス: &Path) -> bool {
    let ディレクトリで許容 = パス
        .components()
        .any(|部品| matches!(部品, Component::Normal(名前) if 名前 == "tests" || 名前 == "examples"));
    let ファイル名で許容 = パス.file_name().and_then(|名前| 名前.to_str()).is_some_and(|名前| 名前.ends_with("_tests.rs"));
    ディレクトリで許容 || ファイル名で許容
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let パス許容 = パスがテストまたは例か(パス);
    let mut cfg_testを見た = false;
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        let 許容されるか = パス許容 || cfg_testを見た;
        if let Some(パターン) = 不正allowを含むか(行).filter(|_| !許容されるか) {
            違反一覧.push(違反::行単位(パス.to_path_buf(), 行番号 + 1, format!("不正なallow({パターン})")));
        }
        if 行.contains("cfg(test)") {
            cfg_testを見た = true;
        }
    }
    違反一覧
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 前にcfg_testがあれば許容する() {
        let 内容 = "#[cfg(test)]\n#[allow(clippy::unwrap_used)]\nfn f() {}";
        assert!(検査する(Path::new("src/x.rs"), 内容).is_empty());
    }

    #[test]
    fn cfg_testが無ければ違反() {
        let 内容 = "#![allow(clippy::unwrap_used)]";
        assert_eq!(検査する(Path::new("src/x.rs"), 内容).len(), 1);
    }

    #[test]
    fn testsディレクトリなら許容する() {
        let 内容 = "#![allow(clippy::expect_used)]";
        assert!(検査する(Path::new("crates/foo/tests/x.rs"), 内容).is_empty());
    }

    #[test]
    fn _testsで終わるファイル名なら許容する() {
        let 内容 = "#![allow(clippy::expect_used)]";
        assert!(検査する(Path::new("crates/foo/barrier_derivation/barrier_derivation_tests.rs"), 内容).is_empty());
    }
}
