//! 文字列リテラルの中身から、ファイル名らしいものを選ぶ工程。受け取るのはリテラルの中身、
//! 返すのはそれが拡張子を持つかどうかである。
//!
//! 拡張子とみなすのは、点に続く英字始まりの英数字が2文字以上10文字以下で、その後が英数字でない並びである。
//! 英字始まりに限るのは、`0.05`のような数の綴りを拡張子と読み違えないためである。
//! リテラルの切り出しは`source_lexing`の字句の走査が持つ。ここは切り出された中身だけを見る。

/// 綴りのうちファイル名の部分。区切りより前のディレクトリを落とす。
///
/// 同じファイルを指す綴りが、`shaders/scene.slang`と`scene.slang`、`/ui_vertex.spv`と`ui_vertex.spv`のように
/// 前置きの違いで別物になるのを防ぐ。片方が生成の出力名でもう片方が取り込みの名前という対がこの形を取り、
/// そこが本来いちばん食い違ってはならない場所である。
pub(super) fn ファイル名の部分(中身: &str) -> String {
    中身.rsplit(['/', '\\']).next().unwrap_or_default().to_string()
}

pub(super) fn 拡張子を含むか(中身: &str) -> bool {
    中身.match_indices('.').any(|(位置, _)| 拡張子が始まるか(中身, 位置))
}

fn 拡張子が始まるか(中身: &str, 点の位置: usize) -> bool {
    let 続き: String = 中身[点の位置 + 1..].chars().take_while(char::is_ascii_alphanumeric).collect();
    let 長さが拡張子の範囲 = (2..=10).contains(&続き.chars().count());
    let 英字始まり = 続き.chars().next().is_some_and(|文字| 文字.is_ascii_alphabetic());
    let 後ろが英数字でない = 中身[点の位置 + 1 + 続き.len()..]
        .chars()
        .next()
        .is_none_or(|文字| !文字.is_ascii_alphanumeric());
    長さが拡張子の範囲 && 英字始まり && 後ろが英数字でない
}
