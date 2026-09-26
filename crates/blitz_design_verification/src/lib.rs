//! 設計の命題と検証の共通語彙の正本(Issue #174 のオーナー裁定の案B)。設計概念・設計関係・分類・設計関係グラフ・命題・検証器・証拠・遷移モデル・
//! 有限全数の検証と、ドメインの側が実装する数え上げの契約を持つ。プリミティブな事実から設計関係と保持する関係を導く定義の式も持つ(Issue #193)。
//!
//! **このクレートはドメインのクレートも `xtask` も知らない。** 通常依存は `blitz_design` だけである。Rustのソースを読んで
//! プリミティブな事実を出す抽出器は `xtask` に残り、実物の遷移関数を呼んで振る舞いの命題を検証するのはドメイン側の試験である。
//! どちらもこのクレートを使う側であり、このクレートはどちらの語彙も持たない。
//!
//! **構造の検証と振る舞いの検証を2つのクレートへ分けない。** 分けると、構造の検証器が語る `命題` と振る舞いの検証器が語る
//! `命題` が別の型になり、構造の原子命題(設計関係が在る)と振る舞いの原子命題(発生する)を1つの連言へ結べない。
//! 命題の型を1つに保つことがこのクレートを1つに保つ理由である。
//! 参照: `_doc/設計/設計オントロジー.md` 第6節から第10節。

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod atom;
mod concept;
#[cfg(test)]
mod concept_tests;
mod coverage;
mod domain;
mod evidence;
mod exhaustive;
mod extraction_loss;
mod law;
#[cfg(test)]
mod law_tests;
mod proposition;
mod quantification;
mod relation;
mod relation_graph;
#[cfg(test)]
mod relation_graph_tests;
mod specification;
mod transition_model;
mod verifier;
#[path = "分類.rs"]
mod 分類;
#[path = "導出.rs"]
mod 導出;

pub use atom::{原子命題, 項};
pub use concept::{Rustの項目の種類, 設計概念, 設計概念の名前空間, 設計概念の種類, 設計概念の識別子, 設計概念への参照};
pub use coverage::数え上げの網羅性;
pub use domain::{探索が組んだ遷移モデルと網羅性, 探索の上限, 数え上げ, 有限に数え上げられる領域, 遷移の帰結, 遷移失敗結果の元の状態の食い違い, 領域から遷移モデルを組む};
pub use evidence::{反例, 未決定の理由, 検証の方式, 検証結果, 証拠};
pub use exhaustive::有限全数の検証器;
pub use extraction_loss::{落とした理由ごとの件数, 関係を落とした抽出の欠落};
pub use law::{排他の推論規則, 推論規則, 普遍命題としての法則};
pub use proposition::{命題, 非空の命題一覧, 非空の命題一覧の生成の失敗};
pub use quantification::{対象集合, 束縛の割り当て, 束縛変数};
pub use relation::{保持する関係, 導出の根拠, 抽出の出どころ, 抽出元の構文, 設計関係, 設計関係の種類};
pub use relation_graph::{同一性の衝突, 組み立てた設計関係グラフと同一性の衝突, 設計関係グラフ};
pub use specification::{名前付きの命題, 命題の集合, 検証の集計};
pub use transition_model::{到達可能な状態一覧, 遷移, 遷移の材料, 遷移の識別子, 遷移モデル};
pub use verifier::{列挙した対象, 原子と対象集合の解き手, 命題の検証器, 構造の検証器, 結合を解く工程};
pub use 分類::{マーカーの実装の事実, 上位トレイトの宣言の事実, 事実の出どころ, 分類の事実, 節点の分類};
pub use 導出::{
    エンティティの識別子の関連型の事実, フィールドの型の表記の事実, フレーム型, プリミティブな事実, 事実の原子, 型の名前の解決, 型の定義の事実, 型の表記, 型の表記の読み取り, 型の表記を読む, 型の表記を読めない理由, 定義の式, 導出の欠け,
    導出の欠けの理由, 導出の結果, 導出の規則, 本体の原子, 概念の変数, 表記が名指す型, 関数の役割, 関数の役割の宣言の事実, 頭の関係,
};
