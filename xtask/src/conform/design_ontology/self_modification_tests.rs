//! `M不変データ` の自己変更の禁止の試験。受け手と引数の形・引数の型の中の可変参照・宣言の書き方の違い・実装の見出しの読み方・実装の対象の型の結び付け・定義の位置のモジュールで数える型の同一性・型の別名を通した結び付け・型を包む実装・
//! トレイトの宣言の関数と宣言を読める範囲・トレイトの同一性・波括弧付きのモジュールの中の `use`・関数の本体などの局所の `use` と項目・マクロの呼び出しの中の `use` と項目・cfg で切り替わる同名の候補・トレイトの宣言の本体のマクロ・prelude と依存クレートの名前を名乗るトレイト・マクロを通した自己変更・対象の型を決められない実装と台帳を、観点ごとのファイルに分けて置く。

mod conditional_candidate_tests;
mod const_generic_impl_tests;
mod declaration_form_tests;
mod definition_position_tests;
mod impl_target_tests;
mod implementation_binding_tests;
mod lexical_scope_tests;
mod macro_expansion_tests;
mod macro_invocation_scope_tests;
mod mutable_self_law_tests;
mod nested_module_use_tests;
mod parameter_type_tests;
mod receiver_form_tests;
mod trait_body_macro_tests;
mod trait_default_method_tests;
mod trait_identity_tests;
mod trait_name_disguise_tests;
mod trait_scope_tests;
mod type_alias_binding_tests;
mod unbound_implementation_tests;
mod wrapping_implementation_tests;
