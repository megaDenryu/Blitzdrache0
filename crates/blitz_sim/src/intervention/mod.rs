//! 介入(intervention。判断53): プレイヤーイベントをシミュレーションへ流し込む型付きの値。
//! 参照: `_doc/設計/シミュレーション層.md`「介入モデル」。

mod encode;
#[cfg(test)]
mod encode_tests;
mod kind;

pub use encode::バイト列にする;
pub use kind::介入;
