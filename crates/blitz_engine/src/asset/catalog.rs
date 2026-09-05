//! カタログ: 安定IDとファイルパスの対応表。パス直接参照を禁じる要石であり、
//! ロードAPIはカタログとIDのみを受ける。
//! 参照: `_doc/計画/ユビキタス言語.md`「カタログ」。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::id::アセットID;
use super::{asset_metadata::アセットメタデータ, catalog_entry::カタログ項目, world_provenance::世界の由来};

/// アセットID → ファイルパスの対応表。
#[derive(Debug)]
pub struct カタログ {
    登録一覧: HashMap<アセットID, カタログ項目>,
    世界の由来: 世界の由来,
}

impl カタログ {
    /// 空のカタログを作る。
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new(),
            世界の由来: 世界の由来::生成による由来を持たない,
        }
    }

    pub fn 世界の由来を記録する(&mut self, 世界の由来: 世界の由来) {
        self.世界の由来 = 世界の由来;
    }

    pub fn 世界の由来(&self) -> 世界の由来 {
        self.世界の由来
    }

    /// アセットIDにファイルパスを対応付ける。
    pub fn 登録する(&mut self, id: アセットID, パス: PathBuf) {
        self.詳細を登録する(id, パス, Vec::new(), アセットメタデータ::default());
    }

    pub fn 詳細を登録する(
        &mut self, id: アセットID, 実行時パス: PathBuf, ソース依存一覧: Vec<PathBuf>, メタデータ: アセットメタデータ
    ) {
        self.登録一覧.insert(id, カタログ項目::生成する(実行時パス, ソース依存一覧, メタデータ));
    }

    /// アセットIDからファイルパスを参照する。未登録なら`None`。
    pub fn パスを参照する(&self, id: &アセットID) -> Option<&Path> {
        self.登録一覧.get(id).map(カタログ項目::実行時パス)
    }

    pub fn 項目を参照する(&self, id: &アセットID) -> Option<&カタログ項目> {
        self.登録一覧.get(id)
    }

    /// ホットリロード監視用にID・パスの組を列挙する。
    pub fn 全登録を走査する(&self) -> impl Iterator<Item = (&アセットID, &Path)> {
        self.登録一覧.iter().map(|(id, 項目)| (id, 項目.実行時パス()))
    }

    pub fn 全項目を走査する(&self) -> impl Iterator<Item = (&アセットID, &カタログ項目)> {
        self.登録一覧.iter()
    }
}

impl Default for カタログ {
    fn default() -> Self {
        Self::空を作る()
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::super::id::アセットID;
    use super::カタログ;

    fn 試験用id(名前: &str) -> アセットID {
        match アセットID::生成する(名前) {
            Ok(id) => id,
            Err(誤り) => panic!("試験用IDの生成に失敗した: {誤り}"),
        }
    }

    #[test]
    fn 登録したパスを参照できる() {
        let id = 試験用id("quad");
        let パス = PathBuf::from("assets/smoke/quad.gltf");

        let mut カタログ = カタログ::空を作る();
        カタログ.登録する(id.clone(), パス.clone());

        assert_eq!(カタログ.パスを参照する(&id), Some(パス.as_path()));
    }

    #[test]
    fn 未登録のidはnoneを返す() {
        let id = 試験用id("未登録");
        let カタログ = カタログ::空を作る();

        assert_eq!(カタログ.パスを参照する(&id), None);
    }

    #[test]
    fn 全登録を走査できる() {
        let id = 試験用id("quad");
        let パス = PathBuf::from("assets/smoke/quad.gltf");

        let mut カタログ = カタログ::空を作る();
        カタログ.登録する(id.clone(), パス.clone());

        let 走査結果: Vec<(&アセットID, &Path)> = カタログ.全登録を走査する().collect();
        assert_eq!(走査結果, vec![(&id, パス.as_path())]);
    }
}
