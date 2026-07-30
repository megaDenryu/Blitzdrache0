//! カタログ: 安定IDとファイルパスの対応表。パス直接参照を禁じる要石であり、
//! ロードAPIはカタログとIDのみを受ける。
//! 参照: `_doc/計画/ユビキタス言語.md`「カタログ」。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::id::アセットID;
use super::{asset_metadata::アセットメタデータ, catalog_entry::カタログ項目};

/// アセットID → ファイルパスの対応表。
#[derive(Debug, Default)]
pub struct カタログ {
    登録一覧: HashMap<アセットID, カタログ項目>,
}

impl カタログ {
    /// 空のカタログを作る。
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new()
        }
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
