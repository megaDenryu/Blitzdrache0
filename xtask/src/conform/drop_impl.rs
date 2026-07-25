//! Drop実装の配置検査: blitz_renderのvulkan配下とrenderer配下でDropトレイト実装を禁止し、
//! `crates/blitz_render/src/renderer/mod.rs`だけを例外にする。
//! Vulkan資源の必要な破棄順序は資源の束をまたいで交錯する(提示一式が持つスワップチェーンとサーフェスの破棄の間に
//! vkDestroyDeviceが挟まる)ため、破棄順を宣言順という見えない場所へ移すDropに任せると
//! renderer/destroy.rsから順序の記述が消える。破棄順の誤りはvalidation layerが捕まえないため、
//! この配置だけを機械検査で守る。
//!
//! 注意: ash::Entryを最後に落とすという不変条件はフィールド宣言順では保証されていない。実際の保証は、
//! 単一のRAIIオーナー(mod.rsのimpl Drop for レンダラー)が全フィールドが生きた状態で全Vulkan破棄を済ませること、
//! ash 0.38のInstance・Device・各loaderにDrop実装が無いこと、他の束にVulkanを呼ぶDrop実装が無いことの3つである。
//! この検査が守るのは3つ目であり、対象範囲を狭めるとその保証に穴が空く。

use std::path::Path;

use super::violation::違反;

const 検査対象接頭辞一覧: [&str; 2] = ["crates/blitz_render/src/vulkan/", "crates/blitz_render/src/renderer/"];

/// 唯一のRAIIオーナーであり、全Vulkan破棄を1箇所で行う設計の本体。ここにあるDrop実装だけが正解である。
const 例外ファイル: &str = "crates/blitz_render/src/renderer/mod.rs";

pub fn 検査対象パスか(パス: &Path) -> bool {
    let 表記 = パス.to_string_lossy().replace('\\', "/");
    表記 != 例外ファイル && 検査対象接頭辞一覧.iter().any(|接頭辞| 表記.starts_with(接頭辞))
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
                "blitz_renderのDrop実装(破棄順が資源の束をまたいで交錯するため、順序の記述はrenderer/mod.rsの単一オーナーとrenderer/destroy.rsだけに置く)".to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn vulkan配下とrenderer配下を検査対象にする() {
        assert!(検査対象パスか(Path::new(r"crates\blitz_render\src\vulkan\swapchain.rs")));
        assert!(検査対象パスか(
            Path::new("crates/blitz_render/src/renderer/scene_draw_resources.rs")
        ));
        assert!(検査対象パスか(Path::new(r"crates\blitz_render\src\renderer\frame_progress.rs")));
        assert!(!検査対象パスか(Path::new("crates/blitz_engine/src/streaming/loader.rs")));
    }

    #[test]
    fn 単一オーナーのmodだけを例外にする() {
        assert!(!検査対象パスか(Path::new("crates/blitz_render/src/renderer/mod.rs")));
        assert!(!検査対象パスか(Path::new(r"crates\blitz_render\src\renderer\mod.rs")));
        assert!(検査対象パスか(Path::new("crates/blitz_render/src/renderer/generate/mod.rs")));
    }

    #[test]
    fn ライフタイム付きの実装も検出する() {
        assert!(行がドロップ実装か("impl Drop for スワップチェーン {"));
        assert!(行がドロップ実装か("impl<'a> Drop for 転送実行環境<'a> {"));
        assert!(!行がドロップ実装か("// impl Drop for は書かない"));
        assert!(!行がドロップ実装か("impl スワップチェーン {"));
    }

    #[test]
    fn 例外ファイルのドロップ実装は違反にしない() {
        let 内容 = "impl Drop for レンダラー {\n}\n";
        assert!(検査する(Path::new("crates/blitz_render/src/renderer/mod.rs"), 内容).is_empty());
        assert_eq!(検査する(Path::new("crates/blitz_render/src/renderer/frame_progress.rs"), 内容).len(), 1);
        assert_eq!(検査する(Path::new("crates/blitz_render/src/vulkan/swapchain.rs"), 内容).len(), 1);
    }
}
