//! 有限全数の検証器自身の単体試験。判定は子モジュールが持ち、題材は `fixture` が持つ。
//!
//! **ここに在る題材は検証器自身の単体試験のfixtureであり、実ゲームの仕様の代表ケースでも受理の根拠でもない**
//! (Issue #174 のオーナー裁定)。`cargo xtask design-verify` はこの題材を1度も呼ばない。実ゲームの振る舞いの命題は、
//! ドメインのクレートの試験が実物の遷移関数を呼んで検証する。
//!
//! 純粋な計算だけであり外部とのやり取りを持たないため、`tests/` へ実行ファイルを増やさず `src` の中に置く
//! (CLAUDE.md「ファイル・関数の分割」のクレートの `tests/` の直下についての条)。

mod asymmetry_tests;
mod exploration_tests;
mod fixture;
mod representative_tests;
