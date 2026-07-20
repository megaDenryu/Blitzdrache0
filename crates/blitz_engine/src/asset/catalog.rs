//! カタログ: 安定IDとファイルパスの対応表。パス直接参照を禁じる要石であり、
//! ロードAPIはカタログとIDのみを受ける。
//! 参照: `_doc/計画/ユビキタス言語.md`「カタログ」。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::id::アセットID;

/// アセットID → ファイルパスの対応表。
#[derive(Debug, Default)]
pub struct カタログ {
    登録一覧: HashMap<アセットID, PathBuf>,
}

impl カタログ {
    /// 空のカタログを作る。
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new(),
        }
    }

    /// アセットIDにファイルパスを対応付ける。
    pub fn 登録する(&mut self, id: アセットID, パス: PathBuf) {
        self.登録一覧.insert(id, パス);
    }

    /// アセットIDからファイルパスを引く。未登録なら`None`。
    pub fn パスを引く(&self, id: &アセットID) -> Option<&Path> {
        self.登録一覧.get(id).map(PathBuf::as_path)
    }

    /// ホットリロード監視用にID・パスの組を列挙する。
    pub fn 全登録を走査する(&self) -> impl Iterator<Item = (&アセットID, &Path)> {
        self.登録一覧.iter().map(|(id, パス)| (id, パス.as_path()))
    }
}
