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

pub fn 依存名一覧を取り出す(内容: &str) -> Vec<String> {
    let mut 依存節か = false;
    let mut 依存名一覧 = Vec::new();
    for 行 in 内容.lines() {
        let 行 = 行.trim();
        if 行.starts_with('[') {
            依存節か = 行 == "[dependencies]";
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
    fn 依存が空でも空一覧() {
        assert!(依存名一覧を取り出す("[package]\nname = \"xtask\"\n[dependencies]\n").is_empty());
    }
}
