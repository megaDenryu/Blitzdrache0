//! パッケージの宣言(Cargo.toml)の本文が、graphite へ依存しているかの判定。受け取るのは本文、返すのは真偽である。
//!
//! 見分ける書き方は次のとおりである。依存の節(`[dependencies]`・`[dev-dependencies]`・`[build-dependencies]`・対象を絞った `[target.<条件>.dependencies]`)の
//! `graphite = ...` と `graphite.workspace = true` のような点で続けた鍵、`別名 = { package = "graphite", ... }` の別名、
//! 依存1件ごとの表の節 `[dependencies.graphite]` と、その表の中の `package = "graphite"` である。
//! `graphite-cli` のような名前の続く別の依存と、注釈の行は拾わない。行を読むだけで拾えない書き方が残っても、
//! 照合はGraphiteの生成物を持つパッケージが対象に全部入っていることを別に突き合わせるため、取りこぼしは違反として現れる。

const 依存の名前: &str = "graphite";

/// 本文のどこかで graphite へ依存しているか。
pub(super) fn graphiteに依存するか(本文: &str) -> bool {
    let mut 節 = 節の種類::依存の節でない;
    for 行 in 本文.lines().map(str::trim) {
        if 行.is_empty() || 行.starts_with('#') {
            continue;
        }
        if 行.starts_with('[') {
            節 = 節の種類::見出しから読む(行);
            if 節 == 節の種類::graphiteの依存の表 {
                return true;
            }
            continue;
        }
        let Some((鍵, 値)) = 行.split_once('=') else {
            continue;
        };
        let 鍵 = 鍵.trim().trim_matches('"');
        let 依存する = match 節 {
            節の種類::依存の節でない | 節の種類::graphiteの依存の表 => false,
            節の種類::依存の節 => 鍵.split('.').next() == Some(依存の名前) || パッケージ名としてgraphiteを名乗るか(値),
            節の種類::別名の依存の表 => 鍵 == "package" && 値.trim().trim_matches('"') == 依存の名前,
        };
        if 依存する {
            return true;
        }
    }
    false
}

/// いま読んでいる行が、どの節の中に在るか。
#[derive(Debug, PartialEq, Eq)]
enum 節の種類 {
    依存の節でない,
    依存の節,
    graphiteの依存の表,
    別名の依存の表,
}

impl 節の種類 {
    fn 見出しから読む(見出し: &str) -> Self {
        let 中身 = 見出し.trim_start_matches('[').trim_end_matches(']').trim();
        let 節の名前一覧: Vec<&str> = 中身.split('.').map(|名前| 名前.trim().trim_matches('"')).collect();
        let Some(依存の節の位置) = 節の名前一覧.iter().position(|名前| 名前.ends_with("dependencies")) else {
            return Self::依存の節でない;
        };
        match 節の名前一覧.get(依存の節の位置 + 1) {
            None => Self::依存の節,
            Some(&名前) if 名前 == 依存の名前 => Self::graphiteの依存の表,
            Some(_) => Self::別名の依存の表,
        }
    }
}

// インライン表の中の `package = "graphite"` を、空白の入れ方に左右されずに見つける。
fn パッケージ名としてgraphiteを名乗るか(値: &str) -> bool {
    let 空白を除いた値: String = 値.chars().filter(|文字| !文字.is_whitespace()).collect();
    空白を除いた値.contains(&format!("package=\"{依存の名前}\""))
}

#[cfg(test)]
mod tests {
    use super::graphiteに依存するか;

    #[test]
    fn graphiteへの依存の書き方を全部見分ける() {
        for 本文 in [
            "[dependencies]\ngraphite = { workspace = true }\n",
            "[dependencies]\n  graphite= { git = \"https://github.com/megaDenryu/Graphite\" }\n",
            "[dependencies]\ngraphite.workspace = true\n",
            "[dev-dependencies]\ngraphite = \"0.1\"\n",
            "[target.'cfg(windows)'.dependencies]\ngraphite = { workspace = true }\n",
            "[dependencies]\n型付きのグラフ = { package = \"graphite\", workspace = true }\n",
            "[dependencies.graphite]\nworkspace = true\n",
            "[dependencies.型付きのグラフ]\npackage = \"graphite\"\nworkspace = true\n",
        ] {
            assert!(graphiteに依存するか(本文), "graphiteへの依存を見分けなかった: {本文}");
        }
    }

    #[test]
    fn graphiteでない依存と注釈と依存の節の外は拾わない() {
        for 本文 in [
            "[dependencies]\ngraphite-cli = \"1\"\n",
            "[dependencies]\n# graphite = 注釈\n",
            "[dependencies]\nthiserror = { workspace = true }\n",
            "[package]\nname = \"graphite\"\n",
            "[dependencies.型付きのグラフ]\npackage = \"graphite-cli\"\n",
        ] {
            assert!(!graphiteに依存するか(本文), "graphiteへの依存でない本文を依存と見分けた: {本文}");
        }
    }
}
