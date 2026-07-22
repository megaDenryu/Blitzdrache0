//! slangc実行ファイルの発見。VULKAN_SDK配下を優先し、無ければPATH経由を試す。

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// slangcの起動に使うプログラム指定。
pub(super) enum スランガー位置 {
    /// VULKAN_SDK配下で発見した絶対パス。
    絶対パス(PathBuf),
    /// PATH解決に委ねる（"slangc"をそのままプログラム名として渡す）。
    パス経由,
}

impl スランガー位置 {
    pub(super) fn プログラム名(&self) -> &Path {
        match self {
            Self::絶対パス(パス) => パス.as_path(),
            Self::パス経由 => Path::new("slangc"),
        }
    }
}

pub(super) fn 発見する() -> Result<スランガー位置, String> {
    if let Ok(sdk) = env::var("VULKAN_SDK") {
        let 候補 = PathBuf::from(sdk).join("Bin").join("slangc.exe");
        if 候補.is_file() {
            return Ok(スランガー位置::絶対パス(候補));
        }
    }

    if slangcがpathで動くか() {
        return Ok(スランガー位置::パス経由);
    }

    Err("slangcが見つからない。VULKAN_SDK環境変数を確認するか、PATHにslangcを追加すること".to_string())
}

fn slangcがpathで動くか() -> bool {
    Command::new("slangc")
        .arg("-v")
        .output()
        .map(|出力| 出力.status.success())
        .unwrap_or(false)
}
