//! Issue #59の第4段階の段階C(本実装)の検収の計器。本番の当て方(δ の内側の点の行を積まず右辺を −C_t)で、27組(正接0.50・
//! 0.55・0.61 × 回し0・30・45度 × 受理の倍率0.5・1・2倍)と、細分16本の正接0.55の3方向とその旧の形の対照と、刻み2倍の
//! 平行移動と正接0.58を重ねた敵対の反例(本番と候補(i))を600刻み走らせ、分類・坂に沿った変位・退避の件数と理由の内訳・
//! 錨の置き直しを綴る。合否は判定しない(判定は `slope_tests`・`slope_tolerance_scale_tests`・`slope_substep16_tests`・
//! `slope_far_translation_tests`・`tolerance_boundary_tests` が持つ)。
//! 退避の理由は、粘着の候補が解けなかった連立の読み取りの結末の綴りで数える。受理の判定の破れは契約の名前で綴られる。
//! 実行は `cargo test -p blitz_sim --release 本実装の検収 -- --ignored --nocapture` である。
//! 参照: `_doc/計測/剛体の接触の静止摩擦の許容差依存の診断_2026-09-09.md`

#![cfg(test)]

mod classification;
mod matrix;
