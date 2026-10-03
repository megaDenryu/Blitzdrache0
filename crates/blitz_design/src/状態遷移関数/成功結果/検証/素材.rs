use crate::{MParameter, Mイベント, Mコマンド, M不変データ, M状態, M状態遷移が受け取る命令, M規則, 状態遷移失敗結果, 状態遷移成功結果};
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct 検証用の状態;
impl M不変データ for 検証用の状態 {}
impl M状態 for 検証用の状態 {}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum 検証用の出来事 {
    開始した,
    終了した,
}
impl M不変データ for 検証用の出来事 {}
impl Mイベント for 検証用の出来事 {}

#[derive(Clone)]
pub(super) enum 検証用の命令 {
    進める,
}
impl M不変データ for 検証用の命令 {}
impl M状態遷移が受け取る命令 for 検証用の命令 {}
impl Mコマンド for 検証用の命令 {}

#[derive(Clone)]
pub(super) struct 検証用の規則;
impl M不変データ for 検証用の規則 {}
impl M規則 for 検証用の規則 {}

#[derive(Clone)]
pub(super) struct 検証用の状態遷移パラメータ;
impl M不変データ for 検証用の状態遷移パラメータ {}
impl MParameter for 検証用の状態遷移パラメータ {}

impl 検証用の状態 {
    pub(super) fn 配列の出来事を返す(
        self,
        _: &検証用の命令,
        _: &検証用の規則,
        _: 検証用の状態遷移パラメータ,
    ) -> Result<状態遷移成功結果<Self, 検証用の出来事, [検証用の出来事; 1]>, 状態遷移失敗結果<Self, Infallible>> {
        Ok(状態遷移成功結果::生成する(self, [検証用の出来事::開始した]))
    }
}
