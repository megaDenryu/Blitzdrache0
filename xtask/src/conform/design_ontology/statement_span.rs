//! `use` 文を書き出した行から、その文を終える `;` までを行をまたいで繋ぐ純粋な関数。受け取るのは書き出しの行の予約語より後ろと続きの行の一覧、返すのは `;` の手前までの本文である。
//! rustfmt は表示の幅(`max_width = 250`)を超える文を折る。日本語の識別子は表示の幅を2つ使うため、125文字でこの幅に届く。1行しか読まないと、折れた正当な文の取り込みを落とすため、`;` まで行を繋いでから読む。
//! `use` の木は `;` を持てないため、最初の `;` で文を終える。

/// 書き出しの残りと続きの行を空白で繋ぎ、最初の `;` の手前までを本文として返す。ファイルの最後まで `;` が現れなければ無い。
pub fn セミコロンまで繋いだ本文(書き出しの残り: &str, 続きの行一覧: &[String]) -> Option<String> {
    let mut 本文 = String::new();
    for 部分 in std::iter::once(書き出しの残り).chain(続きの行一覧.iter().map(String::as_str)) {
        if !本文.is_empty() {
            本文.push(' ');
        }
        match 部分.split_once(';') {
            Some((手前, _)) => {
                本文.push_str(手前);
                return Some(本文);
            }
            None => 本文.push_str(部分),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::セミコロンまで繋いだ本文;

    #[test]
    fn 折れた文を最初のセミコロンまで繋ぐ() {
        let 続き = vec!["    b, c};".to_string(), "use d;".to_string()];
        assert_eq!(セミコロンまで繋いだ本文(" a::{", &続き).as_deref(), Some(" a::{     b, c}"));
        assert_eq!(セミコロンまで繋いだ本文(" a::{b, c}; ", &[]).as_deref(), Some(" a::{b, c}"));
    }

    #[test]
    fn ファイルの最後までセミコロンが無ければ無い() {
        assert!(セミコロンまで繋いだ本文(" a::{b, c", &["    d".to_string()]).is_none());
    }
}
