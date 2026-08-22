//! ソースアセットの書き出し。担当するのは、保管庫が持つ大域世界とチャンクを
//! blitz_asset_compilerのソースアセット形式(高さ場+チャンク目録)へ変換して`assets/<世界名>/`へ
//! 書き出すことである。参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」

mod chunk_asset_file_names;
mod chunk_directory_text;
mod command;
mod destination;
mod editor_chunk_source;
mod error;
mod numeric_conversion;
mod one_building;
mod response;
mod result;
mod world_name;

pub use command::ソースアセット書き出しコマンド;
pub use destination::世界ソース出力先;
pub use error::書き出しエラー;
pub use one_building::一棟だけの検証世界を書き出すコマンド;
pub use result::書き出し結果;
pub use world_name::出力世界名;
