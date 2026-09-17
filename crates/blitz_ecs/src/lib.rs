//! ゲーム世界の個体群の基盤。いま持っているのは実行時個体IDと生存台帳であり、個体構成要素の置き場・問い合わせ・構造変更の予約と反映は Issue #49 が足す。
//!
//! このクレートが公開する型名・メソッド名・フィールド名はすべて日本語であり、設計方式の略称である ECS は
//! クレート名にだけ現れる。一般的なECSの抽象名をこのクレートの中の識別子として使うことは
//! `cargo xtask conform` が機械で禁じる。
//!
//! 注意: このクレートは thiserror 以外の何も知らない。具体ゲームの型も、物理・描画・アセットの型も知らない。
//! 依存の向きは `xtask/src/conform/dependency_whitelist/ledger.rs` の白リストが機械で強制する。
//!
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`。用語は `_doc/計画/ユビキタス言語.md`
//! 「ゲーム世界の個体と個体構成要素の語彙」に従う。

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod liveness_error;
mod liveness_ledger;
#[cfg(test)]
mod liveness_ledger_tests;
mod liveness_slot;
mod liveness_state;
mod runtime_entity_id;

pub use liveness_error::生存台帳エラー;
pub use liveness_ledger::生存台帳;
pub use liveness_state::個体の生存状態;
pub use runtime_entity_id::ゲーム世界の実行時個体ID;
