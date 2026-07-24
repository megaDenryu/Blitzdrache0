//! 節参照実在検査: 生存型文書の中で同一文書内の節を名指しする参照が、実在する見出しを
//! 指しているか確認する。節を削除・改名したときに参照だけが残る劣化を機械的に止める。
//! 対象は `_doc/` 配下の .md と README.md から、ログ型の `_doc/開発スレッド/` を除いたもの。
//! 開発スレッドは過去の発言を逐語収録する追記専用の記録であり、他文書の節名を引用するため
//! 同一文書内の参照として判定できない。

use std::path::{Component, Path, PathBuf};

use super::section_reference::節名を抽出する;
use super::violation::違反;
use crate::file_scan;

pub fn 全文書を検査する() -> Result<Vec<違反>, String> {
    let mut 対象一覧 = file_scan::対象ファイル一覧を集める(&["_doc"], &["md"])?;
    対象一覧.push(PathBuf::from("README.md"));
    let mut 違反一覧 = Vec::new();
    for パス in 対象一覧.iter().filter(|パス| 対象文書か(パス)) {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        違反一覧.extend(検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn 対象文書か(パス: &Path) -> bool {
    !パス
        .components()
        .any(|部品| matches!(部品, Component::Normal(名前) if 名前 == "開発スレッド"))
}

/// 見出し行から `#` と前後の空白を除いた文字列を返す。括弧付きの見出しを取りこぼさないため、
/// 参照側との一致判定は完全一致ではなく包含で行う。
pub fn 見出し名を取り出す(行: &str) -> Option<&str> {
    let 井桁数 = 行.chars().take_while(|文字| *文字 == '#').count();
    if 井桁数 == 0 || 井桁数 > 6 {
        return None;
    }
    let 残り = &行[井桁数..];
    if 残り.starts_with(' ') { Some(残り.trim()) } else { None }
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let 見出し一覧: Vec<&str> = 内容.lines().filter_map(見出し名を取り出す).collect();
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        if 見出し名を取り出す(行).is_some() {
            continue;
        }
        for 節名 in 節名を抽出する(行) {
            if !見出し一覧.iter().any(|見出し| 見出し.contains(&節名)) {
                違反一覧.push(違反::行単位(
                    パス.to_path_buf(),
                    行番号 + 1,
                    format!("同一文書内に実在しない節への参照: {節名}"),
                ));
            }
        }
    }
    違反一覧
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 見出し行から名前を取り出す() {
        assert_eq!(見出し名を取り出す("## 方針移管の現在状態"), Some("方針移管の現在状態"));
        assert_eq!(見出し名を取り出す("#見出しでない"), None);
        assert_eq!(見出し名を取り出す("本文"), None);
    }

    #[test]
    fn 実在する見出しへの参照は違反にならない() {
        let 内容 = "## 方針移管の現在状態(足場)\n所在は下の「方針移管の現在状態」で管理する。\n";
        assert!(検査する(Path::new("_doc/設計/x.md"), 内容).is_empty());
    }

    #[test]
    fn 実在しない見出しへの参照を違反にする() {
        let 内容 = "## 方針移管の現在状態\n所在は下の「既知の逸脱」で管理する。\n";
        assert_eq!(検査する(Path::new("_doc/設計/x.md"), 内容).len(), 1);
    }

    #[test]
    fn 開発スレッドは対象外() {
        assert!(!対象文書か(Path::new("_doc/開発スレッド/x.md")));
        assert!(対象文書か(Path::new("_doc/設計/x.md")));
    }
}
