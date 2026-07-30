//! 時間帯の配線本体。担当するのは「世界の空方針とゲーム時計を持ち、そのフレームのライティング入力・空入力・大気LUT入力・
//! 露出倍率を返す」ことである。呼び出し元は、世界が空を持つかどうかも時刻の進み方も知らずに値だけを受け取る。
//! 空を持たない世界は`時間帯`を持たず、ライティングも露出も世界が決めた値のままである。
//! 生成局面は`create`が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「露出の時間変化」

mod create;

use blitz_engine::sky::atmosphere::大気媒体方針;
use blitz_render::atmosphere::大気散乱媒体;
use blitz_render::atmosphere_lut_input::大気LUT入力;
use blitz_render::{ライティング入力, 空入力};

use super::atmosphere_input;
use super::atmosphere_update::大気更新判定;
use super::clock::時間帯;
use crate::app::frame::フレーム視点;
use crate::cli::時間帯起動設定;

pub(in crate::app) struct 天空配線 {
    時間帯: Option<時間帯>,
    /// 空パスを積むかどうか。世界の方針と起動指定から生成時に決まり、以降変わらない。
    空を描く: bool,
    /// 空を持つ世界だけが持つ、LUT生成の入力へ下ろし済みの大気と、その出どころの方針。時刻に依存しないため生成時に1度だけ写す。
    /// 方針を残すのは、焼き直しの判定に使う大気静的キーがそこから決まるためである。
    大気: Option<(大気媒体方針, 大気散乱媒体)>,
    /// 大気LUTを焼き直すかどうかの判定。前回焼いたときの鍵だけを持つ。
    大気更新判定: 大気更新判定,
    /// シーンが決めた基準のライティング。時刻が置き換えるのは方向光・環境光・影の落ち方だけであり、
    /// 点光源と影の正射影範囲はこの基準のまま残す。
    基準ライティング: ライティング入力,
    /// そのフレームのライティング。空を持つ世界では時刻から導き直し、持たない世界では基準のまま動かない。
    ライティング: ライティング入力,
}

impl 天空配線 {
    pub(in crate::app) fn 生成する(シーン名: &str, 設定: &時間帯起動設定, 基準: ライティング入力) -> Self {
        create::生成する(シーン名, 設定, 基準)
    }

    pub(in crate::app) fn 空を描くか(&self) -> bool {
        self.空を描く
    }

    /// 実時間の経過ぶん時計を進め、時刻が動いたぶんだけライティングと空入力を導き直す。
    pub(in crate::app) fn 進める(&mut self) {
        let Some(時間帯) = &mut self.時間帯 else {
            return;
        };
        時間帯.進める();
        self.ライティングを導き直す();
    }

    pub(in crate::app) fn ライティング(&self) -> ライティング入力 {
        self.ライティング
    }

    pub(in crate::app) fn 空入力(&self) -> Option<空入力> {
        self.空を描く.then(|| self.時間帯.as_ref().map(時間帯::空入力)).flatten()
    }

    pub(in crate::app) fn 再現条件(&self) -> Option<super::空の再現条件> {
        let (_, 媒体) = self.空を描く.then_some(self.大気).flatten()?;
        Some(atmosphere_input::再現条件を組む(媒体, self.時間帯.as_ref()?))
    }

    /// LUT生成の入力になる大気と2つの観測条件、そのフレームで何を焼き直すかの指示。空中遠近の条件はカメラに依るため視点を受け取る。大気LUT資源は空段階を持つフレーム構成でだけ
    /// 作られるため、空パスを積まない指定(`--no-sky`)では下ろした媒体を持っていても渡さない。
    pub(in crate::app) fn 大気lut入力(&mut self, 視点: &フレーム視点) -> Option<大気LUT入力> {
        let (方針, 媒体) = self.空を描く.then_some(self.大気).flatten()?;
        let 時間帯 = self.時間帯.as_ref()?;
        let 空描画 = 時間帯.空描画方針();
        let 状態 = *時間帯.状態();
        let 材料 = atmosphere_input::大気入力の材料 {
            方針: &方針,
            媒体,
            状態: &状態,
            空描画,
            視点,
        };
        Some(atmosphere_input::組む(&材料, &mut self.大気更新判定))
    }

    /// 最終露出倍率。基準の露出倍率へ、天空状態の露出補正段を2の冪として掛ける。
    pub(in crate::app) fn 露出倍率(&self, 基準露出: f32) -> f32 {
        let Some(時間帯) = &self.時間帯 else {
            return 基準露出;
        };
        基準露出 * 2.0_f32.powf(時間帯.状態().露出補正段().値())
    }

    fn ライティングを導き直す(&mut self) {
        let Some(時間帯) = &self.時間帯 else {
            self.ライティング = self.基準ライティング;
            return;
        };
        self.ライティング = blitz_engine::天空状態をライティングへ写す(self.基準ライティング, 時間帯.状態())
            .unwrap_or_else(|誤り| panic!("天空状態からライティング入力を作れなかった: {誤り}"));
    }
}
