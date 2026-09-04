//! 剛体: 識別子と4群の状態(配置・質量特性・運動種別・実行状態)を持つ1つの動的物体(判断2)。フィールドはすべて私有であり、台帳のメソッドだけが書き換える(判断23)。
//! 作用の受け付けと取り出しは`body_actions`が、状態の書き換えは子の`state_change`が持つ。私有のフィールドへ触れる範囲をこのモジュールの木の中へ閉じるため、剛体は単一のファイルでなくモジュールで持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断2」「判断23」。

mod state_change;

use super::action_accumulator::一刻みの作用の蓄積器;
use super::body_error::剛体エラー;
use super::body_id::剛体の識別子;
use super::body_kind::運動種別;
use super::execution_state::実行状態;
use super::mass_properties::質量特性;
use super::motion_state::運動状態;
use super::placement::配置;
use super::wake_reason::休止から起きた理由;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 剛体 {
    識別子: 剛体の識別子,
    配置: 配置,
    質量特性: 質量特性,
    運動種別: 運動種別,
    実行状態: 実行状態,
    直近に休止から起きた理由: Option<休止から起きた理由>,
}

impl 剛体 {
    // 生成された剛体は起きた状態で始める(判断7・判断18)。まだ休止したことが無いため起きた理由は持たない。
    pub(in crate::rigid_body) fn 生成する(
        識別子: 剛体の識別子, 配置: 配置, 質量特性: 質量特性, 運動種別: 運動種別
    ) -> Self {
        Self {
            識別子,
            配置,
            質量特性,
            運動種別,
            実行状態: 実行状態::起きている,
            直近に休止から起きた理由: None,
        }
    }

    pub fn 識別子(&self) -> 剛体の識別子 {
        self.識別子
    }

    pub fn 配置(&self) -> &配置 {
        &self.配置
    }

    pub fn 質量特性(&self) -> &質量特性 {
        &self.質量特性
    }

    pub fn 運動種別(&self) -> &運動種別 {
        &self.運動種別
    }

    pub fn 実行状態(&self) -> 実行状態 {
        self.実行状態
    }

    /// この剛体が最後に休止から起きたときの契機(判断18)。休止したことが無ければ持たない。
    pub fn 直近に休止から起きた理由(&self) -> Option<休止から起きた理由> {
        self.直近に休止から起きた理由
    }

    /// 動的または運動学的の速度。静的な剛体からは型付きエラーで読めない。
    pub fn 速度(&self) -> Result<運動状態, 剛体エラー> {
        self.運動種別.速度().ok_or(剛体エラー::静的な剛体は速度を持たない { 識別子: self.識別子 })
    }

    // 作用の受け付けが蓄積器を借りるための口。触れるのは動的剛体の蓄積器だけである。
    pub(in crate::rigid_body) fn 蓄積器を借りる(&mut self) -> Result<&mut 一刻みの作用の蓄積器, 剛体エラー> {
        let 識別子 = self.識別子;
        match &mut self.運動種別 {
            運動種別::動的 { 作用の蓄積器, .. } => Ok(作用の蓄積器),
            運動種別::運動学的 { .. } => Err(剛体エラー::運動学的な剛体へ作用を加えようとした { 識別子 }),
            運動種別::静的 => Err(剛体エラー::静的な剛体へ作用を加えようとした { 識別子 }),
        }
    }
}
