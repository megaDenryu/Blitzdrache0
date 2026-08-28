//! 世界の形への問い合わせ: 地形と建物の両方を相手にして、線分が最初に当たる形と、掃引したカプセルが
//! 最初に触れる形を答える。
//!
//! 高さ場の読み口と別のモジュールにするのは、この問い合わせが高さ場に加えて読込済みチャンクの静的物理形状を
//! 相手にするためである。読み口を広げると、高さ場を持つだけの型がストリーミングの状態を知ることになる。
//! 逆に調停へ足すと、ストリーミングの調停が地形の高さ場を知ることになる。どちらの向きにも依存を増やさずに
//! 済ませるため、2つを受け取って組む操作サービスをここに置く。
//!
//! 完全性は2つの答えの合成である。地形の側が高さ場の広がりの外で打ち切ったか、建物の側が読み込まれていない
//! チャンクを覆いに含んだなら、合成した答えはいったん「評価できない領域を含む」になる。そのとき当たりが見つかって
//! いれば、問い合わせの範囲をその当たりまでへ切り詰めた覆いで建物の側の完全性を問い直す。切り詰めた範囲が完全なら
//! その当たりより早い当たりは在りえないため確定として返し、切り詰めても完全でなければ当たりを捨てて
//! 「何にも当たらない」の側で答える。捨てる側の規律は、掃引の走査が覆いの中に答えられない升目を1つでも
//! 見つけたときに接触を捨てるのと同じである。切り詰めの決まりは `truncated_confirmation.rs` が持つ。
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
mod truncated_completeness_tests;
mod truncated_confirmation;
#[cfg(test)]
mod world_query_fixture;

pub use building_contact::カプセルと建物の接触;
pub use building_hit::線分と建物の当たり;
pub use building_place::建物の子形状の所在;
pub use capsule_hit::掃引したカプセルが最初に触れる世界の形;
pub use error::世界の形への問い合わせエラー;
pub use query::世界の形への問い合わせ;
pub use segment_hit::線分が最初に当たる世界の形;
