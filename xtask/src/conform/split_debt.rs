//! 切り出しの根拠が未是正のまま残っているファイルの台帳。
//! 型のフィールドと責務を減らす再設計が済むまで是正できないファイルだけをここに列挙し、
//! 検査はこの一覧に限って経緯語を見逃す。文言だけを整えて検査を通す是正を防ぐための台帳である。
//! 全件が是正済みのため現在の一覧は空だが、仕組みは残す。空の台帳では例外が1件も無く、どのファイルの経緯語も違反になる。
//! 参照: CLAUDE.md「切り出しの根拠義務(パーシャル規約)」。

use std::path::Path;

use super::violation::違反;

/// 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。
/// 削除できるのは、そのファイルの分割根拠が例外1〜4のいずれかを満たすよう型を再設計したときだけである。
const 未是正ファイル一覧: [&str; 0] = [];

/// 走査で得るパスの区切り文字は実行環境で変わるため、斜線へ揃えてから台帳と照合する。
fn 台帳の表記へ揃える(パス: &Path) -> String {
    パス.to_string_lossy().replace('\\', "/")
}

pub fn 既知の未是正ファイルか(パス: &Path) -> bool {
    未是正ファイル一覧.contains(&台帳の表記へ揃える(パス).as_str())
}

/// 台帳に載っているのに経緯語が無いファイルを違反として報告し、台帳からの削除を強制する。
/// これがないと是正済みの項目が残り続け、次に同じ違反を書いたときに見逃す穴になる。
pub fn 台帳の陳腐化を検査する(パス: &Path, 未是正既知: bool, 経緯語を見つけた: bool) -> Vec<違反> {
    if 未是正既知 && !経緯語を見つけた {
        return vec![違反::ファイル単位(
            パス.to_path_buf(),
            "切り出し未是正の台帳に載っているが経緯語が無い(台帳から削除する)".to_string(),
        )];
    }
    Vec::new()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 台帳が空なのでどのファイルも既知の未是正ではない() {
        assert!(!既知の未是正ファイルか(Path::new("crates/blitz_render/src/renderer/draw.rs")));
        assert!(!既知の未是正ファイルか(Path::new(r"crates\blitz_render\src\renderer\draw.rs")));
    }

    #[test]
    fn 区切り文字が逆斜線でも斜線表記へ揃う() {
        assert_eq!(
            台帳の表記へ揃える(Path::new(r"crates\blitz_render\src\renderer\draw.rs")),
            "crates/blitz_render/src/renderer/draw.rs"
        );
    }

    #[test]
    fn 是正済みなのに台帳に残っていたら違反にする() {
        assert_eq!(台帳の陳腐化を検査する(Path::new("a.rs"), true, false).len(), 1);
        assert!(台帳の陳腐化を検査する(Path::new("a.rs"), true, true).is_empty());
        assert!(台帳の陳腐化を検査する(Path::new("a.rs"), false, false).is_empty());
    }
}
