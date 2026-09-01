//! キツネの場所巡りの配線が、台帳・カメラ・移動の記録・画面の表示内容・終了時の要約を外へ渡す口。
//! 触れるのは読むか借りるだけであり、状態を進めるのは親モジュールの1刻みの更新だけである。

use blitz_engine::height_field::高さ場の読み口;
use blitz_math::大域ワールド位置;

use super::キツネの場所巡りの配線;
use crate::app::描画補間の割合;
use crate::cli::遊ぶゲームの指定;
use crate::game::camera_wiring::プレイヤーカメラの配線;
use crate::game::entity_ledger::ゲーム状態の台帳;
use crate::game::instrument::移動の計器;
use crate::game::movement_record::移動の観測の記録;
use crate::game::summary::ゲーム進行の要約;
use crate::overlay_ui::ゲーム画面の表示内容;

impl キツネの場所巡りの配線 {
    pub(in crate::game) fn 台帳(&self) -> &ゲーム状態の台帳 {
        &self.台帳
    }

    pub(in crate::game) fn 台帳を借りる(&mut self) -> &mut ゲーム状態の台帳 {
        &mut self.台帳
    }

    pub(in crate::game) fn プレイヤーの大域位置(&self) -> 大域ワールド位置 {
        self.状態.プレイヤーの大域位置()
    }

    pub(in crate::game) fn カメラの注視点にする混ぜた大域位置(&self, 割合: 描画補間の割合) -> 大域ワールド位置 {
        self.カメラ
            .注視点を足元から求める(self.台帳.エンティティの混ぜた大域位置を求める(self.プレイヤーのエンティティid(), 割合))
    }

    pub(in crate::game) fn カメラ(&self) -> &プレイヤーカメラの配線 {
        &self.カメラ
    }

    pub(in crate::game) fn カメラを借りる(&mut self) -> &mut プレイヤーカメラの配線 {
        &mut self.カメラ
    }

    pub(in crate::game) fn カメラと高さ場の読み口を借りる(
        &mut self,
    ) -> (&mut プレイヤーカメラの配線, &高さ場の読み口) {
        (&mut self.カメラ, &self.高さ場の読み口)
    }

    pub(in crate::game) fn 移動の記録を借りる(&mut self) -> &mut 移動の観測の記録 {
        &mut self.移動の記録
    }

    pub(in crate::game) fn 移動の計器を作る(&self) -> 移動の計器 {
        self.移動の記録.計器を作る(self.状態.プレイヤーの移動状態())
    }

    // そのフレームにゲームの画面が出す値。進行段階と巡った数だけを写し、位置も時刻も渡さない。
    pub(in crate::game) fn 画面へ重ねる表示内容を作る(&self) -> ゲーム画面の表示内容 {
        ゲーム画面の表示内容 {
            進行段階: self.状態.進行段階(),
            到達済みの目的地数: self.状態.場所巡り().到達済みの目的地数(),
            目的地の総数: self.道順.目的地の総数(),
        }
    }

    pub(in crate::game) fn 進行の要約を作る(&self) -> ゲーム進行の要約 {
        ゲーム進行の要約 {
            ゲームの表示名: 遊ぶゲームの指定::キツネの場所巡り(self.操作の出どころ).表示名(),
            ゲーム更新の回数: self.ゲーム更新の回数,
            最後の進行段階の呼び名: self.状態.進行段階().呼び名(),
            到達済みの目的地数: self.状態.場所巡り().到達済みの目的地数(),
            目的地の総数: self.道順.目的地の総数(),
            プレイヤーの大域位置: self.状態.プレイヤーの大域位置(),
            最後の移動状態の呼び名: self.状態.プレイヤーの移動状態().呼び名(),
            移動の問い合わせ件数の合計: self.移動の記録.問い合わせ件数の合計(),
            所要時間を測った刻みの回数: self.移動の記録.刻みの回数(),
            刻みの所要時間の合計: self.移動の記録.刻みの所要時間の合計(),
        }
    }
}
