//! 借りの型: 個体群への問い合わせが返す、1つまたは2つの置き場への参照を内部に持ち、実行時個体IDの昇順の反復と
//! 個別の読み書きをメソッドで提供する型の群。初版は5つであり、任意長のタプルへ広げるマクロは作らない。
//! 2型の走査は、両方の置き場の密な列がIDの昇順である(契約25)ことに依る合流走査で行う。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断6」。

mod merge_walk;
mod read_one;
mod read_two;
mod read_write;
mod write_one;
mod write_two;

pub use read_one::一型を読む借り;
pub use read_two::二型を読む借り;
pub use read_write::一型を読みもう一型を可変に借りる借り;
pub use write_one::一型を可変に借りる借り;
pub use write_two::二型を可変に借りる借り;
