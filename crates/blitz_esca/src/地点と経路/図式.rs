//! 地点と経路のつながりを書くGraphiteの図式(schema)の宣言と、生成物の取り込み。地点を節点、経路を辺とし、経路は開閉の予定を積み荷に持つ。
//! 経路を向きの無い辺にするのは、橋や門や峠道がどちらの地点からも通れる箇所だからである。同じ2つの地点のあいだに経路を2本以上置くことを禁じないのは、
//! 橋と浅瀬のように別の箇所を別の経路として開閉したいためである。
//! 図式に `#[derive(Clone)]` を付けるのは、完成したグラフを `M不変データ` の地図(`地点と経路のグラフ`)として複製できるようにするためである。
//! 生成物はGraphiteの生成器(`cargo graphite generate`)が書き、手で編集しない。参照: `_doc/設計/Esca/設計正本.md` 5節。

use blitz_design::M不変データ;

use super::開閉の予定::経路の開閉の予定;

/// 地点と経路のグラフの節点の値。地点がどの区域に対応するかは、区域を足す作業(第2段階の区域と歩行の接続)で持たせる。いまは地点の同一性だけを図式の識別子が担う。
#[derive(Debug, Clone, PartialEq)]
pub struct 地点;

impl M不変データ for 地点 {}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
#[rustfmt::skip]
pub mod 地点と経路の図式 {
    include!("generated/地点と経路の図式.rs");
}

#[rustfmt::skip]
graphite::dynamic_graph_schema! {
    generated = "generated/地点と経路の図式.rs";
    #[derive(Clone)]
    schema 地点と経路の図式 {
        node 地点;
        edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点;
    }
}
