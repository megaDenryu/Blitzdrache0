//! 機能一覧の綴りの読み取り。受け取るのは依存の宣言の右辺の綴り、返すのは明示された機能の名前を宣言の順に並べた一覧である。
//!
//! 分けているのは、`default-features = false` の綴りが `features = ` を部分文字列として含み、素朴に探すと
//! 既定機能の宣言を機能一覧と読み違えるためである。この読み違えは検査を素通りさせるだけで何の症状も出さない。
//! 何が違反かを持つ側から、綴りの解き方を持つ側を分ける。

/// 機能一覧の宣言の始まり。角括弧まで含めるのは、`default-features = false` と綴りを分けるためである。
const 機能一覧の前置き: &str = "features = [";

pub(super) fn 機能一覧を読む(右辺: &str) -> Vec<String> {
    let Some(開始) = 機能一覧の開始位置を探す(右辺) else {
        return Vec::new();
    };
    let 残り = &右辺[開始 + 機能一覧の前置き.len()..];
    let Some(閉じ括弧) = 残り.find(']') else {
        return Vec::new();
    };
    残り[..閉じ括弧]
        .split(',')
        .map(|語| 語.trim().trim_matches('"').to_string())
        .filter(|名前| !名前.is_empty())
        .collect()
}

/// `default-features = [` のように別のキーの一部として現れた前置きを飛ばす。
/// キーの区切りとして認めるのは行頭・空白・波括弧・読点だけであり、ハイフンや英数字が前に来たら別のキーである。
fn 機能一覧の開始位置を探す(右辺: &str) -> Option<usize> {
    let mut 探索開始 = 0;
    while let Some(相対位置) = 右辺[探索開始..].find(機能一覧の前置き) {
        let 位置 = 探索開始 + 相対位置;
        let 直前 = 右辺[..位置].chars().next_back();
        match 直前 {
            None | Some(' ') | Some('\t') | Some('{') | Some(',') => return Some(位置),
            Some(_) => 探索開始 = 位置 + 機能一覧の前置き.len(),
        }
    }
    None
}
