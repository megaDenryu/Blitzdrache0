//! ベイ格子の層。担当するのは、建物エディターが編集する升目の複体の語彙と、その格子を組み立て手順へ写す変換である。
//!
//! 升目1つが一間四方の骨格1件にあたり、升目の4つの側面が壁のはめ口に、升目の床が床のはめ口に、
//! 真上に升目の無い升目の上面が屋根の受け口にあたる。**格子は升目の間隔も階高も持たない。**
//! どこへ据わるかは接合点の宣言だけが決めるためであり、画面で見る縮尺はエディターの責務である。
//!
//! この束が知ってよいのは`joint`と`part`と`expansion`の組み立て手順だけである。ファイルも直列化も
//! 画面も知らない。変換は接合点の宣言を1つも読まない純粋計算であり、同じ格子と同じ綴りからは常に同じ手順が出る。
//!
//! 組み立て規則(`expansion`の`一間四方の骨格の規則`)と分けるのは、決め方が違うためである。規則は候補を並べて
//! 種から引き、連なりしか表せない。格子は人が1つずつ選んだ結果を持ち、枝分かれする任意の木を表せる。
//! 参照: `_doc/計画/エディターからゲームまでの統合の作戦.md`

mod cell;
mod cell_coordinate;
mod cell_horizontal_face;
mod cell_table;
mod error;
mod grid;
mod grid_builder;
mod grid_validation;
mod joint_spelling;
mod opening_fill;
mod ornament;
mod part_role;
mod part_spelling;
mod part_spelling_builder;
mod recipe_conversion;
mod side_face;
mod spanning_tree;
mod tree_link;
mod tree_node;
mod wall_step;

/// 骨格方式の試験(`crates/blitz_assembly/src/expansion/frame_tests.rs`)が、格子から作った手順と規則から作った
/// 手順の配置表を突き合わせるために材料を借りる。展開の層のカタログがそちらにあるためである。
#[cfg(test)]
pub(crate) mod grid_fixture;
#[cfg(test)]
mod grid_recipe_tests;
#[cfg(test)]
mod grid_rejection_tests;
#[cfg(test)]
mod ornament_recipe_tests;
#[cfg(test)]
mod value_rejection_tests;

pub use cell::升目の宣言;
pub use cell_coordinate::升目の座標;
pub use cell_horizontal_face::{升目の屋根, 升目の床};
pub use error::ベイ格子エラー;
pub use grid::ベイ格子;
pub use grid_builder::ベイ格子の組み手;
pub use opening_fill::{はめ口の値, 壁の種類};
pub use ornament::{壁の外面へ付ける飾り, 煙突の段数};
pub use part_role::骨格方式の部品の役割;
pub use part_spelling::骨格方式の部品の綴り;
pub use part_spelling_builder::骨格方式の部品の綴りの組み手;
pub use side_face::升目の側面;
