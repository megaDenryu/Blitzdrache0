//! 入力契約を検査するglTFのファイル: 検査の対象を指す役割つきのパス。担当するのは、そのファイルを開くことと、
//! 開けた文書へ全項目の走査を1回通して結果を返すことである。
//!
//! 裸のパスでなく役割の型で受けるのは、この道具が受け取れるのが.glbか.gltfの1ファイルだけであり、
//! 出力先や置き場のパスを渡しても裸の`&Path`では型が通るためである。
//! 文書を開く失敗を結果の1件目の違反として返すのは、開けないこと自体が読み手の直すべき破れだからである。

use std::path::{Path, PathBuf};

use super::super::document;
use super::inspection::開いた文書の契約検査;
use super::result::契約検査結果;

pub struct 入力契約を検査するglTFのファイル {
    パス: PathBuf,
}

impl 入力契約を検査するglTFのファイル {
    pub fn 生成する(パス: &Path) -> Self {
        Self {
            パス: パス.to_path_buf()
        }
    }

    pub fn 全項目を検査する(&self) -> 契約検査結果 {
        let 文書 = match document::文書を開く(&self.パス) {
            Ok(文書) => 文書,
            Err(誤り) => return 契約検査結果::開けなかった(誤り.to_string()),
        };
        let mut 検査 = 開いた文書の契約検査::文書から始める(&文書);
        検査.全項目を走査する();
        検査.結果へ畳む()
    }
}
