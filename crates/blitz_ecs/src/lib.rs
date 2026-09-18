//! ゲーム世界の個体群の基盤。実行時個体IDと生存台帳・個体構成要素の標識となる`trait`と疎な集合の置き場・型を消した置き場の集まりを持つ個体群・
//! 問い合わせが返す5つの借りの型・構造変更の予約とセッション型の反映、から成る。
//!
//! このクレートが公開する型名・メソッド名・フィールド名はすべて日本語であり、設計方式の略称である ECS は
//! クレート名にだけ現れる。一般的なECSの抽象名をこのクレートの中の識別子として使うことは
//! `cargo xtask conform` が機械で禁じる。
//!
//! `#[cfg(test)]` の下にある素朴な置き場(`naive_baseline`)は、疎な集合の数字と並べる比較対象として測るためだけの実装であり、本番の置き場ではない。
//!
//! 注意: このクレートは thiserror 以外の何も知らない。具体ゲームの型も、物理・描画・アセットの型も知らない。
//! 依存の向きは `xtask/src/conform/dependency_whitelist/ledger.rs` の白リストが機械で強制する。
//!
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`。用語は `_doc/計画/ユビキタス言語.md`
//! 「ゲーム世界の個体と個体構成要素の語彙」に従う。

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod borrow;
mod component_count;
mod component_marker;
mod component_storage;
mod erased_storage;
mod liveness_error;
mod liveness_ledger;
#[cfg(test)]
mod liveness_ledger_tests;
mod liveness_slot;
mod liveness_state;
#[cfg(test)]
mod naive_baseline;
#[cfg(test)]
mod naive_baseline_measurement;
#[cfg(test)]
mod naive_baseline_samples;
#[cfg(test)]
mod naive_baseline_tests;
mod population;
mod runtime_entity_id;
#[cfg(test)]
mod sparse_set_measurement;
mod storage_collection;

pub use borrow::{一型を可変に借りる借り, 一型を読みもう一型を可変に借りる借り, 一型を読む借り, 二型を可変に借りる借り, 二型を読む借り};
pub use component_count::個体構成要素の型ごとの件数と占有量;
pub use component_marker::個体構成要素;
pub use component_storage::{個体構成要素の置き場, 個体構成要素の置き場エラー};
pub use liveness_error::生存台帳エラー;
pub use liveness_ledger::生存台帳;
pub use liveness_state::個体の生存状態;
pub use population::structural_change::{構造変更の予約, 構造変更の予約エラー, 構造変更の反映, 構造変更の反映の結果};
pub use population::{ゲーム世界の個体群, 実行時個体IDの発行口, 読みながら構造変更を予約する借り};
pub use runtime_entity_id::ゲーム世界の実行時個体ID;
