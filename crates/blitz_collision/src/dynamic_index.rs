//! 動く形の空間索引: 動く物体の外接する箱を保持し、覆いに触れうる形と、互いに重なりうる形の対を挙げる。
//!
//! **適する用途。** 毎刻み少しずつ動く、中規模の個数の物体である。剛体・キャラクター・破壊片がこれに当たる。
//! どれも1つの形が1つの箱で足り、1刻みの移動が自分の大きさに比べて小さい。索引はその「小さく動く」性質を
//! ゆとり付きの箱で使い切り、物体が動いても構造を触らずに済ませる。
//!
//! **適さない用途。** 2つある。1つは静的で巨大な三角形網であり、崖・洞窟がこれに当たる。三角形1枚ごとに箱を
//! 登録すると節点が三角形の枚数だけ育ち、しかも動かないのだから木を組み替える仕組みが丸ごと無駄になる。
//! そちらは事前に組んだ境界の木が担当する。もう1つは粒子系であり、同じ大きさの粒子が数万個ある場合である。
//! 木は節点を1つずつ辿るため、粒子1個あたりの費用が箱の比較の連なりになる。そちらは空間を升目に切って近所だけを
//! 調べる空間ハッシュが担当する(判断12が挙げた3つ目の方式であり、まだ実装していない)。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断12: 空間探索は粗い候補選別と詳細判定を責務として分け、世界は二段構えで辿る」
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断11: 静的衝突メッシュは階層を持ち、事前生成を基本とする」
//!
//! **方式は動く形の境界箱の木である。** 葉が形1つの箱を持ち、内部節点が2つの子を包む箱を持つ二分木である。
//! 登録は葉を1枚挿し、除去は葉を1枚外す。どちらも触るのは根から葉までの1本の道だけであり、木の全体を組み直さない。
//!
//! **ゆとり付きの箱。** 木へ載せるのは実体の箱そのものではなく、それを全方向へ幅のぶんだけ広げた箱である。
//! 実体がその中に居る間、箱の更新は木を1節点も触らない。ゆとりが無いと、形が1ミリ動くたびに葉を外して挿し直す
//! ことになり、動く物体のための索引という選択そのものが無意味になる。
//! 参照: `crates/blitz_collision/src/dynamic_index/margin.rs`
//!
//! **挿す先の選び方。** 根から下り、葉を加えたときに包む箱の表面積がいちばん育たない枝を選ぶ。等しいときは
//! 並びの先の枝を選ぶため、選び方は入力だけで決まる。
//! 参照: `crates/blitz_collision/src/dynamic_index/tree/insertion.rs`
//!
//! **組み替えの仕方。** 葉を挿した後と外した後に、触った節点から根まで辿り、左右の部分木の高さの差が1を越えた
//! 節点で高いほうの子を1段持ち上げる。高さの差を1以内に保つ木は、葉の数の対数に比例した深さに収まる。
//! 参照: `crates/blitz_collision/src/dynamic_index/tree/balance.rs`
//!
//! **決定性。** 挿す先の選び方も、組み替えの向きも、節点の枠の使い回しも、同じ操作列からは同じ結果になる。
//! そのうえで列挙の並びは識別子の昇順に固定するため、答えの並びは木の形にすら依らない。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」
//!
//! **ここに無いもの。** 実体どうしが本当に重なるかは答えない。答えるのは箱の重なりまでであり、そこから先は
//! 形の種類ごとの詳細判定が受け持つ。世界のどこに属する形かも持たない。索引が持つのは利用者が与えた識別子だけである。

mod box_edge_check;
mod error;
mod identifier;
mod index;
mod index_query;
mod leaf_scan;
mod margin;
mod node;
mod node_arena;
mod pair;
mod pair_query;
mod traversal_stack;
mod tree;
mod update_result;

#[cfg(test)]
mod boundary_tests;
#[cfg(test)]
mod brute_force_fixture;
#[cfg(test)]
mod brute_force_tests;
#[cfg(test)]
mod containment_tests;
#[cfg(test)]
mod degenerate_tests;
#[cfg(test)]
mod determinism_tests;
#[cfg(test)]
mod extreme_coordinate_tests;
#[cfg(test)]
mod invariant_fixture;
#[cfg(test)]
mod margin_tests;
#[cfg(test)]
mod pseudo_random_fixture;
#[cfg(test)]
mod round_trip_tests;
#[cfg(test)]
mod sample_fixture;
#[cfg(test)]
mod shape_dump_fixture;

pub use error::{
    動く形の空間索引の問い合わせエラー, 動く形の空間索引の操作エラー, 箱に持たせるゆとりの幅の生成エラー
};
pub use identifier::動く形の識別子;
pub use index::動く形の空間索引;
pub use margin::箱に持たせるゆとりの幅;
pub use pair::重なりうる形の対;
pub use update_result::箱の更新で木を組み替えたか;
