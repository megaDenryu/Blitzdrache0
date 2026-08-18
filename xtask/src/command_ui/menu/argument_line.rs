//! メニューの引数入力行の解釈。担当するのは、1行のテキストを引用符の外の空白で区切って
//! 語の列へ分けることだけである。二重引用符`"..."`で囲まれた範囲は空白を含んでいても同じ語の
//! 中身として保ち、語の途中に現れる引用符(`--path="a b"`のような形)も引用符の内側だけ空白を
//! 保護して1語へ繋げる。閉じられていない引用符は黙って握り潰さず、型付きの破れとして返す。

use std::fmt;

#[derive(Debug)]
pub(super) enum 引数の行の破れ {
    引用符が閉じていない,
}

impl fmt::Display for 引数の行の破れ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::引用符が閉じていない => write!(f, "引用符が閉じていない"),
        }
    }
}

impl std::error::Error for 引数の行の破れ {}

/// 1行を引用符の外の空白で区切り、語の列へ分ける。空の引用符`""`は空の語になる。
pub(super) fn 語へ分ける(行: &str) -> Result<Vec<String>, 引数の行の破れ> {
    let mut 語一覧 = Vec::new();
    let mut 現在の語: Option<String> = None;
    let mut 引用符の中か = false;
    for 文字 in 行.chars() {
        match 文字 {
            '"' => {
                引用符の中か = !引用符の中か;
                現在の語.get_or_insert_with(String::new);
            }
            文字 if 文字.is_whitespace() && !引用符の中か => {
                if let Some(語) = 現在の語.take() {
                    語一覧.push(語);
                }
            }
            文字 => {
                現在の語.get_or_insert_with(String::new).push(文字);
            }
        }
    }
    if 引用符の中か {
        return Err(引数の行の破れ::引用符が閉じていない);
    }
    if let Some(語) = 現在の語 {
        語一覧.push(語);
    }
    Ok(語一覧)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 空白入りの引用符区間は1語になる() {
        assert_eq!(語へ分ける(r#"a "b c" d"#).unwrap(), vec!["a", "b c", "d"]);
    }

    #[test]
    fn 語の途中の引用符は空白を保護したまま1語へ繋がる() {
        assert_eq!(語へ分ける(r#"--path="C:\a b""#).unwrap(), vec![r"--path=C:\a b"]);
    }

    #[test]
    fn 閉じ忘れた引用符は破れになる() {
        assert!(matches!(語へ分ける(r#""C:\foo"#), Err(引数の行の破れ::引用符が閉じていない)));
    }

    #[test]
    fn 空行は空の語一覧になる() {
        assert_eq!(語へ分ける("\r\n").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn 引用符ありと無しの引数が混在できる() {
        assert_eq!(語へ分ける(r#"--flag "a b" --other=1"#).unwrap(), vec!["--flag", "a b", "--other=1"]);
    }

    #[test]
    fn 空の引用符は空の語になる() {
        assert_eq!(語へ分ける(r#"a "" b"#).unwrap(), vec!["a", "", "b"]);
    }
}
