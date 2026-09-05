//! 剛体の接触の一刻みの工程(判断19)。
//! 基本刻みを整数nで細分し、細分1本の中で予測・接触・反復・速度再構成・速度段階・休止判定・刻み境界確定を回す。

mod pipeline_body_rows;
mod pipeline_contact_batch;
mod pipeline_def;
mod pipeline_error;
mod pipeline_handover;
mod pipeline_history;
mod pipeline_policy;
mod pipeline_solve_body;
mod pipeline_solve_static;
mod pipeline_solver;
mod pipeline_space;
mod pipeline_static_rows;
mod pipeline_step;
mod pipeline_substep;
mod pipeline_velocity;
mod pipeline_velocity_body;
mod step_actions;
mod substep_placement;
mod substep_predict;
mod tentative_multipliers;

#[cfg(test)]
mod contact_test_pipeline_scene;
#[cfg(test)]
mod island_wake_tests;
#[cfg(test)]
mod pipeline_fixture;
#[cfg(test)]
mod pipeline_tests;
#[cfg(test)]
mod side_by_side_fixture;
#[cfg(test)]
mod side_by_side_rest_tests;
#[cfg(test)]
mod substep_impulse_tests;
#[cfg(test)]
mod tower_anchor_counterfactual_tests;
#[cfg(test)]
mod tower_fixture;
#[cfg(test)]
mod tower_invariance_tests;
#[cfg(test)]
mod tower_rest_tests;
#[cfg(test)]
mod tower_standing_tests;
#[cfg(test)]
mod tower_substep_count_tests;
#[cfg(test)]
mod transition_discard_tests;

pub use pipeline_def::剛体の接触の一刻みの工程;
pub use pipeline_error::接触の工程エラー;
pub use pipeline_history::接触履歴の保持;
pub use pipeline_policy::接触の品質と時間方針;
pub use pipeline_solver::接触の解法ソルバー;
pub use pipeline_space::接触の空間と世界;
