//! 参照パス実在検査: バッククォート囲みで`_doc/`または`shaders/`から始まる文字列を
//! `crates/`または`xtask/`から始まる参照も同じくリポジトリルート相対で実在するか確認する。

use std::path::Path;

use super::violation::違反;

pub fn バッククォート内パスを抽出する(行: &str) -> Vec<String> {
    let mut 結果 = Vec::new();
    let mut 残り = 行;
    while let Some(開始) = 残り.find('`') {
        let 開始後 = &残り[開始 + 1..];
        let Some(終了) = 開始後.find('`') else { break };
        let 中身 = &開始後[..終了];
        if ["_doc/", "shaders/", "crates/", "xtask/"].iter().any(|接頭辞| 中身.starts_with(接頭辞)) {
            結果.push(中身.to_string());
        }
        残り = &開始後[終了 + 1..];
    }
    結果
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        for 参照パス in バッククォート内パスを抽出する(行) {
            if !Path::new(&参照パス).exists() {
                違反一覧.push(違反::行単位(
                    パス.to_path_buf(),
                    行番号 + 1,
                    format!("参照パスが存在しない: {参照パス}"),
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
    fn doc配下のパスを抽出する() {
        let 行 = "//! 参照: `_doc/計画/ユビキタス言語.md`「数学DDDの語彙」。";
        assert_eq!(バッククォート内パスを抽出する(行), vec!["_doc/計画/ユビキタス言語.md".to_string()]);
    }

    #[test]
    fn ソース配下のパスも抽出する() {
        let 行 = "既定は`shaders/scene.slang`。実装は`crates/blitz_engine/src/lib.rs`と`xtask/src/main.rs`。";
        assert_eq!(
            バッククォート内パスを抽出する(行),
            vec![
                "shaders/scene.slang".to_string(),
                "crates/blitz_engine/src/lib.rs".to_string(),
                "xtask/src/main.rs".to_string()
            ]
        );
    }

    #[test]
    fn バッククォートが無ければ空() {
        assert!(バッククォート内パスを抽出する("普通の文").is_empty());
    }
}
