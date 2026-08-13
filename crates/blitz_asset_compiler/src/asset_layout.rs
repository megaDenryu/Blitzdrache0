//! アセットの置き場: ソースと実行時形式がどのディレクトリのどの名前で並ぶかを表す型の集まり。
//! ソースを書き出す側と実行時形式へ焼く側の両方がここを読むため、置き場の決め事はこの1箇所にだけ在る。
//!
//! 役割ごとに型を分けるのは、ソースの置き場と生成物の置き場が同じ`Path`では取り違えても型が通るためである。
//! 直下のパスを組む汎用の口をモジュールの外へ出さないのは、任意のファイル名が流れると綴りの正本が意味を失うためである。
//! 世界ごとの綴りは、その世界の置き場の型が持って役割別のメソッドで見せる。
//! 世界のディレクトリ名やファイル名の綴りを型の中へ閉じるのは、綴りの写しが2つのバイナリへ散ると、
//! 食い違ったときに「ファイルが見つからない」という遠い場所の失敗になるためである。写しを注意書きで同期させない。
//! 参照: `_doc/設計/大規模世界の生成と遠景.md`

mod chunk_height_grid;
#[cfg(test)]
mod cleanup_tests;
mod error;
mod fox_tour_source_directory;
mod height_grid_file;
mod runtime_output_root;
mod source_root;
mod world_directory_name;
mod world_source_directory;

pub use chunk_height_grid::{チャンクの高さ格子, チャンクの高さ格子ソース};
pub use error::アセット配置エラー;
pub use fox_tour_source_directory::場所巡りの世界のソースディレクトリ;
pub use height_grid_file::高さ格子のファイル;
pub use runtime_output_root::実行時形式の出力ルート;
pub use source_root::{ソースルート, チャンク目録ソースの置き場};
pub use world_directory_name::世界のディレクトリ名;
