//! 遷移モデルと有限全数の検証(Issue #173。層6と代表ケース)。
//!
//! 静的な設計関係グラフだけでは「ある条件の状態から遷移すると、あるイベントが成立する」という時間の向きの仕様を表せない。
//! この木は、前状態・入力・適用した規則・後状態・生成した出来事・失敗を持つ遷移モデルと、そのモデルの状態と遷移を
//! 全部列挙して振る舞いの命題を検証する検証器と、その検証器が命題を解けることを示す題材を持つ。
//!
//! **有限の対象を全部列挙できたときだけ証明済みとする。** 列挙し切れなかったときは未決定で答える。
//! 参照: Issue #173「時間と遷移」「Layer 4: Verification」。

mod exhaustive_verifier;
mod representative_case;
#[cfg(test)]
mod representative_case_tests;
mod transition_model;

pub use exhaustive_verifier::有限全数の検証器;
pub use representative_case::対戦進行の題材;
pub use transition_model::{遷移, 遷移の材料, 遷移の識別子, 遷移モデル};
