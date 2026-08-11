//! 実行時のslangc発見。build_support側と同じ方針だが、ビルドスクリプトと実行バイナリは
//! 別のコンパイル単位のためコードは共有できず、ここに同等のロジックを持つ。

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) enum スランガー位置 {
    絶対パス(PathBuf),
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

use super::compile_error::シェーダー再コンパイルエラー;

pub(super) fn 発見する() -> Result<スランガー位置, シェーダー再コンパイルエラー> {
    if let Ok(sdk) = env::var("VULKAN_SDK") {
        let 候補 = PathBuf::from(sdk).join("Bin").join("slangc.exe");
        if 候補.is_file() {
            return Ok(スランガー位置::絶対パス(候補));
        }
    }

    let path経由で動くか = Command::new("slangc")
        .arg("-v")
        .output()
        .map(|出力| 出力.status.success())
        .unwrap_or(false);
    if path経由で動くか {
        return Ok(スランガー位置::パス経由);
    }

    Err(シェーダー再コンパイルエラー::スランガーが見つからない)
}
