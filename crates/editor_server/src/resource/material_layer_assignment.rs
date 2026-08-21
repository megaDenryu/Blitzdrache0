//! 層割当の型契約。地表材質ペイントの4層(草・泥・岩・砂)それぞれがどのエンジン材質名を
//! 参照するかを持つ。参照先がマテリアル一覧に実在するかの検証は`マテリアル台帳`が持つ
//! (層割当単体は参照先の一覧を知らないため)。

use serde::{Deserialize, Serialize};

/// 層割当とは、地表材質の4層それぞれが参照するエンジン材質名を表す値のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 層割当 {
    pub 草: String,
    pub 泥: String,
    pub 岩: String,
    pub 砂: String,
}

impl 層割当 {
    /// 4層が参照するエンジン材質名を一覧で返す。`マテリアル台帳`が参照先の実在を検証するために使う。
    pub fn 参照する材質名一覧(&self) -> [&str; 4] {
        [self.草.as_str(), self.泥.as_str(), self.岩.as_str(), self.砂.as_str()]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 参照する材質名一覧は4層の順で返る() {
        let 対象 = 層割当 {
            草: "grass_a".to_string(),
            泥: "dirt_a".to_string(),
            岩: "rock_a".to_string(),
            砂: "sand_a".to_string(),
        };
        assert_eq!(対象.参照する材質名一覧(), ["grass_a", "dirt_a", "rock_a", "sand_a"]);
    }
}
