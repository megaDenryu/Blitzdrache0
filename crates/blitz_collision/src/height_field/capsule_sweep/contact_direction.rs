//! 候補を接触として持つかを、保持したカプセルの掃引の変位から判定する工程。触れるのは求解が保持するカプセルの
//! 掃引の変位だけである。始まりから触れている候補と掃引の途中の根の両方(`contact_solver.rs`)がこの判定を通す。
//! 規則そのものは求解の部品が持つ。
//! 参照: `crates/blitz_collision/src/solver/approach.rs`

use super::super::grid_origin_displacement::高さ場の中の変位;
use super::contact_place::触れた場所;
use super::contact_solver::カプセルと三角形の接触の求解;
use crate::solver::掃引が面へ向かうかの判定;

impl カプセルと三角形の接触の求解<'_> {
    pub(super) fn 面へ向かう接触か(&self, 場所: &触れた場所) -> bool {
        掃引が面へ向かうかの判定::掃引の変位から生成する(self.掃引の変位())
            .接触として持つか(高さ場の中の変位::単位向きから読む(場所.接触の単位法線()))
    }
}
