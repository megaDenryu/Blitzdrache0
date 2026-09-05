//! 接触の履歴(判断16)。参加者の組ごとに、接触1点の静止摩擦の錨と前の細分の接触法線を鍵の昇順の一覧で保ち、
//! 細分ごとに新しい接触点集合の鍵と併走して継続と終了と開始へ分ける。乗数は引き継がない(暖機はしない)。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断16: 接触の履歴は参加者の組ごとに特徴の識別を鍵に全順序の併走で対応付け、暖機はしない」

mod body_body_key;
mod body_static_key;
mod contact_history;
mod continuation_record;
mod correspondence;
#[cfg(test)]
mod correspondence_tests;
mod entry;
mod error;
mod kinds;
#[cfg(test)]
mod residual_discard_tests;

pub use body_body_key::剛体どうしの接触の鍵;
pub use body_static_key::剛体と静的世界の接触の鍵;
pub use contact_history::接触の履歴;
pub use continuation_record::接触の継続の記録;
pub use correspondence::{接触の併走の結果, 接触の対応付け};
pub use entry::接触の履歴の項目;
pub use error::接触の履歴エラー;
pub use kinds::{
    剛体と静的世界の接触の履歴, 剛体と静的世界の接触の履歴の項目, 剛体と静的世界の接触の継続の記録, 剛体どうしの接触の履歴,
    剛体どうしの接触の履歴の項目, 剛体どうしの接触の継続の記録,
};
