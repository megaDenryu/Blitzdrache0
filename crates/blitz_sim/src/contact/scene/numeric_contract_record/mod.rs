//! Issue #59が求める数値契約の診断の計器。合否は判定せず、表を作るだけである。
//! 走査は4つある。坂の境界(傾きの正接と鉛直軸まわりの回しの組)、世界内の平行移動、解法の許容差、
//! および分類が1つの条件だけで分かれた対が分岐する細分の探索である。
//! どの走査も、単精度の本番の経路と倍精度の参照計算を同じ細分の同じ地点で並べて綴る。
//! 判定する検査は`slope_tests`と`tolerance_boundary_tests`が持ち、この計器はそれらの閾値も場面も動かさない。
//! 実行は `cargo test -p blitz_sim --release <試験名> -- --ignored --nocapture` である。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

#![cfg(test)]

mod breakdown_line;
mod candidate_reading;
mod comparison_line;
mod comparison_substep;
mod diagnostic_case;
mod divergence_pair;
mod divergence_scan;
mod double_precision_reading;
mod holding_class;
mod single_precision_reading;
mod slope_boundary_table;
mod tolerance_table;
mod translation_table;
