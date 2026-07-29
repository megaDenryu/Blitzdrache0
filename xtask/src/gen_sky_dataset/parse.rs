//! C言語ヘッダから`double 名前[] = { ... };`の中身を数として取り出す工程。
//! 受け取るのはヘッダ全文と配列名と期待要素数、返すのはf32へ丸めた値の並びである。
//! 正規表現の依存を足さないため、括弧の位置探索と区切り文字での分解だけで読む。

pub(super) fn 注釈を落とす(内容: &str) -> String {
    let ブロック除去 = ブロック注釈を落とす(内容);
    ブロック除去
        .lines()
        .map(|行| 行.split_once("//").map_or(行, |(前, _)| 前))
        .collect::<Vec<_>>()
        .join("\n")
}

fn ブロック注釈を落とす(内容: &str) -> String {
    let mut 結果 = String::with_capacity(内容.len());
    let mut 残り = 内容;
    while let Some(開始) = 残り.find("/*") {
        結果.push_str(&残り[..開始]);
        let 開始後 = &残り[開始 + 2..];
        let Some(終了) = 開始後.find("*/") else { return 結果 };
        残り = &開始後[終了 + 2..];
    }
    結果.push_str(残り);
    結果
}

pub(super) fn 配列を取り出す(内容: &str, 名前: &str, 期待要素数: usize) -> Result<Vec<f32>, String> {
    let 宣言 = format!("double {名前}[]");
    let 宣言位置 = 内容.find(&宣言).ok_or_else(|| format!("{名前}の宣言が見つからない"))?;
    let 宣言後 = &内容[宣言位置 + 宣言.len()..];
    let 開き位置 = 宣言後.find('{').ok_or_else(|| format!("{名前}の開き波括弧が見つからない"))?;
    let 閉じ位置 = 宣言後.find('}').ok_or_else(|| format!("{名前}の閉じ波括弧が見つからない"))?;
    if 閉じ位置 < 開き位置 {
        return Err(format!("{名前}の波括弧の順序が逆である"));
    }
    let 中身 = &宣言後[開き位置 + 1..閉じ位置];
    let mut 値一覧 = Vec::with_capacity(期待要素数);
    for 語 in 中身.split(',') {
        let 整形 = 語.trim();
        if 整形.is_empty() {
            continue;
        }
        // 元データの表記はdoubleだが、評価も描画もf32で行うため十進表記から直接f32へ読む(f64を経由した二重丸めを避ける)。
        let 値: f32 = 整形.parse().map_err(|誤り| format!("{名前}の要素`{整形}`を数として読めない: {誤り}"))?;
        if !値.is_finite() {
            return Err(format!("{名前}の要素`{整形}`がf32で有限にならない"));
        }
        値一覧.push(値);
    }
    if 値一覧.len() != 期待要素数 {
        return Err(format!("{名前}の要素数が{}で、期待の{期待要素数}と違う", 値一覧.len()));
    }
    Ok(値一覧)
}
