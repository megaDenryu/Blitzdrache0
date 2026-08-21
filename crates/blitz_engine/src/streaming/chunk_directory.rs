//! チャンク目録: チャンク座標からアセットIDを参照する対応表であり、同時に世界に存在するチャンクの集合を表す。
//! アセットIDからパスを参照する`カタログ`とは別概念であり、目録が座標を解決した後にカタログがパスを解決する2段の関係にある。
//! 参照: `_doc/設計/チャンクストリーミング.md`「チャンク目録」

use std::collections::HashMap;

use super::{chunk_edge_length::チャンク一辺, directory_error::チャンク目録エラー};
use crate::{アセットID, チャンク座標};

#[derive(Debug)]
pub struct チャンク目録 {
    チャンク一辺: チャンク一辺,
    登録一覧: HashMap<チャンク座標, アセットID>,
}

impl チャンク目録 {
    pub fn 空を作る(チャンク一辺: チャンク一辺) -> Self {
        Self {
            チャンク一辺,
            登録一覧: HashMap::new(),
        }
    }

    pub fn 一辺(&self) -> チャンク一辺 {
        self.チャンク一辺
    }

    /// 1つの座標へ2つのアセットを割り当てると、どちらを読むかが登録順に依存するため座標重複を拒否する。
    pub fn 登録する(&mut self, チャンク: チャンク座標, アセット: アセットID) -> Result<(), チャンク目録エラー> {
        if self.登録一覧.contains_key(&チャンク) {
            return Err(チャンク目録エラー::座標重複(チャンク));
        }
        self.登録一覧.insert(チャンク, アセット);
        Ok(())
    }

    /// 目録に無い座標は世界の端であって異常ではないため`None`を返す。必要集合は世界の外側の座標も含む。
    pub fn アセットを参照する(&self, チャンク: チャンク座標) -> Option<&アセットID> {
        self.登録一覧.get(&チャンク)
    }

    pub fn 存在するか(&self, チャンク: チャンク座標) -> bool {
        self.登録一覧.contains_key(&チャンク)
    }

    pub fn 件数(&self) -> usize {
        self.登録一覧.len()
    }

    /// 永続化と検証のために全登録を列挙する。HashMapの走査順は不定であり、順序が要る側が整列する。
    pub fn 全登録を走査する(&self) -> impl Iterator<Item = (チャンク座標, &アセットID)> {
        self.登録一覧.iter().map(|(チャンク, アセット)| (*チャンク, アセット))
    }
}
