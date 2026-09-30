//! 暫定の定義から最初の文を切り出す。文の終わりは句点であるが、括弧(丸括弧・全角丸括弧・鉤括弧)の中とコードの囲み(バッククォート)の中の句点は文を終えない。
//! 句点が無い定義は、定義の全体を1文とみなす。

#[derive(Clone, Copy)]
pub(super) struct 定義文の最初の文<'a>(&'a str);

impl<'a> 定義文の最初の文<'a> {
    pub(super) fn 定義から切り出す(定義: &'a str) -> Self {
        let mut 括弧の深さ = 0_usize;
        let mut コードの中か = false;
        for (位置, 文字) in 定義.char_indices() {
            match 文字 {
                '`' => コードの中か = !コードの中か,
                _ if コードの中か => {}
                '(' | '（' | '「' => 括弧の深さ += 1,
                ')' | '）' | '」' => 括弧の深さ = 括弧の深さ.saturating_sub(1),
                '。' if 括弧の深さ == 0 => return Self(&定義[..位置 + '。'.len_utf8()]),
                _ => {}
            }
        }
        Self(定義.trim())
    }

    pub(super) fn 文(&self) -> &'a str {
        self.0
    }
}
