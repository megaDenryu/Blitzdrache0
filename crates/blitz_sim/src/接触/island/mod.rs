//! 接触島(判断17)。
//! 動的剛体の連結成分であり、反復・休止と再開・並列化の論理単位である。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断17: 接触島は動的剛体の連結成分であり、島の中の反復の順序は鍵の辞書式昇順である」

mod contact_island;
mod island_builder;
mod island_range;
mod island_reorder;
#[cfg(test)]
mod island_tests;
mod island_union_find;
mod previous_islands;

pub use contact_island::接触島;
pub use island_builder::接触島の一覧を構築する;
pub use island_range::島の拘束の添字区間;
pub use previous_islands::直前の細分の接触島の一覧;
