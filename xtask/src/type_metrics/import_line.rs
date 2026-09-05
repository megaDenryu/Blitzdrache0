//! ファイルの本文から`use`宣言を1件ずつ取り出す。折り返して複数行に書かれた宣言は、終端の記号までを繋いで1件にする。
//!
//! 取り出すのは宣言の綴りだけであり、波括弧の展開は`import_tree`が行う。

const 宣言の始まり: &str = "use ";
const 宣言の終わり: char = ';';

pub fn 取り込みの宣言一覧(内容: &str) -> Vec<String> {
    let mut 宣言一覧 = Vec::new();
    let mut 組み立て中: Option<String> = None;
    for 行 in 内容.lines() {
        let 整形 = 行.trim();
        let 続き = match 組み立て中.take() {
            Some(途中) => format!("{途中} {整形}"),
            None => match 宣言の本体を始める(整形) {
                Some(本体) => 本体,
                None => continue,
            },
        };
        match 続き.split_once(宣言の終わり) {
            Some((宣言, _)) => 宣言一覧.push(宣言.trim().to_string()),
            None => 組み立て中 = Some(続き),
        }
    }
    宣言一覧
}

/// `use`から始まる行だけを宣言の始まりとみなし、可視性の修飾子は取り除く。行の途中に現れる`use`は対象にしない。
fn 宣言の本体を始める(整形された行: &str) -> Option<String> {
    let 修飾子なし = super::keyword::修飾子を取り除く(整形された行);
    修飾子なし.strip_prefix(宣言の始まり).map(|本体| 本体.trim().to_string())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 折り返さない宣言を取り出す() {
        assert_eq!(
            取り込みの宣言一覧("use super::sweep_solver::求解;\n"),
            vec!["super::sweep_solver::求解".to_string()]
        );
    }

    #[test]
    fn 折り返した宣言を1件へ繋ぐ() {
        let 内容 = "use super::{\n    建物外形カタログ, 建物外形定義\n};\n";
        assert_eq!(取り込みの宣言一覧(内容), vec!["super::{ 建物外形カタログ, 建物外形定義 }".to_string()]);
    }

    #[test]
    fn 可視性の修飾子を取り除いて取り出す() {
        assert_eq!(
            取り込みの宣言一覧("pub(crate) use crate::far::設定;\n"),
            vec!["crate::far::設定".to_string()]
        );
    }

    #[test]
    fn useで始まらない行は宣言にしない() {
        assert!(取り込みの宣言一覧("let 使用 = user;\nfn 使う() {}\n").is_empty());
    }
}
