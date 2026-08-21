//! 木に張った升目: 木の1つの節点。どの升目かと、親からどう繋がっているかを持つ。
//!
//! 親を`Option`で持つのは、根だけが親を持たないためである。根を「自分自身を親に持つ升目」で表すと、
//! 指示を組む工程が根を据える場合と継ぐ場合を座標の比較で見分けることになり、比較を書き忘れると
//! 根の骨格を自分自身へ繋ぐ指示が出る。
//!
//! 継ぎ方の語彙(`tree_link`)から分けるのは、変わる理由が違うためである。継ぎ方は骨格が宣言する口が
//! 増えると変わり、この型は木の張り方が変わると変わる。

use super::cell_coordinate::升目の座標;
use super::tree_link::親からの継ぎ方;

/// 木の中で子が親へ繋がる1本の辺。親の座標と継ぎ方の組である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 親への繋がり {
    親の座標: 升目の座標,
    継ぎ方: 親からの継ぎ方,
}

impl 親への繋がり {
    pub(super) fn 親の座標(self) -> 升目の座標 {
        self.親の座標
    }

    pub(super) fn 継ぎ方(self) -> 親からの継ぎ方 {
        self.継ぎ方
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 木に張った升目 {
    座標: 升目の座標,
    親への繋がり: Option<親への繋がり>,
}

impl 木に張った升目 {
    pub(super) fn 根として張る(座標: 升目の座標) -> Self {
        Self {
            座標, 親への繋がり: None
        }
    }

    pub(super) fn 親から継いで張る(座標: 升目の座標, 親の座標: 升目の座標, 継ぎ方: 親からの継ぎ方) -> Self {
        Self {
            座標,
            親への繋がり: Some(親への繋がり { 親の座標, 継ぎ方 }),
        }
    }

    pub(super) fn 座標(&self) -> 升目の座標 {
        self.座標
    }

    pub(super) fn 親への繋がり(&self) -> Option<親への繋がり> {
        self.親への繋がり
    }
}
