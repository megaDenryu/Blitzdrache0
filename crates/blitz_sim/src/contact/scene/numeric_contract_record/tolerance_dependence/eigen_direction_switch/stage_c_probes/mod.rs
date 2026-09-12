//! Issue #103の段階C(一次計測)の計器。親の2026-09-13の指示で置いた4本を1つの木にまとめる。
//! 段階Aと段階Bの計器と同じ場面(細分16本の正接0.55・静止摩擦係数0.6・受理1倍)を読むため親の木の下に置くが、
//! 親の`mod.rs`のモジュールの宣言が100行を超えたため、この木へ分けた。
//! 4本の入口と測るものを書く。
//! 第1手(`perturbation_control_window`)は、同じ窓を3つの対(本番と本番、本番と姿勢を摂動した本番、本番と
//! 代わりの捨て方)で走らせ、円錐の比の差を細分ごとに対で綴る。走行ごとの非決定性が無いことの根拠になる。
//! 第2手(`discarded_direction_census_run`)は、本番だけを9600細分走らせ、本番が右辺の判定で捨てている向きの
//! 相対固有値と右辺と許容差の比と剛体へ入る補正を集める。
//! 第3手(`carried_state_window`)は、細分0の終わりに次の細分へ持ち越される状態をビット列で突き合わせ、
//! 錨からの接線変位の合計も綴る。
//! 第4手(`reconstruction_window`)は、速度の再構成の前と後を分けて綴り、配置の表現の1刻みが速度でいくつになるかを
//! 並べて置く。
//! どれも合否を判定しない。本番の判定と閾値と既定は1つも変えていない。
//! 参照: `_doc/計測/剛体の接触の静止摩擦の許容差依存の診断_2026-09-09.md`

#![cfg(test)]

mod bit_comparison;
mod carried_state;
mod carried_state_window;
mod discarded_direction_census;
mod discarded_direction_census_run;
mod ill_conditioned_direction_reading;
mod ill_conditioned_direction_tally;
mod paired_run;
mod perturbation_control_window;
mod posture_perturbation;
mod reconstruction_floor;
mod reconstruction_pair_line;
mod reconstruction_reading;
mod reconstruction_window;
