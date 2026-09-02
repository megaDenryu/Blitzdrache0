//! 参加する2点の添字を持つ距離拘束。`距離拘束の引数`が持たない参加点の添字を足したものであり、拘束グラフの拘束一覧の1件である。

use super::error::拘束グラフエラー;
use super::point_index::点添字;
use crate::xpbd::距離拘束の引数;

/// 拘束グラフの1本の距離拘束。aとbは別の点でなければならない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 添字付き距離拘束 {
    pub a: 点添字,
    pub b: 点添字,
    pub 引数: 距離拘束の引数,
}

impl 添字付き距離拘束 {
    /// 同じ点を結ぶ拘束は型付きエラーで拒む。距離が常に0で向きが定まらず、反復の中で無言に無視され続けるためである。
    pub fn 生成する(a: 点添字, b: 点添字, 引数: 距離拘束の引数) -> Result<Self, 拘束グラフエラー> {
        if a == b {
            return Err(拘束グラフエラー::同じ点を結ぶ拘束 { 点: a });
        }
        Ok(Self { a, b, 引数 })
    }

    /// その点がこの拘束のどちら側かを答える。どちらでもなければ`None`である。
    pub fn 側(&self, 点: 点添字) -> Option<super::adjacency::隣接の側> {
        if 点 == self.a {
            Some(super::adjacency::隣接の側::A)
        } else if 点 == self.b {
            Some(super::adjacency::隣接の側::B)
        } else {
            None
        }
    }
}
