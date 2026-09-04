//! 動的剛体の連結成分の抽出(判断17)。
//! 接触拘束で結ばれた動的剛体をUnion-Findでまとめ、最小の剛体識別子の昇順で並べた島の剛体一覧を返す。
//! ハッシュ表を使わずBTreeMapで決定論的な全順序を保つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断17: 接触島は動的剛体の連結成分であり、島の中の反復の順序は鍵の辞書式昇順である」

use std::collections::BTreeMap;

use crate::rigid_body::剛体の識別子;

pub(super) struct 動的剛体の素集合 {
    親: BTreeMap<剛体の識別子, 剛体の識別子>,
}

impl 動的剛体の素集合 {
    pub(super) fn 動的剛体一覧から生成する(動的剛体一覧: &[剛体の識別子]) -> Self {
        let mut 親 = BTreeMap::new();
        for &識別子 in 動的剛体一覧 {
            親.insert(識別子, 識別子);
        }
        Self { 親 }
    }

    pub(super) fn 代表元を検索する(&mut self, 識別子: 剛体の識別子) -> Option<剛体の識別子> {
        if !self.親.contains_key(&識別子) {
            return None;
        }
        let mut 現在 = 識別子;
        while let Some(&親識別子) = self.親.get(&現在) {
            if 親識別子 == 現在 {
                break;
            }
            現在 = 親識別子;
        }
        let 代表元 = 現在;
        let mut 経路 = 識別子;
        while let Some(&親識別子) = self.親.get(&経路) {
            if 親識別子 == 代表元 {
                break;
            }
            self.親.insert(経路, 代表元);
            経路 = 親識別子;
        }
        Some(代表元)
    }

    pub(super) fn 併合する(&mut self, 一方: 剛体の識別子, 他方: 剛体の識別子) {
        let (Some(代表一方), Some(代表他方)) = (self.代表元を検索する(一方), self.代表元を検索する(他方)) else {
            return;
        };
        if 代表一方 != 代表他方 {
            let 小さい方 = 代表一方.min(代表他方);
            let 大きい方 = 代表一方.max(代表他方);
            self.親.insert(大きい方, 小さい方);
        }
    }

    /// 最小の剛体識別子の昇順で並べた、島ごとの動的剛体一覧(各島の中も識別子昇順)を返す。
    pub(super) fn 島ごとの動的剛体一覧を構築する(&mut self) -> Vec<Vec<剛体の識別子>> {
        let mut 島マップ: BTreeMap<剛体の識別子, Vec<剛体の識別子>> = BTreeMap::new();
        let 剛体一覧: Vec<剛体の識別子> = self.親.keys().copied().collect();
        for 識別子 in 剛体一覧 {
            if let Some(代表元) = self.代表元を検索する(識別子) {
                島マップ.entry(代表元).or_default().push(識別子);
            }
        }
        let mut 結果 = Vec::with_capacity(島マップ.len());
        for (_, mut 剛体たち) in 島マップ {
            剛体たち.sort_unstable();
            結果.push(剛体たち);
        }
        結果.sort_unstable_by_key(|剛体たち| 剛体たち.first().copied());
        結果
    }
}
