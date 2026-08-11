//! アセット生成器の起こし方の共通語彙: xtaskの5つの入口が`blitz_asset_compiler`のexampleを起こすときに使う型の集まり。
//!
//! 層と依存の向きを先に決める。ここは入口を1つも知らず、入口の側だけがここを知る。
//! 実測では5つの入口がそれぞれ`Command::new("cargo")`から書き起こしており、example名の綴りが5箇所、
//! 「起動に失敗」「終了コードで失敗」の文面が5組に散っていた。綴りが散ると、生成器の置き場を変える改定が
//! 5箇所の同時変更になり、直し漏れた入口だけが静かに動かなくなる。
//!
//! 何を生成させるかは`specification`が枝で数え上げ、生成器へ渡す語の並びは`arguments`が、
//! 1回ぶんの起動は`launch`が、破れの型は`error`が持つ。

mod arguments;
mod error;
mod launch;
mod specification;

pub use error::生成器エラー;
pub use launch::アセット生成器の起動;
pub use specification::生成の指定;
