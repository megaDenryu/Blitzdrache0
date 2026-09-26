//! 地点と経路のつながりを書くGraphiteの図式(schema)の宣言と、生成物の取り込み。地点を節点、経路を辺とする。
//! 地点は区域を持ち、経路は種類と通り口と開閉の予定(経路の積み荷)を持つ。
//! 経路を向きの無い辺にするのは、橋や門や峠道がどちらの地点からも通れる箇所だからである。同じ2つの地点のあいだに経路を2本以上置くことを禁じないのは、
//! 橋と浅瀬のように別の箇所を別の経路として開閉したいためである。
//! 図式に `#[derive(Clone)]` を付けるのは、完成したグラフを `M不変データ` の地図(`地点と経路のグラフ`)として複製できるようにするためである。
//! 生成物はGraphiteの生成器(`cargo graphite generate`)が書き、手で編集しない。参照: `_doc/設計/Esca/設計正本.md` 5節。

use blitz_design::M不変データ;

use super::区域::区域;
use super::経路の積み荷::経路の積み荷;

/// 地点と経路のグラフの節点の値。地点が世界のどの範囲かを表す区域を持つ。地点の同一性は図式の識別子が担う。
#[derive(Debug, Clone, PartialEq)]
pub struct 地点 {
    区域: 区域,
}

impl M不変データ for 地点 {}

impl 地点 {
    pub fn 区域で生成する(区域: 区域) -> Self {
        Self { 区域 }
    }

    pub fn 区域(&self) -> &区域 {
        &self.区域
    }
}

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
        edge 経路 = 地点 -[積み荷: 経路の積み荷]- 地点;
    }
}
