//! 判断13と判断14の定量の基準を測るための試験専用の場面(判断19の細分の工程が組み上がるまでの間の材料)。
//! 1つの動的な直方体と1つの静的な直方体だけを、細分の工程の順(予測→接触点集合→履歴の併走→反復→速度の再構成→速度段階)で進める。
//! 本物の細分の工程は後続の便が`blitz_sim`の一刻みの工程として組む。ここに置くのは、坂と落下の基準がこの便の検収の材料であり、
//! 工程の完成を待つと判断13と判断14の式を実際に配置へ適用して測り直せないためである。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断19: 剛体は基本刻みを整数nで細分し、細分1本の中で予測・接触・反復・速度再構成・速度段階を回す」

mod anchor_retention_tests;
mod drop_tests;
mod scene_geometry;
mod scene_settings;
mod slope_fixture;
mod slope_geometry;
mod slope_tests;
mod substep_harness;
mod substep_solve;
mod substep_step;
mod substep_velocity_stage;
