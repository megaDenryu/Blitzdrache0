//! ドメイン命題とその検証と証拠(Issue #173。層3から層5)。
//!
//! 設計関係はモデルの中の関係であり、命題はその関係について真偽を問うものである。この2つを層として分ける。
//! 命題は原子・否定・連言・選言・含意・全称・存在で組み立て、検証はその命題を解く手立て(検証器)と分離する。
//! 検証の結末は `bool` でなく `検証結果`(証明済み・反証済み・未決定)で答える。「反例の探索で反例が出なかった」を
//! 「証明済み」と呼ばないことをこの型が止める。
//! 参照: `_doc/設計/設計オントロジー.md` 第7節から第9節。

mod atom;
mod evidence;
mod law;
#[cfg(test)]
mod law_tests;
mod proposition;
mod quantification;
mod specification;
mod verifier;

pub use atom::{原子命題, 項};
pub use evidence::{反例, 未決定の理由, 検証の方式, 検証結果, 証拠};
pub use law::{排他の推論規則, 推論規則, 普遍命題としての法則};
pub use proposition::命題;
pub use quantification::{対象集合, 束縛の割り当て, 束縛変数};
pub use specification::{名前付きの命題, 命題の集合, 検証の集計};
pub use verifier::{列挙した対象, 原子と対象集合の解き手, 命題の検証器, 構造の検証器, 結合を解く工程};
