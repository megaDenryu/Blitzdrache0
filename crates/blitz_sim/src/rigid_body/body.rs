//! 剛体: 識別子と4群の状態(配置・質量特性・運動種別・実行状態)を持つ1つの動的物体(判断2)。フィールドはすべて私有であり、台帳のメソッドだけが書き換える(判断23)。
//! 作用の受け付けと取り出しは`body_actions`が持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断2」「判断23」。

use super::body_error::剛体エラー;
use super::body_id::剛体の識別子;
use super::body_kind::運動種別;
use super::execution_state::実行状態;
use super::mass_properties::質量特性;
use super::motion_state::運動状態;
use super::placement::配置;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 剛体 {
    識別子: 剛体の識別子,
    配置: 配置,
    質量特性: 質量特性,
    運動種別: 運動種別,
    実行状態: 実行状態,
}

impl 剛体 {
    // 生成された剛体は起きた状態で始める(判断7・判断18)。
    pub(super) fn 生成する(識別子: 剛体の識別子, 配置: 配置, 質量特性: 質量特性, 運動種別: 運動種別) -> Self {
        Self {
            識別子,
            配置,
            質量特性,
            運動種別,
            実行状態: 実行状態::起きている,
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

    /// 動的または運動学的の速度。静的な剛体からは型付きエラーで読めない。
    pub fn 速度(&self) -> Result<運動状態, 剛体エラー> {
        self.運動種別.速度().ok_or(剛体エラー::静的な剛体は速度を持たない { 識別子: self.識別子 })
    }

    // 作用の受け付けが蓄積器を借りるための口。
    pub(super) fn 運動種別を書き換える(&mut self) -> &mut 運動種別 {
        &mut self.運動種別
    }

    // 物理の細分が確定した配置と再構成した速度を動的な剛体へ書き戻す。蓄積器と静穏の数はそのまま保つ。
    pub(crate) fn 配置と速度を書き換える(&mut self, 新しい配置: 配置, 速度: 運動状態) -> Result<(), 剛体エラー> {
        let 運動種別::動的 { 運動状態, .. } = &mut self.運動種別 else {
            return Err(剛体エラー::動的でない剛体の状態を物理が書き換えようとした { 識別子: self.識別子 });
        };
        *運動状態 = 速度;
        self.配置 = 新しい配置;
        Ok(())
    }
}
