//! 自由関数の引数の並びを1つずつへ分け、参照で丸ごと受け取っている型の名前を取り出す工程。
//! 受け取るのは引数の並びの綴り、返すのは型の名前である。
//!
//! 取り出すのは`&型`・`&mut 型`・`&'寿命 型`の形だけである。スライス`&[要素]`・タプル`&(左, 右)`・
//! トレイトの対象`&dyn 役`・入れ物`&Vec<要素>`は、丸ごと受け取る形と読み分けられないため取り出さない。
//! 入れ物は外側の名前だけを返すため、その名前がクレートの型と一致しない限り検出に至らない。

pub fn 引数へ分ける(綴り: &str) -> Vec<String> {
    let mut 引数一覧 = Vec::new();
    let mut 現在 = String::new();
    let mut 深さ = 0usize;
    let mut 直前 = ' ';
    for 文字 in 綴り.chars() {
        match 文字 {
            '<' | '(' | '[' => 深さ += 1,
            '>' if 直前 != '-' => 深さ = 深さ.saturating_sub(1),
            ')' | ']' => 深さ = 深さ.saturating_sub(1),
            ',' if 深さ == 0 => {
                引数一覧.push(std::mem::take(&mut 現在));
                直前 = 文字;
                continue;
            }
            _ => {}
        }
        現在.push(文字);
        直前 = 文字;
    }
    引数一覧.push(現在);
    引数一覧
        .into_iter()
        .map(|引数| 引数.trim().to_string())
        .filter(|引数| !引数.is_empty())
        .collect()
}

pub fn 丸ごと受け取る型の名前(引数: &str) -> Option<String> {
    let 位置 = 引数.find(':')?;
    let 参照の後ろ = 引数.get(位置 + 1..)?.trim().strip_prefix('&')?.trim_start();
    let 寿命の後ろ = 寿命注釈を飛ばす(参照の後ろ);
    let 可変の後ろ = 寿命の後ろ.strip_prefix("mut ").unwrap_or(寿命の後ろ).trim_start();
    経路の末尾の型名(可変の後ろ)
}

fn 寿命注釈を飛ばす(綴り: &str) -> &str {
    let Some(注釈) = 綴り.strip_prefix('\'') else {
        return 綴り;
    };
    注釈.find(char::is_whitespace).map_or(綴り, |位置| 注釈[位置..].trim_start())
}

/// 経路の末尾の区切りだけを型名として読む。名前の後ろに山括弧以外が続く綴りは、丸ごと受け取る形でないため落とす。
fn 経路の末尾の型名(綴り: &str) -> Option<String> {
    let 先頭: String = 綴り
        .chars()
        .take_while(|文字| 文字.is_alphanumeric() || *文字 == '_' || *文字 == ':')
        .collect();
    let 残り = 綴り.get(先頭.len()..)?.trim();
    if !残り.is_empty() && !残り.starts_with('<') {
        return None;
    }
    let 名前 = 先頭.rsplit("::").next()?;
    (!名前.is_empty()).then(|| 名前.to_string())
}
