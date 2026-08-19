//! プロジェクトルートの解決。起動引数`--project <ルート>`で受け、省略時はリポジトリルートを使う
//! (参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断2」)。
//! このディレクトリ配下のどこに何を置くか(`editor_data/`等)はこの型の関心事ではなく、
//! それを使う側(`storage::file_repository`の`編集データディレクトリ`)が自分の配置として持つ
//! (グローバルCLAUDE.md「プリミティブ執着禁止はパス・テキスト・名前にも適用する」「役割の型は
//! 自分の配置を知る」)。この型が公開する生の`&Path`は、その導出のための境界1箇所として渡す。

use std::path::{Path, PathBuf};

use crate::repository_root::リポジトリルート;

/// プロジェクトルートとは、いま開いている1つのゲームプロジェクトのルートディレクトリのことである。
#[derive(Debug, Clone)]
pub struct プロジェクトルート(PathBuf);

impl プロジェクトルート {
    pub fn 生成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// プロジェクト情報応答が使う、フォルダ名から導く表示名。取り出せない場合は
    /// ルートパスの文字列表現をそのまま使う(黙って空文字にはしない)。
    pub fn プロジェクト名(&self) -> String {
        match self.0.file_name() {
            Some(名前) => 名前.to_string_lossy().into_owned(),
            None => self.0.to_string_lossy().into_owned(),
        }
    }

    pub fn パス文字列(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }
}

/// `--project <ルート>`引数を探す。無ければ既定ルート(リポジトリルート)を使う。
pub fn プロジェクトルートを解決する(
    引数一覧: &[String], 既定ルート: &リポジトリルート
) -> プロジェクトルート {
    let 指定パス = 引数一覧
        .iter()
        .position(|引数| 引数 == "--project")
        .and_then(|添字| 引数一覧.get(添字 + 1));
    match 指定パス {
        Some(パス) => プロジェクトルート::生成する(PathBuf::from(パス)),
        None => プロジェクトルート::生成する(既定ルート.パス().to_path_buf()),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 指定が無ければ既定ルートを使う() {
        let 既定ルート = リポジトリルート::生成する(PathBuf::from("/repo"));
        let 解決結果 = プロジェクトルートを解決する(&[], &既定ルート);
        assert_eq!(解決結果.パス(), Path::new("/repo"));
    }

    #[test]
    fn projectフラグの次の引数を使う() {
        let 既定ルート = リポジトリルート::生成する(PathBuf::from("/repo"));
        let 引数一覧 = vec!["--project".to_string(), "/game".to_string()];
        let 解決結果 = プロジェクトルートを解決する(&引数一覧, &既定ルート);
        assert_eq!(解決結果.パス(), Path::new("/game"));
    }
}
