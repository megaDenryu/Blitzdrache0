//! マテリアル台帳の型契約。`GET|PUT /api/マテリアル台帳`が読み書きするJSONの正本であり、
//! マテリアル定義の一覧と、地表材質4層それぞれの参照先(層割当)を1本の直列化単位へ束ねる
//! (世界単位で1つ。参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断9」)。

use serde::{Deserialize, Serialize};

use super::material_definition::マテリアル定義;
use super::material_layer_assignment::層割当;
use super::validation_error::資源検証エラー;

/// マテリアル台帳とは、エディターが持つマテリアル定義の一覧と、地表材質4層それぞれの
/// エンジン材質名への割当を束ねた、`マテリアル台帳`のJSON1本ぶんの内容のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct マテリアル台帳 {
    pub マテリアル一覧: Vec<マテリアル定義>,
    pub 層割当: 層割当,
}

impl マテリアル台帳 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        let 登録済み材質名一覧 = self.登録済み材質名一覧を確かめる()?;
        for 参照先 in self.層割当.参照する材質名一覧() {
            if !登録済み材質名一覧.contains(&参照先) {
                return Err(資源検証エラー::参照先が存在しない {
                    フィールド名: "マテリアル台帳.層割当",
                    値: 参照先.to_string(),
                });
            }
        }
        Ok(())
    }

    /// 一覧の各件を検証しつつ、材質名の重複が無いことを確かめて材質名一覧を返す。
    fn 登録済み材質名一覧を確かめる(&self) -> Result<Vec<&str>, 資源検証エラー> {
        let mut 材質名一覧: Vec<&str> = Vec::new();
        for マテリアル in &self.マテリアル一覧 {
            マテリアル.検証する()?;
            if 材質名一覧.contains(&マテリアル.エンジン材質名.as_str()) {
                return Err(資源検証エラー::名前が重複している {
                    フィールド名: "マテリアル台帳.マテリアル一覧.エンジン材質名",
                    値: マテリアル.エンジン材質名.clone(),
                });
            }
            材質名一覧.push(&マテリアル.エンジン材質名);
        }
        Ok(材質名一覧)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 定義(材質名: &str) -> マテリアル定義 {
        マテリアル定義 {
            エンジン材質名: 材質名.to_string(),
            識別色: "#2d5a27".to_string(),
        }
    }

    fn 台帳() -> マテリアル台帳 {
        マテリアル台帳 {
            マテリアル一覧: vec![定義("grass_a"), 定義("dirt_a"), 定義("rock_a"), 定義("sand_a")],
            層割当: 層割当 {
                草: "grass_a".to_string(),
                泥: "dirt_a".to_string(),
                岩: "rock_a".to_string(),
                砂: "sand_a".to_string(),
            },
        }
    }

    #[test]
    fn 参照が揃っていれば検証を通る() {
        台帳().検証する().unwrap();
    }

    #[test]
    fn 材質名が重複していると拒否する() {
        let mut 対象 = 台帳();
        対象.マテリアル一覧.push(定義("grass_a"));
        assert!(対象.検証する().is_err());
    }

    #[test]
    fn 層割当が一覧に無い材質名を参照していると拒否する() {
        let mut 対象 = 台帳();
        対象.層割当.草 = "存在しない材質".to_string();
        assert!(対象.検証する().is_err());
    }

    #[test]
    fn マテリアル定義自体が不正なら拒否する() {
        let mut 対象 = 台帳();
        対象.マテリアル一覧[0].識別色 = "不正".to_string();
        assert!(対象.検証する().is_err());
    }
}
