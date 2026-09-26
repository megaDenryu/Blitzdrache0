//! Escaクレートの試験(Issue #137)。設計オントロジーの構文の法則は `cargo xtask conform` が検査する。

mod behavior_bits;
mod behavior_coverage_tests;
mod behavior_display;
mod behavior_domain;
mod behavior_identity;
mod behavior_inference_tests;
mod behavior_landmark;
mod behavior_naming;
mod behavior_naming_tests;
mod behavior_proposition;
mod behavior_reachable_tests;
mod elapsed_time_tests;
mod traveler_input_tests;
mod traveler_invariant_tests;
mod traveler_tests;
mod walking_direction_tests;
#[path = "一つの区域の試料.rs"]
mod 一つの区域の試料;
#[path = "刻みの区間の精度の試験.rs"]
mod 刻みの区間の精度の試験;
#[path = "区域の歩行の試験.rs"]
mod 区域の歩行の試験;
#[path = "区域の閉包に留まる歩行の試験.rs"]
mod 区域の閉包に留まる歩行の試験;
#[path = "地図の検証の試験.rs"]
mod 地図の検証の試験;
#[path = "地点と経路の試料.rs"]
mod 地点と経路の試料;
#[path = "境目の許容幅の近くの歩行の試験.rs"]
mod 境目の許容幅の近くの歩行の試験;
#[path = "平面の幾何の試験.rs"]
mod 平面の幾何の試験;
#[path = "旅の数え上げ.rs"]
mod 旅の数え上げ;
#[path = "旅の数え上げの試料.rs"]
mod 旅の数え上げの試料;
#[path = "旅の数え上げの試験.rs"]
mod 旅の数え上げの試験;
#[path = "時刻と時間帯と地図の試験.rs"]
mod 時刻と時間帯と地図の試験;
#[path = "空間制約と開閉の状態の地図の照合の試験.rs"]
mod 空間制約と開閉の状態の地図の照合の試験;
#[path = "経路の開閉と旅行者の歩行の試験.rs"]
mod 経路の開閉と旅行者の歩行の試験;
#[path = "経路の開閉の出来事と予定の試験.rs"]
mod 経路の開閉の出来事と予定の試験;
#[path = "経路の開閉の数え上げ.rs"]
mod 経路の開閉の数え上げ;
#[path = "経路の開閉の数え上げの試験.rs"]
mod 経路の開閉の数え上げの試験;
#[path = "経路の開閉の状態の作り方の試験.rs"]
mod 経路の開閉の状態の作り方の試験;
#[path = "経路の開閉の遷移の試験.rs"]
mod 経路の開閉の遷移の試験;
