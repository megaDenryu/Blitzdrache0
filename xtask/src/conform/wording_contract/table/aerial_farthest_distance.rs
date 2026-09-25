//! 焼きへ渡った空中遠近条件の最遠距離の行の見出し。
//! 見出しが片側だけ動くと、検収は行を見つけられず「最遠距離を確かめた」という判定そのものが走らなくなる。

use super::文言の契約;

const 出す側: &str = "crates/blitz_app/src/reports/aerial_farthest_distance.rs";
const 読む側: &str = "xtask/src/sky_lut/farthest_distance.rs";

pub(super) const 文言一覧: [文言の契約; 1] = [文言の契約 {
    文言: "空中遠近の最遠距離",
    現れるファイル一覧: &[出す側, 読む側],
}];
