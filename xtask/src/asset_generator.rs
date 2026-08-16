//! アセット生成器の起こし方の共通語彙: xtaskの5つの入口が`blitz_asset_compiler`のexampleを起こすときに使う型の集まり。
//!
//! 層と依存の向きを先に決める。ここは入口を1つも知らず、入口の側だけがここを知る。
//! 実測では5つの入口がそれぞれ`Command::new("cargo")`から書き起こしており、example名の綴りが5箇所、
//! 「起動に失敗」「終了コードで失敗」の文面が5組に散っていた。綴りが散ると、生成器の置き場を変える改定が
//! 5箇所の同時変更になり、直し漏れた入口だけが静かに動かなくなる。
//!
//! 焼ける世界の数え上げは`world_name`が持つ。これが生成器の語彙であるのは、綴りの正本が生成器側にあり、
//! 焼ける対象として渡せるものの集合を決めるのが生成器だからである。
//! 何を生成させるかは`specification`が枝で数え上げ、生成器へ渡す語の並びは`arguments`が、
//! 1回ぶんの起動は`launch`が、破れの型は`error`が持つ。

mod arguments;
mod error;
mod inspection_files;
mod launch;
mod launch_command;
mod planting_count;
mod specification;
mod world_name;

pub use error::生成器エラー;
pub use inspection_files::検査するファイル一覧;
pub use launch::アセット生成器の起動;
pub use planting_count::同居植生の個体数;
pub use specification::生成の指定;
pub use world_name::世界名;
