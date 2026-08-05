//! 時間帯の配線本体。担当するのは「世界の空方針と間接照明方針とゲーム時計を持ち、そのフレームのライティング入力・空入力・
//! 露出倍率を返す」ことである。呼び出し元は、世界が空を持つかどうかも時刻の進み方も知らずに値だけを受け取る。
//! 空を持たない世界は`時間帯`を持たず、ライティングも露出も世界が決めた値のままである。
//! 焼き直しの判定を伴う入力の組み立てと2つの更新の記録の読み取りは`bake_input`、生成局面は`create`、露出の合成と自動露出の設定の組み立ては`exposure`、段差の走査が撮影ごとに据える条件は`scan_override`が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「露出の時間変化」

mod bake_input;
mod create;
mod exposure;
mod scan_override;

use super::atmosphere_input;
use super::atmosphere_update::大気更新判定;
use super::clock::時間帯;
use super::distant_environment_update::遠方環境更新判定;
use super::exposure_elapsed::自動露出の経過秒源;
use super::step_scan::段差走査;
use super::thinning::遠方環境の間引き;
pub(in crate::app) use bake_input::焼き上げ入力の組;
use blitz_engine::auto_exposure::露出方式;
use blitz_engine::sky::atmosphere::大気媒体方針;
use blitz_render::atmosphere::大気散乱媒体;
use blitz_render::indirect_lighting::照明問い合わせ契約;
use blitz_render::{ライティング入力, レンダラーエラー, 空入力};
pub(in crate::app) use create::生成材料;
pub(in crate::app) struct 天空配線 {
    時間帯: Option<時間帯>,
    /// 空パスを積むかどうか。世界の方針と起動指定から生成時に決まり、以降変わらない。
    空を描く: bool,
    /// 空を持つ世界だけが持つ、ベイク済み画像生成の入力へ下ろし済みの大気と、その出どころの方針。時刻に依存しないため生成時に1度だけ写す。
    /// 方針を残すのは、焼き直しの判定に使う大気静的キーがそこから決まるためである。
    大気: Option<(大気媒体方針, 大気散乱媒体)>,
    /// 大気のベイク済み画像を焼き直すかどうかの判定。前回焼いたときの鍵だけを持つ。
    大気更新判定: 大気更新判定,
    /// 世界の間接照明方針から起動時に1度決めた契約。以降変わらない。
    照明問い合わせ契約: 照明問い合わせ契約,
    /// 遠方環境を焼き直すかどうかの判定。前回焼いたときの鍵だけを持つ。
    遠方環境更新判定: 遠方環境更新判定,
    /// 遠方環境の更新を間引く刻みと、生成パスがGPUへ届いたフレームの区間の記録。
    間引き: 遠方環境の間引き,
    /// `--ibl-step-scan`指定時だけ`Some`。撮影の列を持ち、フレーム番号から今の撮影を答える。
    段差走査: Option<段差走査>,
    /// 世界が起動時に決めた露出の決め方。以降変わらない。
    露出方式: 露出方式,
    /// 自動露出の時間適応へ渡す経過秒の供給元。実行の種類が起動時に枝を決め、以降は枝が変わらない。
    自動露出の経過秒源: 自動露出の経過秒源,
    /// シーンが決めた基準のライティング。時刻が置き換えるのは方向光・環境光・影の落ち方だけであり、
    /// 点光源と影の正射影範囲はこの基準のまま残す。
    基準ライティング: ライティング入力,
    /// そのフレームのライティング。空を持つ世界では時刻から導き直し、持たない世界では基準のまま動かない。
    ライティング: ライティング入力,
}
impl 天空配線 {
    /// 世界の間接照明方針が大気の媒体を要るのに世界が媒体を持たない場合は、ここで型付きの失敗になる。
    pub(in crate::app) fn 生成する(材料: create::生成材料<'_>) -> Result<Self, レンダラーエラー> {
        create::生成する(材料)
    }
    pub(in crate::app) fn 空を描くか(&self) -> bool {
        self.空を描く
    }
    /// レンダラーが起動時に構築する照明問い合わせ契約。世界の方針と1対1に対応する。
    pub(in crate::app) fn 照明問い合わせ契約(&self) -> 照明問い合わせ契約 {
        self.照明問い合わせ契約
    }
    /// 実時間の経過ぶん時計を進め、時刻が動いたぶんだけライティングと空入力を導き直す。
    pub(in crate::app) fn 進める(&mut self) {
        let Some(時間帯) = &mut self.時間帯 else {
            return;
        };
        時間帯.進める();
        self.ライティングを導き直す();
    }
    /// シャドウマップ資源を確保するときの一辺。シーンの基準ライティングが持つ多段設定から読むため、
    /// 資源の一辺と多段の構築が使う一辺が同じ1つの値から出る。
    pub(in crate::app) fn 影の一辺解像度(&self) -> blitz_render::cascade::影の一辺解像度 {
        self.基準ライティング.多段設定().解像度()
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
    /// そのフレームで使っている天空状態。空の方針を持たない世界では無い。
    pub(in crate::app) fn 天空状態(&self) -> Option<&blitz_engine::sky::天空状態> {
        self.時間帯.as_ref().map(時間帯::状態)
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
