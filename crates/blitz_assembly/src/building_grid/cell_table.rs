//! 升目の表: 座標から升目の宣言を引く台帳。近さの問い(隣・真上・真下に升目があるか)もここが答える。
//!
//! 裸の連想表を検査と変換へ渡さないのは、近さの導出が呼び出し側へ散るためである。散ると、隣を数える規則が
//! 検査と変換で食い違い、検査を通った格子が変換で別の形に組まれる。
//!
//! 並びを昇順に固定した表を使うのは、走査の順序が指示の並びの順序を決めるためである。挿入の順に依る表を
//! 使うと、同じ升目の集まりから編集の履歴しだいで違う手順が出る。

use std::collections::BTreeMap;

use super::cell::升目の宣言;
use super::cell_coordinate::升目の座標;
use super::error::ベイ格子エラー;
use super::side_face::升目の側面;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct 升目の表 {
    座標ごとの宣言: BTreeMap<升目の座標, 升目の宣言>,
}

impl 升目の表 {
    pub(super) fn 空から始める() -> Self {
        Self {
            座標ごとの宣言: BTreeMap::new(),
        }
    }

    /// 同じ座標へ2度置くのを拒む。1つの座標に据わる骨格は1件であり、後から置いたもので黙って上書きすると
    /// 人が消したつもりのない升目が消える。
    pub(super) fn 升目を1つ置く(&mut self, 座標: 升目の座標, 宣言: 升目の宣言) -> Result<(), ベイ格子エラー> {
        if self.座標ごとの宣言.contains_key(&座標) {
            return Err(ベイ格子エラー::同じ座標へ升目を二重に置いた { 座標 });
        }
        self.座標ごとの宣言.insert(座標, 宣言);
        Ok(())
    }

    pub(super) fn 宣言を引く(&self, 座標: 升目の座標) -> Option<升目の宣言> {
        self.座標ごとの宣言.get(&座標).copied()
    }

    pub(super) fn 升目があるか(&self, 座標: 升目の座標) -> bool {
        self.座標ごとの宣言.contains_key(&座標)
    }

    /// 名指した側面の向こう隣に升目があるか。その面は隣を継ぐ口として使われ、壁を入れる面ではなくなる。
    pub(super) fn 同じ階の隣に升目があるか(&self, 座標: 升目の座標, 側面: 升目の側面) -> bool {
        座標.同じ階の隣(側面).is_some_and(|隣| self.升目があるか(隣))
    }

    pub(super) fn 真上に升目があるか(&self, 座標: 升目の座標) -> bool {
        座標.真上の升目().is_some_and(|真上| self.升目があるか(真上))
    }

    pub(super) fn 真下に升目があるか(&self, 座標: 升目の座標) -> bool {
        座標.真下の升目().is_some_and(|真下| self.升目があるか(真下))
    }

    /// 昇順の座標の並び。検査と木を張る走査がこの順を使う。
    pub(super) fn 座標を昇順に数え上げる(&self) -> impl Iterator<Item = 升目の座標> + '_ {
        self.座標ごとの宣言.keys().copied()
    }

    pub(super) fn 件数(&self) -> usize {
        self.座標ごとの宣言.len()
    }

    pub(super) fn 空か(&self) -> bool {
        self.座標ごとの宣言.is_empty()
    }
}
