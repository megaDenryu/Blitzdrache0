//! Cargo.tomlの最小パース。xtaskは外部依存ゼロを保つため、tomlクレートを使わず
//! [package]のnameと[dependencies]節のキー一覧だけを行単位で読み取る。
//!
//! ファイル名の綴りの正本もここが持つ。読む側が各自で綴ると、綴りが2つ以上のファイルへ散る。

/// パッケージの宣言が書かれているファイルの名前。クレートごとの宣言も、ワークスペースの宣言も同じ名前である。
pub const パッケージ宣言のファイル名: &str = "Cargo.toml";

pub fn クレート名を取り出す(内容: &str) -> Option<String> {
    let mut パッケージ節か = false;
    for 行 in 内容.lines() {
        let 行 = 行.trim();
        if 行.starts_with('[') {
            パッケージ節か = 行 == "[package]";
            continue;
        }
        if !パッケージ節か || !行.starts_with("name") {
            continue;
        }
        let 等号以降 = 行.trim_start_matches("name").trim_start();
        if let Some(値) = 等号以降.strip_prefix('=') {
            return Some(値.trim().trim_matches('"').to_string());
        }
    }
    None
}

/// 対象を絞った依存節の見出しの前後の綴り。`[target.'cfg(windows)'.dependencies]`のように、
/// 間に対象の条件が挟まる。この形を読み落とすと、対象を絞って書いた依存が白リストの検査を素通りする。
const 対象を絞った依存節の見出しの始まり: &str = "[target.";
const 対象を絞った依存節の見出しの終わり: &str = ".dependencies]";

/// 依存節かどうかを見出しの行から判定する。対象を絞らない`[dependencies]`と、対象を絞った依存節の両方を数える。
/// 開発用の依存(`[dev-dependencies]`)と組み立て用の依存(`[build-dependencies]`)は成果物へ入らないため数えない。
fn 依存節の見出しか(行: &str) -> bool {
    行 == "[dependencies]" || (行.starts_with(対象を絞った依存節の見出しの始まり) && 行.ends_with(対象を絞った依存節の見出しの終わり))
}

pub fn 依存名一覧を取り出す(内容: &str) -> Vec<String> {
    let mut 依存節か = false;
    let mut 依存名一覧 = Vec::new();
    for 行 in 内容.lines() {
        let 行 = 行.trim();
        if 行.starts_with('[') {
            依存節か = 依存節の見出しか(行);
            continue;
        }
        if !依存節か || 行.is_empty() || 行.starts_with('#') {
            continue;
        }
        if let Some(等号位置) = 行.find('=') {
            let 名前 = 行[..等号位置].trim();
            if !名前.is_empty() {
                依存名一覧.push(名前.to_string());
            }
        }
    }
    依存名一覧
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn パッケージ名を読み取る() {
        let 内容 = "[package]\nname = \"blitz_math\"\nedition = \"2024\"\n";
        assert_eq!(クレート名を取り出す(内容), Some("blitz_math".to_string()));
    }

    #[test]
    fn 依存節のキーだけ読み取る() {
        let 内容 = "[package]\nname = \"x\"\n[dependencies]\nglam = { workspace = true }\nthiserror = \"2\"\n";
        assert_eq!(依存名一覧を取り出す(内容), vec!["glam".to_string(), "thiserror".to_string()]);
    }

    #[test]
    fn 対象を絞った依存節のキーも読み取る() {
        let 内容 = "[dependencies]\nctrlc = \"3\"\n[target.'cfg(windows)'.dependencies]\nwin32job = { workspace = true }\n";
        assert_eq!(依存名一覧を取り出す(内容), vec!["ctrlc".to_string(), "win32job".to_string()]);
    }

    #[test]
    fn 開発用の依存節は数えない() {
        let 内容 = "[dependencies]\nctrlc = \"3\"\n[dev-dependencies]\nrusty_fork = \"0.3\"\n";
        assert_eq!(依存名一覧を取り出す(内容), vec!["ctrlc".to_string()]);
    }

    #[test]
    fn 依存が空でも空一覧() {
        assert!(依存名一覧を取り出す("[package]\nname = \"xtask\"\n[dependencies]\n").is_empty());
    }
}
