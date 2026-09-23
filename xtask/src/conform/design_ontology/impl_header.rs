//! `impl` の見出しから、型引数内の式ブロックと区別して本体の範囲を読む。

pub struct 実装の見出し {
    pub 表記: String,
    pub 本体の開始行: usize,
    pub 本体の終了行: usize,
}

#[derive(Default)]
struct 見出しの括弧 {
    山: usize,
    丸: usize,
    角: usize,
    式の波: usize,
    直前: Option<char>,
}

impl 見出しの括弧 {
    fn 本体の開きか(&mut self, 文字: char) -> bool {
        if self.式の波 > 0 {
            match 文字 {
                '{' => self.式の波 += 1,
                '}' => self.式の波 -= 1,
                _ => {}
            }
        } else {
            match 文字 {
                '<' => self.山 += 1,
                '>' if self.直前 != Some('-') => self.山 = self.山.saturating_sub(1),
                '(' => self.丸 += 1,
                ')' => self.丸 = self.丸.saturating_sub(1),
                '[' => self.角 += 1,
                ']' => self.角 = self.角.saturating_sub(1),
                '{' if self.山 == 0 && self.丸 == 0 && self.角 == 0 => return true,
                '{' => self.式の波 = 1,
                _ => {}
            }
        }
        self.直前 = Some(文字);
        false
    }
}

pub fn implの見出しを読む(行一覧: &[String], 開始: usize) -> Option<実装の見出し> {
    let 最初の行 = 行一覧.get(開始)?.trim_start();
    let 残り = 最初の行.strip_prefix("impl")?;
    if !残り.starts_with(char::is_whitespace) && !残り.starts_with('<') {
        return None;
    }
    let mut 括弧 = 見出しの括弧::default();
    let mut 表記 = String::new();
    for (行番号, 行) in 行一覧.iter().enumerate().skip(開始) {
        for (文字位置, 文字) in 行.char_indices() {
            表記.push(文字);
            if 括弧.本体の開きか(文字) {
                return Some(実装の見出し {
                    表記,
                    本体の開始行: 行番号,
                    本体の終了行: 本体が閉じる行(行一覧, 行番号, 文字位置)?,
                });
            }
        }
        表記.push(' ');
    }
    None
}

fn 本体が閉じる行(行一覧: &[String], 開始行: usize, 開きの位置: usize) -> Option<usize> {
    let mut 深さ = 0usize;
    for (行番号, 行) in 行一覧.iter().enumerate().skip(開始行) {
        let 読む部分 = if 行番号 == 開始行 { 行.get(開きの位置..)? } else { 行.as_str() };
        for 文字 in 読む部分.chars() {
            match 文字 {
                '{' => 深さ += 1,
                '}' => {
                    深さ = 深さ.checked_sub(1)?;
                    if 深さ == 0 {
                        return Some(行番号);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::implの見出しを読む;

    #[test]
    #[allow(clippy::expect_used)]
    fn 定数式が閉じた行からでも実装本体の終了までを読む() {
        let 行一覧 = ["impl<T> 規則<T> where T: 境界<{", "1 }> {", "fn 変える(&mut self) {}", "}"].map(str::to_string);
        let 見出し = implの見出しを読む(&行一覧, 0).expect("実装本体を読む");
        assert_eq!(見出し.本体の開始行, 1);
        assert_eq!(見出し.本体の終了行, 3);
    }
}
