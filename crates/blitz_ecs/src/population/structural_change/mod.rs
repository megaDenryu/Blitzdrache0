//! 構造変更: 処理の途中の生成・破棄・個体構成要素の追加と削除を予約の列へ積み、反映の境界でセッション型が置き場へ適用する仕組み。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断7」。

mod apply;
mod command;
mod erased_value;
mod error;
#[cfg(test)]
mod removal_tests;
mod reservation;
mod result;
mod session;
#[cfg(test)]
mod tests;

pub use error::構造変更の予約エラー;
pub use reservation::構造変更の予約;
pub use result::構造変更の反映の結果;
pub use session::構造変更の反映;
