//! 世界の形への問い合わせ: 地形と建物の両方を相手にして、線分が最初に当たる形と、掃引したカプセルが
//! 最初に触れる形を答える。
//!
//! 高さ場の読み口と別のモジュールにするのは、この問い合わせが高さ場に加えて読込済みチャンクの静的物理形状を
//! 相手にするためである。読み口を広げると、高さ場を持つだけの型がストリーミングの状態を知ることになる。
//! 逆に調停へ足すと、ストリーミングの調停が地形の高さ場を知ることになる。どちらの向きにも依存を増やさずに
//! 済ませるため、2つを受け取って組む操作サービスをここに置く。
//!
//! 完全性は2つの答えの合成である。地形の側が高さ場の広がりの外で打ち切ったか、建物の側が読み込まれていない
//! チャンクを覆いに含んだなら、合成した答えも「評価できない領域を含む」になる。そのとき見つけた当たりは捨てて
//! 「何にも当たらない」の側で答える。打ち切りより手前に別の当たりがありうるからであり、これは掃引の走査が
//! 覆いの中に答えられない升目を1つでも見つけたときに接触を捨てるのと同じ規律である。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断8: 問い合わせの値と完全性を別軸にする」
//!
//! ゲーム側への配線はここに無い。地面か壁か、そこで止まるか滑るかという意味付けはゲームの側が与える。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断3: 問い合わせの結果を数学層・世界側・ゲーム側の3層に分ける」

mod building_capsule;
mod building_contact;
mod building_hit;
mod building_place;
mod building_segment;
mod capsule_hit;
mod capsule_query;
#[cfg(test)]
mod capsule_world_tests;
mod composition;
mod error;
mod local_frame;
mod query;
mod segment_hit;
mod segment_query;
#[cfg(test)]
mod segment_world_tests;
#[cfg(test)]
mod world_query_fixture;

pub use building_contact::カプセルと建物の接触;
pub use building_hit::線分と建物の当たり;
pub use building_place::建物の子形状の所在;
pub use capsule_hit::掃引したカプセルが最初に触れる世界の形;
pub use error::世界の形への問い合わせエラー;
pub use query::世界の形への問い合わせ;
pub use segment_hit::線分が最初に当たる世界の形;
