//! ゲーム配線がゲーム固有の状態を世界実行へ読ませる口。プレイヤーのエンティティID・刻み境界の確定位置・移動状態・
//! 画面へ重ねる表示内容・終了時の要約であり、どれもゲームの状態を読むだけで書き換えない。
//! 共有の状態(台帳・カメラ・移動の記録)はここに現れず、世界実行が自分の所有物と組み合わせて答えを作る。

use blitz_game::移動状態;
use blitz_math::大域ワールド位置;

use super::fox_player::プレイヤーのエンティティID;
use super::ゲーム配線;
use crate::overlay_ui::ゲーム画面の表示内容;
use crate::world_execution::contract::ゲーム進行の要約;
use crate::world_execution::entity_id::エンティティID;
use crate::world_execution::movement_record::移動の観測の記録;

impl ゲーム配線 {
    /// 遊ばない起動はプレイヤーを持たず`None`である。
    pub(in crate::world_execution) fn プレイヤーのエンティティid(&self) -> Option<エンティティID> {
        match self {
            Self::ゲームを遊ばない => None,
            Self::キツネの場所巡り(_) | Self::歩くだけ(_) => Some(プレイヤーのエンティティID),
        }
    }

    pub(in crate::world_execution) fn 刻み境界で確定したプレイヤーの大域位置(&self) -> Option<大域ワールド位置> {
        match self {
            Self::ゲームを遊ばない => None,
            Self::キツネの場所巡り(配線) => Some(配線.プレイヤーの大域位置()),
            Self::歩くだけ(配線) => Some(配線.プレイヤーの大域位置()),
        }
    }

    pub(in crate::world_execution) fn プレイヤーの移動状態(&self) -> Option<移動状態> {
        match self {
            Self::ゲームを遊ばない => None,
            Self::キツネの場所巡り(配線) => Some(配線.プレイヤーの移動状態()),
            Self::歩くだけ(配線) => Some(配線.プレイヤーの移動状態()),
        }
    }

    /// そのフレームにゲームの画面が出す値。歩くだけと遊ばない起動は画面を持たない。
    pub(in crate::world_execution) fn 画面へ重ねる表示内容を作る(&self) -> Option<ゲーム画面の表示内容> {
        match self {
            Self::ゲームを遊ばない | Self::歩くだけ(_) => None,
            Self::キツネの場所巡り(配線) => Some(配線.画面へ重ねる表示内容を作る()),
        }
    }

    /// 終了時の報告へ渡す要約。移動の観測を受け取るのは、要約が共有の記録とゲーム固有の状態の両方から作られるためである。
    pub(in crate::world_execution) fn 進行の要約を作る(&self, 移動の観測: &移動の観測の記録) -> Option<ゲーム進行の要約> {
        match self {
            Self::ゲームを遊ばない | Self::歩くだけ(_) => None,
            Self::キツネの場所巡り(配線) => Some(配線.進行の要約を作る(移動の観測)),
        }
    }
}
