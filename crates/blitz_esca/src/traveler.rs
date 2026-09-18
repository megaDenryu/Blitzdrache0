//! 主人公(旅行者)の移動意図・現在地・歩行規則(判断: Esca設計正本)。
//!
//! プログラミング設計上の役割(コマンド、状態+DTO、規則、イベント)に従って定義する。

use blitz_design::{MDTO, Mイベント, Mコマンド, M状態, M規則};
use blitz_math::{メートル, メートル毎秒};

use crate::traveler_error::歩行の規則の生成の失敗;
use crate::walking_direction::歩行方向;

/// 旅行者が行おうとするアクション(命令・意図)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum 旅行者の意図 {
    静止,
    歩く { 方向: 歩行方向 },
    見回す,
}

impl Mコマンド for 旅行者の意図 {}

/// 旅行者のワールド内の現在地(状態スナップショット)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 旅行者の現在地 {
    東: メートル,
    北: メートル,
}

impl MDTO for 旅行者の現在地 {}
impl M状態 for 旅行者の現在地 {}

impl 旅行者の現在地 {
    pub fn 生成する(東: メートル, 北: メートル) -> Self {
        Self { 東, 北 }
    }

    pub fn 東(&self) -> メートル {
        self.東
    }

    pub fn 北(&self) -> メートル {
        self.北
    }
}

/// 旅行者が歩く際の不変の規則(身体の運動法則)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 歩行の規則 {
    一秒あたりの速さ: メートル毎秒,
}

impl M規則 for 歩行の規則 {}

impl 歩行の規則 {
    pub fn 標準() -> Self {
        Self {
            一秒あたりの速さ: メートル毎秒::生成する(3.0)
        }
    }

    /// 速さが有限かつ0以上であることを確かめて規則を作る。0を許すのは、静止は `旅行者の意図::静止` で表すため速さ0の規則が矛盾を生まないことによる。
    pub fn 生成する(一秒あたりの速さ: メートル毎秒) -> Result<Self, 歩行の規則の生成の失敗> {
        let 値 = 一秒あたりの速さ.値();
        if !値.is_finite() || 値 < 0.0 {
            return Err(歩行の規則の生成の失敗::速さが有限な0以上の数値でない { 速さ: 一秒あたりの速さ });
        }
        Ok(Self { 一秒あたりの速さ })
    }

    pub fn 速さ(&self) -> メートル毎秒 {
        self.一秒あたりの速さ
    }
}

/// 旅行者の行動によって世界に発生した出来事(過去の事実)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 旅行者の出来事 {
    /// 見回す意図の遷移で、旅行者が周囲を観察した事実。
    周囲を観察した,
    /// 移動先が移動可能範囲を越え、実際の移動位置が制限された事実。
    障害物に遮られた,
}

impl Mイベント for 旅行者の出来事 {}
