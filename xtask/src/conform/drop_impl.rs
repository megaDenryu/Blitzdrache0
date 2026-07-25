//! Drop実装の配置検査: blitz_renderのvulkan配下でDropトレイト実装を禁止する。
//! Vulkan資源の必要な破棄順序は資源の束をまたいで交錯する(提示一式が持つスワップチェーンとサーフェスの破棄の間に
//! vkDestroyDeviceが挟まる)ため、破棄順を宣言順という見えない場所へ移すDropに任せると
//! renderer/destroy.rsから順序の記述が消える。破棄順の誤りはvalidation layerが捕まえないため、
//! この配置だけを機械検査で守る。

use std::path::Path;

use super::violation::違反;

const 検査対象接頭辞: &str = "crates/blitz_render/src/vulkan/";

pub fn 検査対象パスか(パス: &Path) -> bool {
    パス.to_string_lossy().replace('\\', "/").starts_with(検査対象接頭辞)
}

/// stdの`Drop`トレイト実装の開始行かを判定する。ライフタイム・ジェネリクス引数付きの`impl<...> Drop for`も対象にする。
pub fn 行がドロップ実装か(行: &str) -> bool {
    let 整形 = 行.trim_start();
    整形.starts_with("impl") && 整形.contains("Drop for")
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    if !検査対象パスか(パス) {
        return Vec::new();
    }
    内容
        .lines()
        .enumerate()
        .filter(|(_, 行)| 行がドロップ実装か(行))
        .map(|(行番号, _)| {
            違反::行単位(
                パス.to_path_buf(),
                行番号 + 1,
                "vulkan配下のDrop実装(破棄順が資源の束をまたいで交錯するため、順序の記述はrenderer/destroy.rsだけに置く)".to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn vulkan配下だけを検査対象にする() {
        assert!(検査対象パスか(Path::new(r"crates\blitz_render\src\vulkan\swapchain.rs")));
        assert!(!検査対象パスか(Path::new("crates/blitz_render/src/renderer/mod.rs")));
        assert!(!検査対象パスか(Path::new("crates/blitz_engine/src/streaming/loader.rs")));
    }

    #[test]
    fn ライフタイム付きの実装も検出する() {
        assert!(行がドロップ実装か("impl Drop for スワップチェーン {"));
        assert!(行がドロップ実装か("impl<'a> Drop for 転送実行環境<'a> {"));
        assert!(!行がドロップ実装か("// impl Drop for は書かない"));
        assert!(!行がドロップ実装か("impl スワップチェーン {"));
    }

    #[test]
    fn 対象外ファイルのドロップ実装は違反にしない() {
        let 内容 = "impl Drop for レンダラー {\n}\n";
        assert!(検査する(Path::new("crates/blitz_render/src/renderer/mod.rs"), 内容).is_empty());
        assert_eq!(検査する(Path::new("crates/blitz_render/src/vulkan/swapchain.rs"), 内容).len(), 1);
    }
}
