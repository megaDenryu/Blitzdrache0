//! 天空配線のうち、そのフレームで何を焼き直すかを決める操作。触れるフィールドは時間帯・下ろし済みの大気・
//! 間接照明方針・2つの更新判定だけであり、ライティングと露出へは触れない。
//!
//! 大気と遠方環境を1つの呼び出しで返すのは、どちらの鍵もスカイビューキーから決まり、別々に導くと
//! 同じフレームの2つの鍵が違うスカイビューキーから作られうるためである。
//!
//! 呼び出し規律: 2つの更新判定は前回焼いた鍵を書き換えるため、1フレームに1度だけ呼ぶ。

use blitz_render::atmosphere_lut_input::大気のベイク済み画像の入力;
use blitz_render::distant_environment::遠方環境の入力;

use super::super::atmosphere_input;
use super::天空配線;
use crate::app::frame::フレーム視点;

/// そのフレームの焼き上げ入力の組。どちらも、その資源を作らない構成・契約では`None`である。
pub(in crate::app) struct 焼き上げ入力の組 {
    pub(in crate::app) 大気: Option<大気のベイク済み画像の入力>,
    pub(in crate::app) 遠方環境: Option<遠方環境の入力>,
}

impl 天空配線 {
    /// ベイク済み画像生成の入力になる大気と2つの観測条件、遠方環境の明るさの尺度、そのフレームで何を焼き直すかの指示。
    /// 空中遠近の条件はカメラに依るため視点を受け取る。大気のベイク済み画像資源は空段階を持つフレーム構成でだけ
    /// 作られるため、空パスを積まない指定(`--no-sky`)では下ろした媒体を持っていても渡さない。
    pub(in crate::app) fn 焼き上げ入力(&mut self, 視点: &フレーム視点) -> 焼き上げ入力の組 {
        let 無し = 焼き上げ入力の組 {
            大気: None, 遠方環境: None
        };
        let Some((方針, 媒体)) = self.空を描く.then_some(self.大気).flatten() else {
            return 無し;
        };
        let Some(時間帯) = self.時間帯.as_ref() else {
            return 無し;
        };
        let 空描画 = 時間帯.空描画方針();
        let 状態 = *時間帯.状態();
        let 材料 = atmosphere_input::大気入力の材料 {
            方針: &方針,
            媒体,
            状態: &状態,
            空描画,
            視点,
        };
        let 導出 = atmosphere_input::組む(&材料, &mut self.大気更新判定);
        焼き上げ入力の組 {
            大気: Some(導出.入力),
            遠方環境: self
                .遠方環境を焼くか()
                .then(|| atmosphere_input::遠方環境を組む(空描画, 導出.スカイビュー鍵, &mut self.遠方環境更新判定)),
        }
    }
}
