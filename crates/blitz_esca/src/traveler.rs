//! 主人公(旅行者)の移動意図・現在地・歩行規則(判断: Esca設計正本)。
//!
//! プログラミング設計上の役割(コマンド、状態+DTO、規則)に従って定義する。

use blitz_math::メートル;

use crate::ontology::{MDTO, Mイベント, Mコマンド, M入力, M状態, M規則};

/// デバイスから届く移動の生入力シグナル(生シグナル is a DTO)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 移動の生入力 {
    pub 東西方向: f32,
    pub 南北方向: f32,
}

impl M入力 for 移動の生入力 {}
impl MDTO for 移動の生入力 {}

/// 旅行者が行おうとするアクション(命令・意図)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum 旅行者の意図 {
    静止,
    歩く { 東の比率: f32, 北の比率: f32 },
    見回す,
}

impl Mコマンド for 旅行者の意図 {}

/// 旅行者のワールド内の現在地(状態スナップショット)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 旅行者の現在地 {
    東: メートル,
    北: メートル,
}

impl M状態 for 旅行者の現在地 {}
impl MDTO for 旅行者の現在地 {}

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
    一秒あたりの速さ: メートル,
}

impl M規則 for 歩行の規則 {}

impl 歩行の規則 {
    pub fn 標準() -> Self {
        Self {
            一秒あたりの速さ: メートル::生成する(3.0),
        }
    }

    pub fn 生成する(一秒あたりの速さ: メートル) -> Self {
        Self { 一秒あたりの速さ }
    }

    pub fn 速さ(&self) -> メートル {
        self.一秒あたりの速さ
    }
}

/// 旅行者の行動によって世界に発生した出来事(過去の事実)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 旅行者の出来事 {
    歩行を開始した,
    立ち止まった,
    周囲を観察した,
    障害物に遮られた,
}

impl Mイベント for 旅行者の出来事 {}

