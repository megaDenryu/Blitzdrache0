//! 行数検査: .rs / .slang の各ファイルが100行以内か確認する。

use std::path::Path;

use super::violation::違反;

pub const 上限行数: usize = 100;

pub fn 行数を数える(内容: &str) -> usize {
    内容.lines().count()
}

pub fn 行数超過か(行数: usize) -> bool {
    行数 > 上限行数
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let 行数 = 行数を数える(内容);
    if 行数超過か(行数) {
        vec![違反::ファイル単位(パス.to_path_buf(), format!("{行数}行"))]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 空文字列は0行() {
        assert_eq!(行数を数える(""), 0);
    }

    #[test]
    fn 末尾改行なしでも行数を数えられる() {
        assert_eq!(行数を数える("a\nb"), 2);
    }

    #[test]
    fn 上限を超えたら違反と判定する() {
        assert!(!行数超過か(上限行数));
        assert!(行数超過か(上限行数 + 1));
    }
}
