//! ベイク済み画像生成の入力へ下ろし済みの大気。空を持つ世界が生成時に1度だけ、大気媒体方針と、
//! それをレンダラーの数学が使う形へ下ろした大気散乱媒体との組として作り、時刻に依存しないため
//! フレームごとに作り直さない。方針を残すのは、焼き直しの判定に使う大気静的キーがそこから決まるためである。

use blitz_engine::sky::atmosphere::{大気媒体方針, 大気静的キー};
use blitz_render::atmosphere::大気散乱媒体;

use super::super::atmosphere_input;
use super::super::clock::時間帯;
use super::super::空の再現条件;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::app::time_of_day::wiring) struct 下ろし済みの大気 {
    方針: 大気媒体方針,
    媒体: 大気散乱媒体,
}

impl 下ろし済みの大気 {
    pub(in crate::app::time_of_day::wiring) fn 生成する(方針: 大気媒体方針, 媒体: 大気散乱媒体) -> Self {
        Self { 方針, 媒体 }
    }

    pub(in crate::app::time_of_day::wiring) fn 方針(&self) -> 大気媒体方針 {
        self.方針
    }

    pub(in crate::app::time_of_day::wiring) fn 媒体(&self) -> 大気散乱媒体 {
        self.媒体
    }

    /// 大気のベイク済み画像を焼き直すかの判定に使う鍵。方針だけから決まる値だが、この対が表す
    /// 大気を一意に識別する値でもあるため、呼び出し側が方針へ改めて触れずここから引けるようにする。
    pub(in crate::app::time_of_day::wiring) fn 静的キー(&self) -> 大気静的キー {
        self.方針.静的キー()
    }

    /// 空代表画素の照合に要る再現条件。大気入力と同じ大気・時間帯から作るため、組む工程自体は
    /// atmosphere_inputへ置いたまま、この型からも呼べるようにする。
    pub(in crate::app::time_of_day::wiring) fn 再現条件を組む(&self, 時間帯: &時間帯) -> 空の再現条件 {
        atmosphere_input::再現条件を組む(self.媒体, 時間帯)
    }
}
