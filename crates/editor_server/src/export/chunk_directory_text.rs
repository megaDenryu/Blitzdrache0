//! チャンク目録ソースのテキストを組み立てる工程。書式(先頭行の形式宣言+1行4欄「x z アセットID 相対パス」)の
//! 正本は、blitz_asset_compilerの`チャンク目録ソースを読み込む`(読み手、`entry.rs`の`行から解析する`)が持つ。
//! 対応する公開の書き手はそのクレートに無い(書き手は`examples/generate_source_assets/directory_source.rs`に
//! あるが`pub(crate)`でexamplesの外から届かない)ため、この書き手をここへ置く。書式が読み手とずれないことは
//! `tests/source_asset_export_http.rs`で、書き出した本文を実際に`チャンク目録ソースを読み込む`へ通す往復試験として検査する。

/// チャンク目録ソースの`chunk_directory.txt`というファイル名。この綴りが2箇所目になる理由は
/// `xtask/src/conform/duplicate_file_literal/allowance/table.rs`に記した
/// (blitz_asset_compilerの正本定数は`pub(super)`で非公開であり、editor_serverから届かない)。
pub(super) const ファイル名: &str = "chunk_directory.txt";

pub(super) struct 目録項目 {
    pub(super) x: i32,
    pub(super) z: i32,
    pub(super) アセットid: String,
    pub(super) 相対ファイル名: String,
}

pub(super) fn 本文を組み立てる(チャンク一辺メートル: f32, 項目一覧: &[目録項目]) -> String {
    let mut 本文 = format!("blitz_chunk_directory 2 {チャンク一辺メートル}");
    本文.push('\n');
    for 項目 in 項目一覧 {
        本文.push_str(&format!("{} {} {} {}\n", 項目.x, 項目.z, 項目.アセットid, 項目.相対ファイル名));
    }
    本文
}
