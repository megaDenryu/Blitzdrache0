//! 材質・テクスチャ・画像の安定IDの発番。触れるのは3つの連番と、画素の中身から画像IDを引く対応だけである。
//!
//! 画像IDを画素の中身から決めるのは、これがテクスチャ台帳の重複除去が効く唯一の入口だからである。別々の材質が
//! 同じ画像を持つとき、番号を素直に増やすだけでは同じ画素を世代の中へ何枚も常駐させてしまう。
//! 突き合わせは中身の完全一致で行う。指紋の一致だけで同じ画像とみなすと、衝突した2枚が無言で入れ替わる。
//! そのため`指紋別`は、同じ指紋に複数の中身が並ぶことを許す。

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::texture_material::テクスチャ素材;

use crate::vulkan::material_table::image_id::画像ID;
use crate::vulkan::material_table::material_id::大域材質ID;
use crate::vulkan::material_table::texture_id::テクスチャID;

pub(super) struct 安定IDの発番 {
    次の材質番号: u64,
    次のテクスチャ番号: u64,
    次の画像番号: u64,
    指紋別: HashMap<u64, Vec<(テクスチャ素材, 画像ID)>>, // 画素の指紋から、画像の中身と発番済み画像IDの対応
}

impl 安定IDの発番 {
    pub(super) fn 新規() -> Self {
        Self {
            次の材質番号: 0,
            次のテクスチャ番号: 0,
            次の画像番号: 0,
            指紋別: HashMap::new(),
        }
    }

    pub(super) fn 材質idを発番する(&mut self) -> 大域材質ID {
        let 番号 = self.次の材質番号;
        self.次の材質番号 = 番号.saturating_add(1);
        大域材質ID::生成する(番号)
    }

    pub(super) fn テクスチャidを発番する(&mut self) -> テクスチャID {
        let 番号 = self.次のテクスチャ番号;
        self.次のテクスチャ番号 = 番号.saturating_add(1);
        テクスチャID::生成する(番号)
    }

    /// 同じ画素と同じ用途の素材には同じ画像IDを返す。初めて見る中身にだけ新しい番号を発番する。
    pub(super) fn 画像idを引き当てる(&mut self, 素材: &テクスチャ素材) -> 画像ID {
        let 指紋 = 指紋を求める(素材);
        let 候補 = self.指紋別.entry(指紋).or_default();
        if let Some((_, 画像id)) = 候補.iter().find(|(既知, _)| 既知 == 素材) {
            return *画像id;
        }
        let 番号 = self.次の画像番号;
        self.次の画像番号 = 番号.saturating_add(1);
        let 画像id = 画像ID::生成する(番号);
        候補.push((素材.clone(), 画像id));
        画像id
    }
}

fn 指紋を求める(素材: &テクスチャ素材) -> u64 {
    let mut 計算器 = DefaultHasher::new();
    素材.幅().hash(&mut 計算器);
    素材.高さ().hash(&mut 計算器);
    素材.用途().hash(&mut 計算器);
    素材.格納形式().hash(&mut 計算器);
    素材.段ごとのバイト列().hash(&mut 計算器);
    計算器.finish()
}
