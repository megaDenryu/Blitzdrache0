//! `M不変データ` の自己変更の禁止の試験。受け手と引数の形・実装の見出しの読み方・実装の対象の型の結び付け・トレイトの宣言の関数と宣言を読める範囲・マクロを通した自己変更・
//! 対象の型を決められない実装と台帳を、観点ごとのファイルに分けて置く。

mod const_generic_impl_tests;
mod impl_target_tests;
mod implementation_binding_tests;
mod macro_expansion_tests;
mod mutable_self_law_tests;
mod receiver_form_tests;
mod trait_default_method_tests;
mod trait_scope_tests;
mod unbound_implementation_tests;
