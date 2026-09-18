//! キツネをプレイヤーの動く個体として台帳へ登録する共通宣言。

use blitz_math::大域ワールド位置;

use super::fox_tour::facing::{キツネが読込時に向いている方位角, 世界での向きを読込時からの回転角へ写す};
use crate::app::scene_load::起動時シーンの束ID;
use crate::world_execution::entity_id::エンティティID;
use crate::world_execution::entity_ledger::{ゲーム状態の台帳, 動く個体の描画先};

pub(super) const プレイヤーのエンティティID: エンティティID = エンティティID::生成する(0);

pub(super) fn 原点のキツネを登録した台帳を作る() -> ゲーム状態の台帳 {
    let mut 台帳 = ゲーム状態の台帳::動く個体を1体も持たない台帳を作る();
    台帳.動く個体を登録する(
        プレイヤーのエンティティID,
        動く個体の描画先 {
            束id: 起動時シーンの束ID,
            描画対象添字: 0,
            個体添字: 0,
        },
        大域ワールド位置::原点(),
        世界での向きを読込時からの回転角へ写す(キツネが読込時に向いている方位角()),
    );
    台帳
}
