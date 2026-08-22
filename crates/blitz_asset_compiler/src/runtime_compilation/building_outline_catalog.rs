//! 建物定義の正本から、実際に展開した部品の外形を持つ版付きカタログを作る。
//! エディターはこの出力だけを読み、建物の寸法を画面側で推測しない。
//!
//! 建物の由来は2系統である。宣言の規則5件と、建物エディターが編集したベイ格子である。どちらも同じ展開器で
//! 展開し、外接箱・ベイ構造・使う部品の一覧はすべて展開結果から導く。由来ごとの違いは`assembly_origin`が持つ。

mod assembly_origin;
mod building_definition_id;
mod catalog;
mod catalog_builder;
mod catalog_file;
mod catalog_json;
mod definition_material;
mod definition_source;
mod error;
mod outline_builder;
mod replacement_candidate;

pub use building_definition_id::建物定義ID;
pub use catalog::{ベイ構造, 建物の入口方向, 建物の外接箱, 建物外形カタログ, 建物定義, 建物定義の用途};
pub use catalog_builder::建物外形カタログの組み立て係;
pub use catalog_file::建物外形カタログのファイル;
pub(crate) use definition_source::{全建物の部品識別一覧, 建物定義の正本, 識別子から建物定義を参照する};
pub use error::建物外形カタログエラー;
pub use replacement_candidate::差し替えの候補;

/// 外部アセットの置き場が無い環境でも編集サーバーを起動できるようにするための、定義0件の版付きカタログ。
/// 「定義が読めなかった」ことは呼び出し側が診断として印字し、無言の代替にはしない。
pub fn 空の建物外形カタログを作る() -> 建物外形カタログ {
    建物外形カタログ::定義一覧から生成する(Vec::new())
}

/// 建物定義の正本が持つ識別子の一覧。外部アセットの置き場に依らず答えられるため、
/// 別クレートが書いた移行先の建物定義IDが正本に実在するかの照合に使える。
pub fn 建物定義の識別子一覧() -> Vec<&'static str> {
    definition_source::全定義().iter().map(|正本| 正本.識別子).collect()
}
