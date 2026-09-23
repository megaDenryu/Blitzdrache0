//! `M不変データ` の自己変更の禁止の試験。受け手と引数の形・引数の型の中の可変参照・宣言の書き方の違い・実装の見出しの読み方・定義の位置のモジュールで数える型の同一性・型を包む実装・
//! 名前の閉包(`use … as` と `type` の別名の辺)と実装の在り処を問わないこと・名前が当たった別の型の実装の台帳・トレイトの宣言の関数と宣言の本体のマクロ・マクロを通した自己変更・対象の型を決められない実装と台帳・包んだ可変参照と型の表記の中のマクロの反例を、観点ごとのファイルに分けて置く。

mod const_generic_impl_tests;
mod declaration_form_tests;
mod definition_position_tests;
mod impl_target_tests;
mod macro_expansion_tests;
mod mutable_self_law_tests;
mod name_closure_tests;
mod name_match_exclusion_tests;
mod parameter_type_tests;
mod receiver_form_tests;
mod trait_body_macro_tests;
mod trait_default_method_tests;
mod two_stage_closure_tests;
mod unbound_implementation_tests;
mod wrapped_reference_and_macro_counterexample_tests;
mod wrapping_implementation_tests;
